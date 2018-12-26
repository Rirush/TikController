use rocket::http::Status;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct LogEntry {
    pub date: String,
    pub severity: u8,
    pub message: String
}

#[get("/router/<id>/logs?<entries>")]
pub fn get_logs(id: u64, entries: Option<u64>) -> Result<Json<Vec<LogEntry>>, Status> {
    Err(Status::raw(501))
}
