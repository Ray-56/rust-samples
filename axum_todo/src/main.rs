use dotenv::dotenv;

use serde::Deserialize;

mod config;

/// Web 配置
#[derive(Deserialize, Debug)]
pub struct WebConfig {
    pub addr: String,
}

/// 应用配置
#[derive(Deserialize, Debug)]
pub struct Config {
    pub web: WebConfig,
}

#[tokio::main]
async fn main() {
    // 解析 .env 文件
    dotenv().ok();

    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }

    // let config = config::Config::builder()
    //     .add_source(config::Environment::default())
    //     .build()
    //     .unwrap();

    // let app: Config = config.try_deserialize().unwrap();

    // let config = config::Config::parse();

    let cfg = config::Config::from_env().expect("初始化配置失败");
    println!("{:?}", cfg);

    // let cfg = config::Config::from_env().expect("初始化配置失败");
    // println!("{:?}", cfg); // Config { web: WebConfig { addr: "0.0.0.0:9527" } }
}
