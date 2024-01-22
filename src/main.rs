use actix_web::{web, post, App, HttpResponse, HttpServer};
use serde::Deserialize;
use handlebars::Handlebars;
use std::collections::HashMap;

#[derive(Deserialize)]
struct FormData {
    name: String,
}

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = HashMap::<&str, &str>::new();
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[post("/form")]
async fn form(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().body(format!("name: {}", form.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "./templates/index.hbs")
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars.clone()))
            .route("/", web::get().to(index))
            .service(form)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
