use sqlx::types::time::PrimitiveDateTime;

pub struct UserDatabase {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
