use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateTeapotData {
    pub name: String,
    pub size: i32
}

#[derive(Serialize, Deserialize)]
pub enum TeapotCommand {
    CreateTeapot(CreateTeapotData),
    BrewTea(Uuid),
    PourTea(Uuid),
}
