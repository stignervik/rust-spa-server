// pub mod unit {
    // use axum::Json;
    // Use Deserialize to convert e.g. from request JSON into Book struct.
    use serde::{Deserialize, Serialize};
    // use serde_json::json;

    // Demo book structure with some example fields for id, title, author.
    // A production app could prefer an id to be type u32, UUID, etc.
    #[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
    pub struct Unit {
        pub id: u32,
        pub name: String,
        pub unit_class: String,
        pub unit_func: String,
    }

    impl Unit {
        #[allow(dead_code)]
        pub fn new(id: u32, name: String, unit_class: String, unit_func: String) -> Self {
            Self {
                id,
                name,
                unit_class,
                unit_func,
            }
        }
        #[allow(dead_code)]
        pub fn id(&self) -> u32 {
            self.id
        }

        /*
        pub fn toJson(&self) -> Json<String> {
            let json = json!({
                "id": self.id,
            });
            json
        }
        */
    }

    // Display the unit using the format.
    // This is a typical Rust trait and is not axum-specific.
    impl std::fmt::Display for Unit {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "id: {} name: {} unit_func: {} unit_class: {}",
                self.id, self.name, self.unit_func, self.unit_class
            )
        }
    }
// }
