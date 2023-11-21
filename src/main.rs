use std::collections::HashMap;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a warp filter for the /hello endpoint with an optional query parameter "name"
    let hello = warp::path!("ytdl")
        .and(warp::query::<HashMap<String, String>>())
        .map(|query_params: HashMap<String, String>| {
            // Access the "name" query parameter if it exists
            let name = query_params
                .get("url")
                .cloned()
                .unwrap_or_else(|| String::from("world"));
            warp::reply::html(format!("Download Dizz: {}!", name))
        });

    // Combine the filter with the warp::serve function to create the server
    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
