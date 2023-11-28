use std::{env, fs};

trait NemoFinder {
    fn make_search(&self, path: &str, nemo_to_find: &str, print_list: &mut Vec<String>);
    fn make_print(&self, sort_flag: bool, print_list: &mut Vec<String>);
}

struct DirSeeker;


fn bubble( arr: &mut Vec<String>){
    let n = arr.len();
    for i in 0..n {
        for j in 0..(n - 1){
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}



impl NemoFinder for DirSeeker{
    fn make_search(&self, path: &str, nemo_to_find: &str, print_list: &mut Vec<String>) {
        if let Ok(printables) = fs::read_dir(&path) {
            for printable in printables {
                if let Ok(printable) = printable {
                    let printable_path = printable.path();
                    if printable_path.is_file() {
                        let fname = printable_path.file_name().unwrap().to_str().unwrap();
                        if !nemo_to_find.is_empty() {
                            if fname == nemo_to_find {
                                eprintln!("Found Nemo at: {:?}", printable_path);
                                break;
                            }
                        } else {
                            print_list.push(fname.to_string())
                        }
                    } else {
                        self.make_search(printable_path.to_str().unwrap(), nemo_to_find, print_list)
                    }
                }
            }
        } else {
            eprintln!("Failed to open folder entered.");
            std::process::exit(1);
        }
    }

    fn make_print(&self, sort_flag: bool, print_list: &mut Vec<String>) {
        if sort_flag {
            bubble(print_list);
        }
        for printable in print_list {
            eprintln!("{:?}", printable);
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let my_path = &args[1];
    let mut nemo_to_find = "";
    let mut sort_flag = false;
    let mut print_list : Vec<String> = Vec::new();

    if let Some(index) = args.iter().position(|value| value == "--find") {
        if index + 1 < args.len() {
            if let Some(val) = args.get(index + 1) {
                nemo_to_find = val;
            }
        }
    };

    if args.iter().any(|val| val == "--sort") {
        sort_flag = true;
    }

    DirSeeker.make_search(my_path, nemo_to_find, &mut print_list);
    DirSeeker.make_print(sort_flag, &mut print_list);


    // Joke
    println!("Some Yoda Thoughts It Was");
}
