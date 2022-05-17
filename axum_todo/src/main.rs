use axum::{routing::get, Router};
use dotenv::dotenv;

mod config;
mod handler;
mod error;

/// 定义自己的 Result
type Result<T> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    // 解析 .env 文件
    dotenv().ok();

    let app = Router::new().route("/", get(handler::usage));

    let cfg = config::Config::from_env().expect("初始化配置失败");
    println!("{:?}", cfg);

    // 绑定到配置文件设置的地址
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // let cfg = config::Config::from_env().expect("初始化配置失败");
    // println!("{:?}", cfg); // Config { web: WebConfig { addr: "0.0.0.0:9527" } }
}
