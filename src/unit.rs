pub mod unit {
    #[derive(Clone)]
    pub struct Unit {
        pub id: String,
        pub name: String,
        pub unit_class: String,
        pub unit_func: String,
    }

    impl Unit {
        pub fn new(id: String, name: String, unit_class: String, unit_func: String) -> Self {
            Self {
                id,
                name,
                unit_class,
                unit_func,
            }
        }
        pub fn id(&self) -> &str {
            &self.id
        }
    }

    // Display the unit using the format "{title} by {author}".
    // This is a typical Rust trait and is not axum-specific.
    impl std::fmt::Display for Unit {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{} {} {} {}", self.id, self.name, self.unit_func, self.unit_class)
        }
    }

}
