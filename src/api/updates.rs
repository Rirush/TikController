use rocket::http::Status;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct SoftwareStatus {
    pub current: String,
    pub update_available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new: Option<String>,
}

#[get("/router/<id>/update", rank = 0)]
pub fn check_for_updates(id: u64) -> Result<Json<SoftwareStatus>, Status> {
    Err(Status::raw(501))
}

#[post("/router/<id>/update", rank = 0)]
pub fn update(id: u64) -> Result<(), Status> {
    Err(Status::raw(501))
}
