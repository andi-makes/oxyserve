use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Result};


#[get("/static/{filename:.*}")]
pub async fn route(req: HttpRequest) -> Result<NamedFile> {
    // Get the data directory
    let data_dir = &std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());
    let file_path: PathBuf = [data_dir, "static", req.match_info().query("filename")]
        .iter()
        .collect();

    Ok(NamedFile::open(file_path)?)
}