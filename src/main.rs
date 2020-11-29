use actix_web::{get, web, App, HttpResponse, HttpServer};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use simple_hab_backend::api::{delete_user, get_user, list, post_user};
use simple_hab_backend::MyError;

#[get("/hello")]
async fn hello() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";

    // HttpResponse::Ok() はステータスコード 200 を持つ HttpResponseBuilder という構造体を返す
    // HttpResponseBuilder の body() という関数にレスポンスのボディを渡すと HttpResponse が返ってくる
    // 戻り値が Result 型なので Ok で包む
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("user.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed to create a table 'user'.");

    // HttpServer::new に App を作る関数を渡してサーバーを起動できる
    // App には service() という関数があり、App::new().service(hoge) のように関数を渡して登録する
    let app = move || {
        App::new().service(hello).service(
            web::scope("/")
                .data(pool.clone())
                .service(list)
                .service(get_user)
                .service(post_user)
                .service(delete_user),
        )
    };
    HttpServer::new(app).bind("0.0.0.0:8080")?.run().await?;
    Ok(())
}
