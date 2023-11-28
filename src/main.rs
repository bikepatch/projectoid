use std::{env, fs};
use std::cell::RefCell;
use std::rc::Rc;

trait NemoFinder {
    fn make_search(self, path: String, nemo_to_find: String);
}

struct DirSeeker;

impl NemoFinder for DirSeeker{
    fn make_search(self, path: String, nemo_to_find: String) {
        if let Ok(printables) = fs::read_dir(&path) {
            for printable in printables {
                match printable {
                    Ok(printable_name) => {
                        if printable_name.path().is_file() {
                            if !nemo_to_find.is_empty() {
                                if printable_name.file_name() == nemo_to_find {
                                    eprintln!("Found Nemo ar: {:?}", printable_name.file_name());
                                }
                            } else {
                                eprintln!("{:?}", printable_name.file_name());
                            }
                        } else if printable.is_dir() {
                            self.make_search(printable.file_name(), nemo_to_find.clone())
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to open file/folder: {}", err);
                    }
                }
            }
        } else {
            eprintln!("Failed to open folder entered.");
            std::process::exit(1);
        }
    }
}

fn main() {
    // Read what comes

    let args: Vec<String> = env::args().collect();

    // Take the dir name
    let my_path = &args[1];

    let nemo_to_find = if args.len() > 3 && args[2] == "--find" {
        args[3].clone()
    } else{
        String::new()
    };

    DirSeeker.make_search(my_path.clone(), nemo_to_find);


    // Joke
    println!("Some Yoda Thoughts It Was");
}
