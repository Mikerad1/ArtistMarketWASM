use std::{str::FromStr, time::SystemTime};

use jwt_provider::*;
use models::db_models::{user::{user::UserDB, self}, role::{role::RoleDB, self}};
use thehaven_interfaces::*;
use time::{ Date, OffsetDateTime, format_description};
use wasmbus_rpc::{actor::prelude::*, minicbor::decode, Timestamp};
use wasmcloud_interface_numbergen::{generate_guid, random_in_range};
use wasmcloud_interface_sqldb::{SqlDb, SqlDbError, SqlDbSender};
mod models;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Auth)]
struct ThehavenAuthActor {}

#[async_trait]
impl Auth for ThehavenAuthActor {

    async fn init_tables(&self, ctx: &Context) -> RpcResult<bool> {
        let db = SqlDbSender::new();
        user::user::init_table(ctx, &db).await?;
        role::role::init_table(ctx, &db).await?;
        Ok(true)
    }

    async fn get_roles(&self, ctx: &Context) -> RpcResult<Vec<thehaven_interfaces::Role>> {
        let db = SqlDbSender::new();
        let resp = db
            .query(ctx, &format!(r#"select * from {};"#, "Roles").into())
            .await?;

        let rows: Vec<RoleDB> = decode(&resp.rows)?;
        let mut roles: Vec<thehaven_interfaces::Role> = Vec::new();
        for row in rows {
            let role = thehaven_interfaces::Role {
                id: row.id,
                name: row.name
            };
            roles.push(role);
        }

        Ok(roles)
    }

    async fn get_user_role<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        msg: &TS,
    ) -> RpcResult<thehaven_interfaces::Role> {
        let db = SqlDbSender::new();
        let resp = db
            .query(
                ctx,
                &format!(
                    r#"
        select * from {} where id = '{}';
        "#,
                    "Users",
                    msg.to_string()
                )
                .into(),
            )
            .await?;

        if resp.num_rows == 0 {
            return Err(SqlDbError::new("notFound", "User was not found".to_string()).into());
        }
        let mut rows: Vec<UserDB> = decode(&resp.rows)?;
        let user = rows.remove(0);
        let role_resp = db
            .query(
                ctx,
                &format!(
                    r#"
        select * from {} where id = '{}';
        "#,
                    "Roles",
                    user.role_id
                )
                .into(),
            )
            .await?;
        if role_resp.num_rows == 0 {
            return Err(SqlDbError::new("notFound", "Role was not found".to_string()).into());
        }

        let mut rows: Vec<RoleDB> = decode(&role_resp.rows)?;
        let role = rows.remove(0);

        let haven_role = thehaven_interfaces::Role {
            id: role.id,
            name: role.name
        };

        Ok(haven_role)
    }

    async fn register(
        &self,
        ctx: &Context,
        msg: &thehaven_interfaces::User,
    ) -> RpcResult<thehaven_interfaces::User> {
        let db = SqlDbSender::new();
        let id = generate_guid().await.unwrap();
        let salt = random_in_range(1, 6).await.unwrap().to_string();
        let hashed_password = argon2::hash_encoded(
            msg.password.as_bytes(),
            salt.as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap();
        let format = time::format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
                sign:mandatory]:[offset_minute]:[offset_second]"
        ).unwrap();
        db.execute(ctx, &format!(
            r#"
            insert into {} (id, username, password, email, created_at, updated_at, id_number, first_name, last_name, phone_number, address, gender, role_id)
            values ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');
            "#, "Users", id, msg.username, hashed_password, msg.email, OffsetDateTime::from_unix_timestamp(msg.created_at.as_nanos().try_into().unwrap()).unwrap(), OffsetDateTime::from_unix_timestamp(msg.updated_at.as_nanos().try_into().unwrap()).unwrap(), msg.id_number, msg.first_name, msg.last_name, msg.phone_number, msg.address, msg.gender, msg.role_id
        ).into())
        .await?;

        let user = thehaven_interfaces::User {
            id: id,
            username: msg.username.clone(),
            password: hashed_password.clone(),
            email: msg.email.clone(),
            created_at: msg.created_at.clone(),
            updated_at: msg.updated_at.clone(),
            id_number: msg.id_number.clone(),
            first_name: msg.first_name.clone(),
            last_name: msg.last_name.clone(),
            phone_number: msg.phone_number.clone(),
            address: msg.address.clone(),
            gender: msg.gender.clone(),
            role_id: msg.role_id.clone(),
        };

        Ok(user)
    }

    async fn login(&self, ctx: &Context, msg: &LoginRequest) -> RpcResult<String> {
        let db = SqlDbSender::new();
        let resp = db
            .query(
                ctx,
                &format!(
                    r#"
        select * from {} where username = '{}';
        "#,
                    "Users", msg.username
                )
                .into(),
            )
            .await?;
        if resp.num_rows == 0 {
            return Err(SqlDbError::new("notFound", "User was not found".to_string()).into());
        }

        let mut rows: Vec<UserDB> = decode(&resp.rows)?;
        let user = rows.remove(0);
        let password_result = argon2::verify_encoded(&user.password, msg.password.as_bytes())
            .map_err(|_| SqlDbError::new("invalidPassword", "Invalid password".to_string()));

        if password_result.unwrap() {
            let format = format_description::parse(
                "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
                     sign:mandatory]:[offset_minute]:[offset_second]"
            ).unwrap();
            
            let jwt_provider = JwtHandlerSender::new();
            let jwt_user = jwt_provider::User {
                id: user.id.clone(),
                email: user.email.clone(),
                created_at: Timestamp::from(SystemTime::from(
                    OffsetDateTime::parse(user.created_at.as_str(), &format).unwrap(),
                )),
                updated_at: Timestamp::from(SystemTime::from(
                    OffsetDateTime::parse(user.updated_at.as_str(), &format).unwrap(),
                )),
                first_name: Some(user.first_name.clone()),
                last_name: Some(user.last_name.clone()),
                username: Some(user.username.clone()),
            };
            let jwt_response = jwt_provider.generate_jwt(ctx, &jwt_user).await?;
            return Ok(jwt_response);
        } else {
            return Err(SqlDbError::new("invalidPassword", "Invalid password".to_string()).into());
        }
    }
}
