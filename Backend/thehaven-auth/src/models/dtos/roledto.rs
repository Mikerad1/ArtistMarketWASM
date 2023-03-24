pub(crate) mod roledto {
    use serde::{Serialize, Deserialize};
    use crate::models::dbModels::role::role::RoleDB;

    #[derive(Serialize, Deserialize)]
    pub struct RoleDTO{
        pub id: String,
        pub name: String
    }
    impl RoleDTO {
        pub fn new(name: String) -> Self {
            RoleDTO {
                id: "".to_string(),
                name
            }
        }
        pub fn new_full(id: String, name: String) -> Self {
            RoleDTO {
                id,
                name
            }
        }
    }
    impl From<RoleDB> for RoleDTO {
        fn from(t: RoleDB) -> RoleDTO {
            RoleDTO::new_full(
                t.id,
                t.name
            )
        }
    }
}