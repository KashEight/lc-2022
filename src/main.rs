use actix_files::Files;
use actix_web::{
    get, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use serde::Deserialize;

#[derive(Deserialize)]
struct WelcomeQuery {
    club: Option<String>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Access to /welcome!\n")
}

#[get("/welcome")]
async fn welcome(query: web::Query<WelcomeQuery>, req: HttpRequest) -> impl Responder {
    let club = query.club.as_deref().unwrap_or_default();
    if club != "LinuxClub" && club != "linuxclub" {
        HttpResponse::Ok().body("Access with correct query string `club` containing our club!\nNote: The name of our club does not need whitespace!\n")
    } else {
        let header = req.headers().get("My-Club");
        if header.is_none() || header.unwrap().to_str().unwrap() != "LinuxClub" {
            HttpResponse::Ok().body("Access with correct header `My-Club` containing our club!\n")
        } else {
            HttpResponse::Ok().body("Access to: https://lc-2022.hytus.net/welcome-to-linuxclub\n")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(welcome)
            .service(Files::new("/welcome-to-linuxclub", "./static/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
