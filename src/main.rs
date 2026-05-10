fn main() {
    if let Err(error) = velvet_21::run() {
        eprintln!("Error: {error}");
        std::process::exit(1)
    }
}
