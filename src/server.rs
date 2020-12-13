mod recipe;
use recipe::sourdough;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn recipe(req: HttpRequest) -> impl Responder {
    let flower = req.match_info().get("flower").unwrap_or("1000").parse::<i32>().unwrap_or(1000);
    let hydration = req.match_info().get("hydration").unwrap_or("70").parse::<i32>().unwrap_or(70);
    let starter = req.match_info().get("starter").unwrap_or("20").parse::<i32>().unwrap_or(20);
    let salt = req.match_info().get("salt").unwrap_or("2").parse::<i32>().unwrap_or(2);
    let brine_water = req.match_info().get("brine_water").unwrap_or("5").parse::<i32>().unwrap_or(5);
    sourdough(flower,hydration,starter,salt,brine_water)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{flower}-{hydration}-{starter}-{salt}-{brine_water}", web::get().to(recipe))
            .route("/{flower}-{hydration}-{starter}-{salt}", web::get().to(recipe))
            .route("/{flower}-{hydration}-{starter}", web::get().to(recipe))
            .route("/{flower}-{hydration}", web::get().to(recipe))
            .route("/{flower}", web::get().to(recipe))
            .route("/", web::get().to(recipe))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

