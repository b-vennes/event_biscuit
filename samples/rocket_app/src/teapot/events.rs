use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TeapotCreatedData {
    pub name: String,
    pub size: i32
}

#[derive(Serialize, Deserialize)]
pub enum TeapotEvent {
    Created(Uuid, TeapotCreatedData),
    TeaBrewed(Uuid),
    TeaPoured(Uuid),
}