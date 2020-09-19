use actix_web::{web, App, HttpServer, HttpRequest, Error, error};
use actix_web::middleware::Logger;
use actix_files::NamedFile;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;
#[macro_use]extern crate lazy_static;
#[macro_use]extern crate log;

const INDEX_HTML: &str = "index.html";

lazy_static! {
    static ref SERVE_ROOT: Option<String> = std::env::args().nth(1);
}

async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    return SERVE_ROOT.as_ref()
        .and_then(|root| Some(root))
        .ok_or(error::ErrorNotFound("Womp womp"))
        .and_then(|root| {
            let path: PathBuf = req.match_info().query("filename").parse().unwrap();
            let p = Path::new(&root).join(path);
            return if p.is_file() {
                let file = NamedFile::open(p)?;
                Ok(file.use_last_modified(true))
            } else {
                let fbp = Path::new(&root).join(INDEX_HTML);
                let file = NamedFile::open(fbp)?;
                Ok(file.use_last_modified(true))
            };
        });
}

fn check_server_root_path() {
    let root = SERVE_ROOT.as_ref()
        .and_then(|root| Some(root))
        .expect("please specify a path to serve files")
        .as_str();
    let server_root = Path::new(&root);
    if !(server_root.is_dir()) {
        panic!("{:?} is not a directory", server_root)
    }
    let index = Path::new(&root).join(INDEX_HTML);
    if !(index.is_file()) {
        panic!("{:?} does not exit!", index)
    }
    info!("serving files in directory: {:?}", fs::canonicalize(&server_root).unwrap());
    info!("all nonexistent routes will be served with {:?}", fs::canonicalize(&index).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,actix_web=info");
    }
    env_logger::init();
    check_server_root_path();
    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .route("/{filename:.*}", web::get().to(index)))
        .bind("0:8088")?
        .run()
        .await
}