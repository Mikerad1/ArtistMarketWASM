pub(crate) mod user {

    use minicbor::{Decode, Encode};
    use wasmcloud_interface_sqldb::{minicbor, SqlDb, SqlDbError, SqlDbSender};

    use wasmbus_rpc::{common::Context, actor::prelude::WasmHost};
    pub use crate::models::dtos::userdto;

    type Db = SqlDbSender<WasmHost>;

    #[derive(Encode, Decode)]
    pub struct UserDB {
        #[n(0)]
        pub id: String,
        #[n(1)]
        pub username: String,
        #[n(2)]
        pub password: String,
        #[n(3)]
        pub email: String,
        #[n(4)]
        pub created_at: String,
        #[n(5)]
        pub updated_at: String,
        #[n(6)]
        pub id_number: String,
        #[n(7)]
        pub first_name: String,
        #[n(8)]
        pub last_name: String,
        #[n(9)]
        pub phone_number: String,
        #[n(10)]
        pub address: String,
        #[n(11)]
        pub gender: String,
        #[n(12)]
        pub role_id: String,
    }

    impl UserDB {
        pub fn new(
            username: String,
            password: String,
            email: String,
            id_number: String,
            first_name: String,
            last_name: String,
            phone_number: String,
            address: String,
            gender: String,
            role_id: String
        ) -> Self {
            UserDB {
                id: "".to_string(),
                username,
                password,
                email,
                created_at: time::OffsetDateTime::now_utc().to_string(),
                updated_at: time::OffsetDateTime::now_utc().to_string(),
                id_number,
                first_name,
                last_name,
                phone_number,
                address,
                gender,
                role_id
            }
        }
        pub fn new_full(
            id: String,
            username: String,
            password: String,
            email: String,
            created_at: String,
            updated_at: String,
            id_number: String,
            first_name: String,
            last_name: String,
            phone_number: String,
            address: String,
            gender: String,
            role_id: String
        ) -> Self {
            UserDB {
                id,
                username,
                password,
                email,
                created_at,
                updated_at,
                id_number,
                first_name,
                last_name,
                phone_number,
                address,
                gender,
                role_id
            }
        }
        
    }
    impl From<userdto::userdto::UserDTO> for UserDB {
        fn from(t: userdto::userdto::UserDTO) -> UserDB {
            UserDB::new_full(
                t.id,
                t.username,
                t.password,
                t.email,
                t.created_at,
                t.updated_at,
                t.id_number,
                t.first_name,
                t.last_name,
                t.phone_number,
                t.address,
                t.gender,
                t.role_id
            )
        }
    }
    
    pub async fn init_table(ctx: &Context, client: &Db) -> Result<(), SqlDbError> {
        let sql = format!(
            r#"create table if not exists {} (
                id varchar(36) not null,
                username varchar(max) not null,
                password varchar(max) not null,
                email varchar(max) not null,
                created_at varchar(max) not null,
                updated_at varchar(max) not null,
                id_number varchar(13) not null,
                first_name varchar(max) not null,
                last_name varchar(max) not null,
                phone_number varchar(10) not null,
                address varchar(max),
                gender varchar(10),
                role_id varchar(36) not null,
            );"#,
            "Users"
        );
        let _resp = client.execute(ctx, &sql.into()).await?;
        Ok(())
    }
}
