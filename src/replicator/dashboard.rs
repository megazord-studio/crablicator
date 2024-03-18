use tokio::task::JoinHandle;
use warp::Filter;

pub fn run_server() -> JoinHandle<()> {
    let hello_route = warp::path::end().map(|| warp::reply::html("Hello, world!"));
    tokio::spawn(async move { warp::serve(hello_route).run(([127, 0, 0, 1], 3030)).await })
}
