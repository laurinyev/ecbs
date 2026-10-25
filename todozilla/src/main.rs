use std::path::Path;

mod config;

fn main() {
    let config = config::Config::new(&Path::new("./TZ.yml"));
    println!("{config:?}");
}
