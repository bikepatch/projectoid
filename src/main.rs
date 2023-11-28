use std::{env, fs};
use std::cell::RefCell;
use std::rc::Rc;

trait NemoFinder {
    fn make_search(self, path: String, nemo_to_find: String);
}

struct DirSeeker;

impl NemoFinder for DirSeeker{
    fn make_search(self, path: String, nemo_to_find: String) {
        todo!()
    }
}

fn main() {
    // Read what comes

    let args: Vec<String> = env::args().collect();

    // Take the dir name
    let my_path = &args[1];

    // Read all
    let printables = match fs::read_dir(my_path) {
        Ok(entered) => entered,
        Err(err) => {
            eprintln!("Failed to open folder entered: {}", err);
            std::process::exit(1);
        }
    };

    // Deal one by one
    for printable in printables {
        match printable {
            Ok(name_to_print) => {
                if name_to_print.path().is_file() {
                    eprintln!("{:?}", name_to_print.file_name());
                }
            },
            Err(err) => {
                eprintln!("Failed to open file/folder: {}", err);
            }
        }
    }

    // Joke
    println!("Some Yoda Thoughts It Was");
}
