use glob::glob;

fn main() {
    for entry in glob("/tmp/*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?} {:?}", path.display(), path.file_stem()),
            Err(e) => println!("{:?}", e),
        }
    }
}
