pub mod databae {
    mod auth;
    mod store;
}

pub fn init(path: String) {
    // Initialize database using path
    assert!(!path.is_empty());
}

#[test]
fn databae_init() {
    init("Dummy path".to_string());
}
