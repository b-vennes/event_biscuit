use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TeapotCreated {
    pub id: Uuid,
    pub name: String,
    pub size: i32
}

#[derive(Serialize, Deserialize)]
pub struct TeaBrewed {
    pub id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct TeaPoured {
    pub id: Uuid
}

#[derive(Serialize, Deserialize)]
pub enum TeapotEvent {
    Created(TeapotCreated),
    TeaBrewed(TeaBrewed),
    TeaPoured(TeaPoured),
}