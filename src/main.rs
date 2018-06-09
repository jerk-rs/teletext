extern crate dotenv;
extern crate teletext;

use dotenv::dotenv;
use std::env;
use teletext::App;

fn main() {
    dotenv().ok();
    let token = env::var("TELETEXT_TOKEN").expect("Can not to get token");
    let app = App::new(&token).expect("Failed to create app");
    app.run().expect("Run failed");
}
