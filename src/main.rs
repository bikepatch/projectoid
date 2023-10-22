use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let my_path = &args[1];

    println!("{}", my_path);
}
