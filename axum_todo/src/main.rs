// mod config;

use config::Environment;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    let a = dotenv().ok();
    // let env = Environment;
    println!("{:#?}", a);
}
