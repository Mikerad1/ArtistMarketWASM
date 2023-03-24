pub (crate) mod role {
    
    use minicbor::{Decode, Encode};
    use wasmbus_rpc::{common::Context, actor::prelude::WasmHost};
    use wasmcloud_interface_sqldb::{minicbor, SqlDb, SqlDbError, SqlDbSender};
    use crate::models::dtos::roledto;

    type Db = SqlDbSender<WasmHost>;

    #[derive(Encode, Decode)]
    pub struct RoleDB {
        #[n(0)]
        pub id: String,
        #[n(1)]
        pub name: String
    }

    impl RoleDB {
        pub fn new(name: String) -> Self {
            RoleDB {
                id: "".to_string(),
                name
            }
        }
        pub fn new_full(id: String, name: String) -> Self {
            RoleDB {
                id,
                name
            }
        }
    }
    
    pub async fn init_table(ctx: &Context, client: &Db) -> Result<(), SqlDbError> {
        let sql = format!(
            r#"create table if not exists {} (
                id varchar(36) not null,
                name varchar(max) not null
            );"#,
            "Roles"
        );
        let _resp = client.execute(ctx, &sql.into()).await?;
        Ok(())
    }
    
    impl From<roledto::roledto::RoleDTO> for RoleDB {
        fn from(t: roledto::roledto::RoleDTO) -> RoleDB {
            RoleDB::new_full(
                t.id,
                t.name
            )
        }
    }
}