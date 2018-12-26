use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/router/backup/<id>")]
pub fn force_backup(id: u64) -> Result<(), Status> {
    Err(Status::raw(501))
}

#[derive(Serialize)]
pub struct BackupDetails {
    pub id: u64,
    pub date: String,
}

#[get("/router/backup/<id>", rank = 1)]
pub fn list_backups(id: u64) -> Result<Json<Vec<BackupDetails>>, Status> {
    Err(Status::raw(501))
}

#[post("/router/restore/<r_id>/<b_id>", rank = 1)]
pub fn restore_from_backup(r_id: u64, b_id: u64) -> Result<(), Status> {
    Err(Status::raw(501))
}
