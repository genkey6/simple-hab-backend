use actix_web::{get, App, HttpResponse, HttpServer};
use dotenv::dotenv;

use simple_hab_backend::api::init_routes;
use simple_hab_backend::db;
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
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    db::init();

    // HttpServer::new に App を作る関数を渡してサーバーを起動できる
    // App には service() という関数があり、App::new().service(hoge) のように関数を渡して登録する
    let app = move || App::new().service(hello).configure(init_routes);
    HttpServer::new(app).bind("0.0.0.0:8080")?.run().await?;
    Ok(())
}
