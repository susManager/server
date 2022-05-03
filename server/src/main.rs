mod todo_rest;

use todo_rest::todos_filter;
use warp::Filter;

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "hello from le root");

    let hi = warp::path("hi").and(warp::get()).map(|| "hi");

    let api = hi.or(todos_filter());

    let routes = api.or(root).or(hi);

    warp::serve(routes).run(([127,0,0,1], 5000)).await;
}
