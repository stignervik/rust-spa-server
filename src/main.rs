//use actix_web::http::StatusCode;
use anyhow::Result;
use axum::routing::post;
use serde_json::json;
//use axum::response::IntoResponse;
//use serde_json::json;
use spa_rs::routing::{get, Router};
use spa_rs::spa_server_root;
use spa_rs::SpaServer;

mod unit;
use crate::unit::unit::Unit;
mod units;
use crate::units::Units;

mod version;

/// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;

/// See file data.rs, which defines the DATA global variable.
mod data;
use crate::data::DATA;
use crate::data::UNIT_STORE;

mod store;
// use crate::store::STORE_UNIT;

use std::sync::Arc;
use std::sync::RwLock;

/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

spa_server_root!("frontend/dist"); // specific your SPA dist file location

// type DogMap = Arc<Mutex<HashMap<String, String>>>; // name to breed

// use axum::AddExtensionLayer;

/*
type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    db: HashMap<String, String>,
}
*/

// type UnitMap = Arc<RwLock<HashMap<String, String>>>;

// use axum_macros;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use axum::extract::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let data = String::new(); // server context can be acccess by [axum::Extension]
    let mut srv = SpaServer::new();

    let unit_map = Arc::new(RwLock::new(Units::new()));
    // let unit_map: UnitMap = UnitMap::new(Arc::new(RwLock::new(HashMap::new())));

    // let shared_state = Arc::new(RwLock::new(HashMap::new()));

    let router = Router::new()
        .route(
            "/units",
            get({
                let unit_map = Arc::clone(&unit_map);
                move || get_units(Arc::clone(&unit_map))
            }),
        )
        .route(
            "/create_unit",
            post({
                let unit_map = Arc::clone(&unit_map);
                let unit_payload:Json<CreateUnit> = axum::Json(CreateUnit { name: "test".to_string() });
                move || create_unit(unit_payload, Arc::clone(&unit_map))
            }),
        )
        // .route("/create_unit", post(create_unit))
        .route("/user", post(create_user))
        .route("/hello/:name", get(json_hello))
        .route("/units_len", get(units_len))
        .route("/booklist", get(get_books))
        .route("/books/:id", get(get_books_id).delete(delete_books_id))
        .route("/add_unit", get(add_unit))
        .route(
            "/books/:id/form",
            get(get_books_id_form).post(post_books_id_form),
        );

    const PORT: u16 = 3000;
    println!("Starting server on port {}", PORT);
    srv.port(PORT)
        .data(data)
        .static_path("/png", "web") // static file generated in runtime
        .route("/api/v1", router);
    srv.run(spa_server_root!()).await?;

    Ok(())
}

/*
pub async fn create_unit() -> axum::response::Html<String> {
    println!("Put units...");
    thread::spawn(move || {
        let mut unit_store = UNIT_STORE.lock().unwrap();
        // data.insert(5, unit.clone());
        let mut len: u32 = unit_store.len() as u32;
        len = len + 1;
        println!("unit store: {}", unit_store.len());
        let unit = Unit::new(
            len,
            String::from("Unit1"),
            String::from("Unit"),
            String::from("Unit"),
        );
        unit_store.insert(unit.id, unit.clone());

        format!("Put unit: {}", &unit)
    })
    .join()
    .unwrap()
    .into()
}
*/

// the input to our `create_user` handler
#[derive(Deserialize, Clone)]
struct CreateUnit {
    name: String,
}

