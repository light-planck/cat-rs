fn main() {
    let args = cat_rs::get_args();

    if let Err(e) = cat_rs::run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
