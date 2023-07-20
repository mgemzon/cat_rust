fn main() {
    if let Err(e) = cat_rust::get_args().and_then(|config| cat_rust::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
