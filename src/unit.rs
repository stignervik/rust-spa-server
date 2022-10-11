pub mod unit {
    // Use Deserialize to convert e.g. from request JSON into Book struct.
    use serde::Deserialize;

    // Demo book structure with some example fields for id, title, author.
    // A production app could prefer an id to be type u32, UUID, etc.
    #[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
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
    }

    // Display the unit using the format "{title} by {author}".
    // This is a typical Rust trait and is not axum-specific.
    impl std::fmt::Display for Unit {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{} {} {} {}",
                self.id, self.name, self.unit_func, self.unit_class
            )
        }
    }
}
