use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use figlet_rs::FIGfont;

mod recipe;
use recipe::sourdough;

async fn recipe(req: HttpRequest) -> impl Responder {
    let flower = req
        .match_info()
        .get("flower")
        .unwrap_or("1000")
        .parse::<i32>()
        .unwrap_or(1000);
    let hydration = req
        .match_info()
        .get("hydration")
        .unwrap_or("70")
        .parse::<i32>()
        .unwrap_or(70);
    let starter = req
        .match_info()
        .get("starter")
        .unwrap_or("20")
        .parse::<i32>()
        .unwrap_or(20);
    let salt = req
        .match_info()
        .get("salt")
        .unwrap_or("2")
        .parse::<i32>()
        .unwrap_or(2);
    let brine_water = req
        .match_info()
        .get("brine_water")
        .unwrap_or("5")
        .parse::<i32>()
        .unwrap_or(5);
    sourdough(flower, hydration, starter, salt, brine_water)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("SourDough");
    println!("{}", figure.unwrap());
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let url = "0.0.0.0";
    let port = "8080";
    println!("-----------> running on {}:{}", url, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route(
                "/{flower}-{hydration}-{starter}-{salt}-{brine_water}",
                web::get().to(recipe),
            )
            .route(
                "/{flower}-{hydration}-{starter}-{salt}",
                web::get().to(recipe),
            )
            .route("/{flower}-{hydration}-{starter}", web::get().to(recipe))
            .route("/{flower}-{hydration}", web::get().to(recipe))
            .route("/{flower}", web::get().to(recipe))
            .route("/", web::get().to(recipe))
    })
    .bind(format!("{}:{}", url, port))?
    .run()
    .await
}
