extern crate pretty_env_logger;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use log::{info, warn};
use warp::Filter;

const INDEX_HTML: &str = "index.html";

fn check_server_root_path() -> PathBuf {
    let root = env::args().nth(1)
        .expect("please specify a path to serve files");
    let server_root = Path::new(&root);
    if !(server_root.is_dir()) {
        panic!("{:?} is not a directory", server_root)
    }
    let index = Path::new(&root).join(INDEX_HTML);
    if !(index.is_file()) {
        panic!("{:?} does not exit!", index)
    }
    info!("serving files in directory: {:?}", fs::canonicalize(&server_root).unwrap());
    info!("all nonexistent routes will be served with {:?}", fs::canonicalize(&index).unwrap());
    return server_root.to_owned();
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let server_root = check_server_root_path();
    if !server_root.is_dir() {
        panic!("{:?} is not a directory", server_root)
    }

    let route = warp::get()
        .and(
            warp::fs::dir(server_root.clone()).or(
                warp::fs::file(server_root.join(INDEX_HTML))
            )
        ).with(warp::log("info"));

    let server = warp::serve(route);

    let tls_path = env::var("TLS_DIR").ok().and_then(|tls_dir| {
        let p = Path::new(&tls_dir);
        if p.is_dir() {
            Some(p.to_owned())
        } else {
            warn!("TLS_DIR defined but not valid");
            None
        }
    });
    match tls_path {
        Some(tls_dir) => {
            let key = Path::new(&tls_dir).join("key.pem");
            let cert = Path::new(&tls_dir).join("cert.pem");
            info!("using TLS key: {:?}", &key);
            info!("using TLS cert: {:?}", &cert);
            server.tls()
                .cert_path(cert)
                .key_path(key)
                .run(([0, 0, 0, 0], 8443)).await;
        }
        None => {
            info!("TLS is not configured! Set env variable TLS_DIR=<path to cert.pem and key.pem> to use TLS.");
            server.run(([0, 0, 0, 0], 8088)).await;
        }
    }
}
