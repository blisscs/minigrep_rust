use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _query = &args[1];
    let _file_path = &args[2];
}
