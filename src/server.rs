mod recipe;
use recipe::sourdough;
use warp::Filter;

#[tokio::main]
async fn main() {
    let sd1 = warp::path!( i32 ).map(|a| sourdough(a,70,20,2,5));
    // let sd2 = warp::path!( i32 / i32).map(|a, b| sourdough(a,b,20,2,5));
    // let sd0 = warp::path!("").map(|| sourdough(1000,70,20,2,5));
    let routes =  warp::get().and(sd1);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
