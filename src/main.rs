use std::process;

use clap;

use shared_secrets::Config;

fn main() {
    let yaml = clap::load_yaml!("../config/args.yaml");
    let app = clap::App::from_yaml(yaml);

    let config = Config::new(app.get_matches()).unwrap_or_else(|e| {
        eprintln!("Problem parsing argument: {}", e);
        process::exit(1);
    });

    if let Err(e) = shared_secrets::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
