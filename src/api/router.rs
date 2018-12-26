use rocket::http::Status;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct StartControllingRouterCredentials {
    pub address: String,
    pub port: Option<u8>,
    pub username: String,
    pub password: String,
    pub description: Option<String>
}

#[derive(Serialize)]
pub struct RouterDetails {
    pub id: u64,
    pub name: String,
    pub address: String,
    pub port: u8,
    pub maintenance_username: String,
    pub maintenance_password: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_description: Option<String>,
    pub backup_date: String,
    pub software_version: String
}

#[post("/router", data = "<creds>")]
pub fn start_controlling(creds: Json<StartControllingRouterCredentials>) -> Result<Json<RouterDetails>, Status> {
    Err(Status::raw(501))
}

#[get("/router")]
pub fn list_routers() -> Result<Json<Vec<RouterDetails>>, Status> {
    Err(Status::raw(501))
}

#[derive(Serialize)]
pub struct AdminCredentials {
    pub username: String,
    pub password: String
}

#[delete("/router?<id>")]
pub fn stop_controlling(id: u8) -> Result<Json<AdminCredentials>, Status> {
    Err(Status::raw(501))
}

#[derive(Deserialize)]
pub struct PatchData {
    pub description: Option<String>,
    pub address: Option<String>
}

#[patch("/router/<id>", data = "<info>")]
pub fn update_router(id: u64, info: Json<PatchData>) -> Result<(), Status> {
    Err(Status::raw(501))
}
