// use std::path::PathBuf;
// 
// use actix_files::NamedFile;
// use actix_web::{get, HttpRequest, Result, Responder};
// 
// 
// #[get("/static/{filename:.*}")]
// pub async fn route(req: HttpRequest) -> impl Responder {
//     // Get the data directory
//     let data_dir = &std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());
//     let file_path: PathBuf = [data_dir, "static", req.match_info().query("filename")]
//         .iter()
//         .collect();
// 
//    NamedFile::open(file_path).unwrap().into_response(req)
// }
