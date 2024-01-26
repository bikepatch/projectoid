use std::{env, fs};
use std::io::Write;

pub trait NemoFinder {
    fn make_search(&self, path: &str, nemo_to_find: &str, print_list: &mut Vec<String>);
    fn make_print(&self, sort_flag: bool, print_list: &mut Vec<String>, output: &str);
}

pub trait PrintStrategy {
    fn make_print(&self, print_list: &Vec<String>);
}

pub struct DirSeeker;
pub struct Outputo;
pub struct Filo{
    fname: String
}


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
                        let relative_path = printable_path.to_str().unwrap();
                        let fname = printable.file_name();
                        if !nemo_to_find.is_empty() {
                            if fname == nemo_to_find {
                                print_list.push(relative_path.parse().unwrap());
                            }
                        } else {
                            print_list.push(relative_path.parse().unwrap());
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

    fn make_print(&self, sort_flag: bool, print_list: &mut Vec<String>, output: &str) {
        if sort_flag {
            bubble(print_list);
        }
        if output != "" {
            if let Ok(mut file) = fs::OpenOptions::new().write(true).open(output) {
                for printable in print_list {
                    writeln!(file, "{:?}", printable).expect("Failed to write into a file");
                }
            }
        } else {
            for printable in print_list {
                eprintln!("{:?}", printable);
            }
        }
    }
}

impl PrintStrategy for Outputo {
    fn make_print(&self, print_list: &Vec<String>) {
        todo!()
    }
}

impl PrintStrategy for Filo {
    fn make_print(&self, print_list: &Vec<String>) {
        todo!()
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let my_path = &args[1];
    let mut nemo_to_find: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut sort_flag = false;
    let mut print_list : Vec<String> = Vec::new();

    if let Some(index) = args.iter().position(|value| value == "--find") {
        nemo_to_find = args.get(index + 1).cloned();
    };

    if let Some(index) = args.iter().position(|value| value == "-f") {
        output_file = args.get(index + 1).cloned();
    };

    if args.iter().any(|val| val == "--sort") {
        sort_flag = true;
    }

    DirSeeker.make_search(my_path, nemo_to_find, &mut print_list);

    if sort_flag {
        bubble(&mut print_list);
    }

    let print_strategy: Box<dyn PrintStrategy> = match &output_file {
        None => {Box::new(Outputo)},
        Some(name) => {Box::new(Filo{ fname: name.clone() })}
    };

    print_strategy.make_print(&print_list);


    // Joke
    println!("Some Yoda Thoughts It Was");
}
