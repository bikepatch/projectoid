use std::{env, fs};


fn bubble <T: PartialOrd>(arr: &mut[T]){
    let n = arr.len();
    for i in 0..n {
        for j in 0..n {
            if arr[i] > arr[j+1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

trait NemoFinder {
    fn make_search(&self, path: &str, nemo_to_find: &str, sort_flag: bool);
}

struct DirSeeker;

impl NemoFinder for DirSeeker{
    fn make_search(&self, path: &str, nemo_to_find: &str, sort_flag: bool) {
        let mut print_list : Vec<String> = Vec::new();
        if let Ok(printables) = fs::read_dir(&path) {
            for printable in printables {
                if let Ok(printable) = printable {
                    let printable_path = printable.path();
                    if printable_path.is_file() {
                        let fname = printable_path.file_name().unwrap().to_str().unwrap();
                        if !nemo_to_find.is_empty() {
                            if fname == nemo_to_find {
                                eprintln!("Found Nemo at: {:?}", printable_path);
                            }
                        } else {
                            eprintln!("{:?}", fname);
                        }
                    } else {
                        self.make_search(printable_path.to_str().unwrap(), nemo_to_find, sort_flag)
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
        &args[3]
    } else{
        ""
    };

    DirSeeker.make_search(my_path, nemo_to_find, true);


    // Joke
    println!("Some Yoda Thoughts It Was");
}
