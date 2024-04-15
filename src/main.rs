use axum::{response::Html, routing::get, Router};
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[tokio::main]
async fn main() {
    let conn = Connection::open("/tmp/myfile.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    ).unwrap();
    let me = Person {
        id: 0,
        name: "Pero".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    ).unwrap();

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
   let conn = Connection::open("/tmp/myfile.db").unwrap();
   let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
   let person_iter = stmt.query_map([], |row| {
       Ok(Person {
           id: row.get(0)?,
           name: row.get(1)?,
           data: row.get(2)?,
       })
   }).unwrap();

   let found_person = format!("Hello {:?}", person_iter.last().unwrap().unwrap().name);

   Html(found_person)

}