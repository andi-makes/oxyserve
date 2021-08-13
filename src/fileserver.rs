/// This is a custom fileserver that sets the Cache-control" header.
/// Inspired by https://github.com/SergioBenitez/Rocket/issues/95#issuecomment-354824883
/// I had to adopt the solution a little bit to make it work with the rocket 0.5 release candidate
use std::path::PathBuf;

use rocket::{fs::NamedFile, response::Responder, Request, Response};

pub struct CachedFile(NamedFile);

/// Implement a custom Responder if a cached file is requested
/// The cache time is set to 1 year
impl<'r> Responder<'r, 'static> for CachedFile {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Cache-control", "max-age=31536000") // 1y
            .ok()
    }
}

/// The static route. All files requested via /static/ have a cache time as defined in
/// The responder. (default: 1 year)
/// It tried to load the file from `$DATA_DIR/static/<path>`
#[get("/static/<file..>", rank = 10)]
pub async fn files(file: PathBuf) -> Option<CachedFile> {
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());

    NamedFile::open(std::path::Path::new(&format!("{}/static", &data_dir)).join(file))
        .await
        .ok()
        .map(CachedFile)
}
