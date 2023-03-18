use actix_files::NamedFile;
use actix_web::{HttpRequest,Result};
use qstring::QString;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = QString::from(req.query_string())
        .get("filename")
        .unwrap()
        .into();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = String::from("127.0.0.1");
    let port = 6969;

    use actix_web::{web, App, HttpServer};

    println!("Server listening on: http://{}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/static", ".").show_files_listing())
            .service(web::redirect("/", "/home?filename=index.html"))
            .route("/home{filename:.*}", web::get().to(index))
    })
    .bind((host, port))?
    .run()
    .await
}
