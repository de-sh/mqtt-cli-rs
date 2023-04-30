use config::Config;

mod config;
fn main() {
    let config = Config::init();
    config.run();
}
