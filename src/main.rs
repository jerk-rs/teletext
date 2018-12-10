use dotenv::dotenv;
use std::env;
use teletext::run;

fn main() {
    dotenv().ok();
    env_logger::init();
    let token = env::var("TELETEXT_TOKEN").expect("Can not to get token");
    run(token);
}
