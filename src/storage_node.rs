use std::collections::HashMap;
use tokio::sync::Mutex;
use warp::Filter;
use std::sync::Arc;


#[derive(Clone)]
struct Storage {
    data: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

#[tokio::main]
async fn main() {
    let storage = Storage {
        data: Arc::new(Mutex::new(HashMap::new())),
    };

    let store = warp::path("store")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_storage(storage.clone()))
        .and_then(store_chunk);

    let retrieve = warp::path("retrieve")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_storage(storage))
        .and_then(retrieve_chunk);

    warp::serve(store.or(retrieve)).run(([127, 0, 0, 1], 3030)).await;
}

fn with_storage(storage: Storage) -> impl Filter<Extract = (Storage,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

async fn store_chunk(chunk: Vec<u8>, storage: Storage) -> Result<impl warp::Reply, warp::Rejection> {
    let hash = base64::encode(&chunk);
    storage.data.lock().await.insert(hash.clone(), chunk);
    Ok(warp::reply::json(&hash))
}

async fn retrieve_chunk(hash: String, storage: Storage) -> Result<impl warp::Reply, warp::Rejection> {
    let data = storage.data.lock().await.get(&hash).cloned();
    match data {
        Some(chunk) => Ok(warp::reply::json(&chunk)),
        None => Err(warp::reject::not_found()),
    }
}
