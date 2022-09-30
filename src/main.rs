use spa_rs::spa_server_root;
use spa_rs::SpaServer;
use spa_rs::routing::{get, Router};
use anyhow::Result;

mod version;
mod unit;

use crate::unit::unit::Unit;


spa_server_root!("frontend/dist");           // specific your SPA dist file location

#[tokio::main]
async fn main() -> Result<()> {

    version::add_two(2);
    let unit = Unit::new(String::from("1"), String::from("Unit1"), String::from("Unit"), String::from("Unit"));
    print!("Unit id: {0}", unit.id());
    /*
    id: String::from("1"),
      name: String::from("1"),
      unitClass: String::from("unit"),
      unitFunc: String::from("unit"),
    };
    */
    // print!("Unit {0}", unit.id());

    let data = String::new();           // server context can be acccess by [axum::Extension]
    let mut srv = SpaServer::new();


    /*
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(test))
        // `POST /users` goes to `create_user`
        .route("/users", get(create_user));
    */

    let router = Router::new()
    .route("/get", get(test))
    .route("/units", get(units));

    srv.port(3000)
        .data(data)
        .static_path("/png", "web")     // static file generated in runtime
        .route("/api/v1", router);
        // .route("/api/v1", Router::new().route("/get", get(test))
             // .route("/units", get(units))
             // .route("/units", get(|| async { "get units" })
             // .route("/api/v1", Router::new()
        //     .route("/units", get(|| async { "get units..." })
        // )
    //);
    srv.run(spa_server_root!()).await?;


    async fn test() -> String {
      let val = String::from("Hello world");
      val
    }

    async fn units() -> String {
      let val = String::from("Units");
      val
    }

    Ok(())
}




/*
use axum::{
    body::{boxed, Full},
    handler::Handler,
    http::{header, StatusCode, Uri},
    response::Response,
    routing::Router,
  };
  use rust_embed::RustEmbed;
  use std::net::SocketAddr;

  static INDEX_HTML: &str = "index.html";

  #[derive(RustEmbed)]
  #[folder = "examples/axum-spa/assets/"]
  struct Assets;

  #[tokio::main]
  async fn main() {
    let app = Router::new().fallback(static_handler.into_service());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
  }

  async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
      return index_html().await;
    }

    match Assets::get(path) {
      Some(content) => {
        let body = boxed(Full::from(content.data));
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        Response::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
      }
      None => {
        if path.contains('.') {
          return not_found().await;
        }

        index_html().await
      }
    }
  }

  async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
      Some(content) => {
        let body = boxed(Full::from(content.data));

        Response::builder().header(header::CONTENT_TYPE, "text/html").body(body).unwrap()
      }
      None => not_found().await,
    }
  }

  async fn not_found() -> Response {
    Response::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("404"))).unwrap()
  }
*/

/*
// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // https://www.youtube.com/watch?v=uYhLWN86V48&t=23s
    // https://github.com/tokio-rs/axum

}
*/

/*
fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
*/