use uuid::Uuid;
pub struct TeapotState {
    pub id: Uuid,
    pub name: String,
    pub size: i32,
    pub remaining: i32
}
