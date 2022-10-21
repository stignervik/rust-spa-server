use serde::{Deserialize, Serialize};

// Demo book structure with some example fields for id, title, author.
// A production app could prefer an id to be type u32, UUID, etc.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub unit_class: String,
    pub unit_func: String,
}

impl Unit {
    #[allow(dead_code)]
    pub fn new(id: String, name: String, unit_class: String, unit_func: String) -> Self {
        Self {
            id,
            name,
            unit_class,
            unit_func,
        }
    }
    #[allow(dead_code)]
    pub fn id(&self) -> String {
        self.id.clone()
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
