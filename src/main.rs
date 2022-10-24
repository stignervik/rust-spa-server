use anyhow::Result;
use axum::routing::post;
use serde_json::json;
use spa_rs::routing::{get, Router};
use spa_rs::spa_server_root;
use spa_rs::SpaServer;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod unit;
use crate::unit::Unit;
mod units;
use crate::units::Units;

mod version;

mod store;
use crate::store::Store;

use std::sync::Arc;
use std::sync::RwLock;

/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

spa_server_root!("frontend/dist"); // specific your SPA dist file location

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use axum::extract::Path;

use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let data = String::new(); // server context can be acccess by [axum::Extension]
    let mut srv = SpaServer::new();

    // Fetch the units from store
    let store = Store::new();
    let units = store.units();

    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .route(
            "/units",
            get({
                let units = Arc::clone(&units);
                move || get_units(units)
            }),
        )
        .route(
            "/create_unit",
            post({
                let units = Arc::clone(&units);
                move |payload: Json<serde_json::Value>| create_unit(payload, Arc::clone(&units))
            }),
        )
        .route(
            "/delete_unit",
            post({
                let units = Arc::clone(&units);
                move |payload: Json<serde_json::Value>| delete_unit(payload, Arc::clone(&units))
            }),
        )
        .route("/user", post(create_user))
        .route("/hello/:name", get(json_hello))
        .route(
            "/units_len",
            get({
                let units = Arc::clone(&units);
                move || units_len(Arc::clone(&units))
            }),
        );

    const PORT: u16 = 3000;
    println!("Starting server on port {}", PORT);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    srv.port(PORT)
        .data(data)
        .static_path("/png", "web") // static file generated in runtime
        .route("/api/v1", router);
    srv.run(spa_server_root!())
        .await
        .expect("failed to start server");

    Ok(())
}

async fn create_unit(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUnit` type
    // Json(payload): Json<CreateUnit>,
    Json(payload): Json<serde_json::Value>,
    unit_map: Arc<RwLock<Units>>,
) -> impl IntoResponse {
    let mut units = unit_map.write().unwrap();
    // let _index: u32 = units.length() as u32 + 1;



    let u: Unit = serde_json::from_str(&payload.to_string()).unwrap();

    // unit_id.hyphenated().to_string(),


    let index = units.length().to_string();
    let name = "Unit ".to_string() + &index;

    let unit_id = Uuid::new_v4();
    let unit = Unit::new(
        unit_id.hyphenated().to_string(),
        name,
        u.unit_class.to_string(),
        u.unit_func.to_string(),
    );

    // let json_unit = axum::Json(unit.clone());

    units.push(unit.clone());
    // let map_len = units.length();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(unit))
}

async fn delete_unit(
    Json(payload): Json<serde_json::Value>,
    unit_map: Arc<RwLock<Units>>,
) -> impl IntoResponse {
    let mut units = unit_map.write().unwrap();
    let u: Unit = serde_json::from_str(&payload.to_string()).unwrap();

    let unit = Unit::new(
        u.id,
        u.name,
        u.unit_class.to_string(),
        u.unit_func.to_string(),
    );

    // let json_unit = axum::Json(unit.clone());

    units.delete_unit(unit.id.clone());
    // units.push(unit.clone());
    // let map_len = units.length();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(unit))
}

// #[axum_macros::debug_handler]
pub async fn get_units(
    unit_map: Arc<RwLock<Units>>,
    //    ) -> axum::response::Json<Unit> {
) -> impl IntoResponse {
    let units = unit_map.read().unwrap();
    if units.length() > 0 {
        // units.get_units()
    } else {
    }

    let unit_id = Uuid::new_v4();
    let unit = Unit::new(
        unit_id.hyphenated().to_string(),
        "Unit".to_string(),
        "Unit".to_string(),
        "Unit".to_string(),
    );

    (StatusCode::CREATED, Json(unit))
}

async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
    let greeting = name.as_str();
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({ "message": hello + greeting })))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
struct User {
    id: u64,
    username: String,
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

pub async fn units_len(unit_map: Arc<RwLock<Units>>) -> axum::response::Html<String> {
    println!("Put units...");
    thread::spawn(move || {
        let units = unit_map.read().unwrap();
        let count = units.length();
        format!("Unit length:{}", count,)
    })
    .join()
    .unwrap()
    .into()
}
