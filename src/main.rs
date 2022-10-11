use anyhow::Result;
use spa_rs::routing::{get, Router};
use spa_rs::spa_server_root;
use spa_rs::SpaServer;

// use axum::Extension;

mod unit;
use crate::unit::unit::Unit;
mod units;
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

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
// use std::sync::Mutex;
/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

// use axum_sqlite::*;

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


#[tokio::main]
async fn main() -> Result<()> {
    let data = String::new(); // server context can be acccess by [axum::Extension]
    let mut srv = SpaServer::new();

    // let map: HashMap<String, String> = HashMap::new();
    // let wrapped_map = Arc::new(Mutex::new(map));

    // let subs:HashMap<String, String> = Arc::new(Mutex::new(HashMap::new()));
    // let subs2 = subs.clone();
    // https://medium.com/codex/updating-our-data-thread-safely-870f8585709

    // let shared_state = SharedState::default();

    let shared_state = Arc::new(RwLock::new(HashMap::new()));

    let router = Router::new()
        .route("/units", get({let shared_state = Arc::clone(&shared_state);
            move || get_units(Arc::clone(&shared_state))}))
        .route("/create_unit", get(create_unit))
        .route("/units_len", get(units_len))
        // .route("/books", get(get_books).put(put_books))
        .route("/booklist", get(get_books))
        .route("/books/:id", get(get_books_id).delete(delete_books_id))
        .route("/add_unit", get(add_unit))
        .route(
            "/books/:id/form",
            get(get_books_id_form).post(post_books_id_form),
        );
        // .layer(AddExtensionLayer::new(shared_state));

    // let unit_store = UNIT_STORE.lock().unwrap();
    // println!("unit store: {}", unit_store.len());
    // let _res = create_unit();
    // println!("create unit: {:#?}", res.await);
    // println!("unit store: {}", unit_store.len());

    /*
    let mut unit_store = UNIT_STORE.lock().unwrap();
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
    println!("unit store: {}", unit_store.len());
    // format!("Put unit: {}", &unit)
    */

    // let words = Arc::new(Mutex::new(vec![]));

    const PORT: u16 = 3000;
    println!("Starting server on port {}", PORT);
    srv.port(PORT)
        .data(data)
        .static_path("/png", "web") // static file generated in runtime
        .route("/api/v1", router);
    srv.run(spa_server_root!()).await?;

    Ok(())
}


/// axum handler for "GET /units" which responds with a resource page.
/// This demo uses our UNIT_STORE; a production app could use a database.
/// This demo must clone the UNIT_STORE in order to sort items by title.
pub async fn get_units(shared_state: Arc<RwLock<HashMap<String,String>>>) -> axum::response::Html<String> {
    thread::spawn(move || {

        let mut map = shared_state.write().unwrap();
        let index = map.len() + 1;
        map.insert(index.to_string(), "Test".to_string());
        let map_len = map.len();

        // let map_read = shared_state.read().unwrap();
        let mut res:String = "List: ".to_string();
        for (key, value) in &*map {
            res = res + key + " " + value +" - ";
        }



        /*
        let data = UNIT_STORE.lock().unwrap();
        let mut units = data.values().collect::<Vec<_>>().clone();
        units.sort_by(|a, b| a.name.cmp(&b.name));
        units
            .iter()
            .map(|&unit| format!("<p>{}</p>\n", &unit))
            .collect::<String>()
        */
        format!("<p>{} {}</p>\n", &map_len, &res)
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

/// axum handler for "PUT /units" which creates a new unit resource.
/// This demo shows how axum can extract JSON data into a Unit struct.
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

/// axum handler for "PUT /books" which creates a new book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
/*
pub async fn put_books(
    axum::extract::Json(book): axum::extract::Json<Book>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        data.insert(book.id + 1, book.clone());
        format!("Put book: {}", &book)
    })
    .join()
    .unwrap()
    .into()
}
*/

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

/*
pub async fn put_units(
    axum::extract::Json(unit): axum::extract::Json<Unit>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut unit_store = UNIT_STORE.lock().unwrap();
        unit_store.insert(5, unit.clone());
        format!("Put unit: {}", &unit)
    })
    .join()
    .unwrap()
    .into()
}
*/

/*
/// axum handler for "GET /unit" which responds with a resource page.
/// This demo uses our UNIT_STORE; a production app could use a database.
/// This demo must clone the UNIT_STORE in order to sort items by title.
pub async fn create_unit() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = UNIT_STORE.lock().unwrap();
        let mut units = data.values().collect::<Vec<_>>().clone();
        units.sort_by(|a, b| a.id.cmp(&b.id));

        /*
        let unit = Unit::new(
                    String::from("1"),
                    String::from("Unit1"),
                    String::from("Unit"),
                    String::from("Unit"),
            );

        */


        units
            .iter()
            .map(|&unit| format!("<p>{}</p>\n", &unit))
            .collect::<String>()

    })
    .join()
    .unwrap()
    .into()
}
*/
