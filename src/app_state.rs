use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db: Database, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }
}