async fn create_unit(
// this argument tells axum to parse the request body
// as JSON into a `CreateUnit` type
    Json(payload): Json<CreateUnit>,
    unit_map: Arc<RwLock<Units>>,
) -> impl IntoResponse {

    // insert your application logic here
    /*
    let unit = CreateUnit {
        name: payload.name,
    };
    */

    let mut units = unit_map.write().unwrap();
        let index: u32 = units.length() as u32 + 1;
        let unit = Unit::new(
            index,
            payload.name,
            String::from("Unit"),
            String::from("Unit"),
        );

        // let json_unit = axum::Json(unit.clone());

        units.push(unit.clone());
        // let map_len = units.length();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
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

// #[axum_macros::debug_handler]
pub async fn get_units(
    unit_map: Arc<RwLock<Units>>,
    // ) -> axum::response::Html<String> {
) -> axum::extract::Json<String> {
    thread::spawn(move || {
        let units = unit_map.write().unwrap();
        let map_len = units.length();

        let mut res: String = "List: ".to_string();
        for (key, value) in units.get_units() {
            res = res + &*key.to_string() + " " + &*(value.id).to_string() + " - ";
        }

        format!(
            "<p>map length: {} res: {}</p>\n",
            &map_len, &res
        )
    })
    .join()
    .unwrap()
    .into()
}

pub async fn add_unit() -> axum::response::Html<String> {
    println!("Put units...");
    thread::spawn(move || {
        let unit_store = UNIT_STORE.lock().unwrap();
        let _count = unit_store.len();
        let value = "Hallo";
        format!("Unit length: yalla {}", &value)
    })
    .join()
    .unwrap()
    .into()
}

pub async fn units_len() -> axum::response::Html<String> {
    println!("Put units...");
    thread::spawn(move || {
        let unit_store = UNIT_STORE.lock().unwrap();
        let count = unit_store.len();

        format!("Unit length: yalla {}", count)
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "PUT /units" which creates a new unit resource.
/// This demo shows how axum can extract JSON data into a Unit struct.
pub async fn put_units(
    axum::extract::Json(_unit): axum::extract::Json<Unit>,
) -> axum::response::Html<String> {
    println!("Put units...");
    thread::spawn(move || {
        let mut unit_store = UNIT_STORE.lock().unwrap();
        // data.insert(5, unit.clone());
        let mut len: u32 = unit_store.len() as u32;
        len = len + 1;
        println!("unit store: {}", unit_store.len());
        let unit = Unit::new(
            len,
            String::from("Unit1"),
            String::from("Unit"),
            String::from("Unit"),
        );
        unit_store.insert(unit.id, unit.clone());

        format!("Put unit: {}", &unit)
    })
    .join()
    .unwrap()
    .into()
}

/// To access data, create a thread, spawn it, then get the lock.
/// When you're done, then join the thread with its parent thread.
#[allow(dead_code)]
async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}", data);
    })
    .join()
    .unwrap()
}

/// axum handler for "GET /books" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books
            .iter()
            .map(|&book| format!("<p>{}</p>\n", &book))
            .collect::<String>()
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "GET /books/:id" which responds with one resource HTML page.
/// This demo app uses our crate::DATA variable, and iterates on it to find the id.
pub async fn get_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!("<p>{}</p>\n", &book),
            None => format!("<p>Book id {} not found</p>", id),
        }
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "DELETE /books/:id" which destroys a resource.
/// This demo extracts an id, then mutates the book in the DATA store.
pub async fn delete_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&id) {
            data.remove(&id);
            format!("Delete book id: {}", &id)
        } else {
            format!("Book id not found: {}", &id)
        }
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "GET /books/:id/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_books_id_form(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!(
                concat!(
                    "<form method=\"post\" action=\"/books/{}/form\">\n",
                    "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                    "<p><input name=\"title\" value=\"{}\"></p>\n",
                    "<p><input name=\"author\" value=\"{}\"></p>\n",
                    "<input type=\"submit\" value=\"Save\">\n",
                    "</form>\n"
                ),
                &book.id, &book.id, &book.title, &book.author
            ),
            None => format!("<p>Book id {} not found</p>", id),
        }
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "POST /books/:id/form" which submits an HTML form.
/// This demo shows how to do a form submission then update a resource.
pub async fn post_books_id_form(form: axum::extract::Form<Book>) -> axum::response::Html<String> {
    let new_book: Book = form.0;
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&new_book.id) {
            data.insert(new_book.id, new_book.clone());
            format!("Post book: {}", &new_book)
        } else {
            format!("Book id not found: {}", &new_book.id)
        }
    })
    .join()
    .unwrap()
    .into()
}
