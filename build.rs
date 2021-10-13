use lalrpop;
use vergen::{vergen, Config};

fn main() {
    lalrpop::process_root().unwrap();
    vergen(Config::default()).unwrap();
}
