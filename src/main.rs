use anyhow::Result;
use spa_rs::routing::{get, Router};
use spa_rs::spa_server_root;
use spa_rs::SpaServer;

mod unit;
mod units;
mod version;

use crate::unit::unit::Unit;
use crate::units::Units;

/// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;

/// See file data.rs, which defines the DATA global variable.
mod data;
use crate::data::DATA;
use crate::data::UNIT_STORE;

/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

spa_server_root!("frontend/dist"); // specific your SPA dist file location

#[tokio::main]
async fn main() -> Result<()> {
    let mut units = Units::new().clone();
    let mut _numlist: Vec<i32>;

    let unit = Unit::new(
        String::from("1"),
        String::from("Unit1"),
        String::from("Unit"),
        String::from("Unit"),
    );
    unit.id();

    units.push(unit);
    units.length();

    let data = String::new(); // server context can be acccess by [axum::Extension]
    let mut srv = SpaServer::new();


    let createunit = || async {
        let _unit = Unit::new(
            String::from("1"),
            String::from("Unit1"),
            String::from("Unit"),
            String::from("Unit"),
        );
        // units.push(unit);

        // Units::typeofcol();
        // units.push(unit);
    };

    let router = Router::new()
        .route("/get", get(test))
        .route("/units", get(get_units))
        .route("/createunit", get(createunit))
        .route("/books", get(get_books).put(put_books))
        .route("/books/:id", get(get_books_id).delete(delete_books_id))
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

    async fn test() -> String {
        let val = String::from("Hello world");
        val
    }

    async fn unitsstr() -> String {
        let val = String::from("Units");
        val
    }

    Ok(())
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
pub async fn put_books(
    axum::extract::Json(book): axum::extract::Json<Book>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        data.insert(book.id, book.clone());
        format!("Put book: {}", &book)
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

/// axum handler for "GET /units" which responds with a resource page.
/// This demo uses our UNIT_STORE; a production app could use a database.
/// This demo must clone the UNIT_STORE in order to sort items by title.
pub async fn get_units() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = UNIT_STORE.lock().unwrap();
        let mut units = data.values().collect::<Vec<_>>().clone();
        units.sort_by(|a, b| a.id.cmp(&b.id));
        units
            .iter()
            .map(|&unit| format!("<p>{}</p>\n", &unit))
            .collect::<String>()
    })
    .join()
    .unwrap()
    .into()
}

