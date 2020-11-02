use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateTeapot {
    pub name: String,
    pub size: i32
}

#[derive(Serialize, Deserialize)]
pub struct BrewTea {
    pub id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct PourTea {
    pub id: Uuid
}

#[derive(Serialize, Deserialize)]
pub enum TeapotCommand {
    CreateTeapot(CreateTeapot),
    BrewTea(BrewTea),
    PourTea(PourTea),
}
