use std::{env, fs};
use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::path::Path;

pub trait NemoFinder {
    fn make_search(&self, path: &str, print_list: &mut Vec<Occurience>);
}

pub trait PrintStrategy {
    fn make_print(&self, print_list: &Vec<Occurience>);
}

pub struct DirSeeker;
pub struct Outputo;
pub struct Filo{
    fname: String
}

pub enum Occurience{
    File(String),
    Directory(String),
    TextFile(String),
    InFileText(String)
}

fn bubble( arr: &mut Vec<Occurience>){
    let n = arr.len();
    for i in 0..n {
        for j in 0..(n - 1){
            if arr[j].file_name() > arr[j + 1].file_name() {
                arr.swap(j, j + 1);
            }
        }
    }
}



impl NemoFinder for DirSeeker{
    fn make_search(&self, path: &str, print_list: &mut Vec<Occurience>) {
        if let Ok(printables) = fs::read_dir(&path) {
            for printable in printables {
                if let Ok(printable) = printable {
                    let printable_path = printable.path();
                    let print_str = printable_path.to_str().unwrap();
                    if printable_path.is_file() {
                        if let Some(extenstion) = printable_path.extension().and_then(|s| s.to_str()){
                            if extenstion == "rs" || extenstion == "txt" {
                                print_list.push(Occurience::TextFile(print_str.parse().unwrap()));
                            } else {
                                print_list.push(Occurience::File(print_str.parse().unwrap()));
                            }
                        }
                    } else {
                        if printable_path.is_dir() {
                            print_list.push(Occurience::Directory(print_str.parse().unwrap()));
                            self.make_search(print_str, print_list);
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to open folder entered.");
            std::process::exit(1);
        }
    }
}

impl PrintStrategy for Outputo {
    fn make_print(&self, print_list: &Vec<Occurience>) {
        for printable in print_list {
            match printable { Occurience::File(path) | Occurience::Directory(path) | Occurience::TextFile(path) | Occurience::InFileText(path) =>
                {
                    eprintln!("{:?}", path);
                }
            }
        }
    }
}

impl PrintStrategy for Filo {
    fn make_print(&self, print_list: &Vec<Occurience>) {
        if let Ok(mut file) = fs::OpenOptions::new().write(true).truncate(true).create(true).open(&self.fname) {
            for printable in print_list {
                match printable { Occurience::File(path) | Occurience::Directory(path) | Occurience::TextFile(path) | Occurience::InFileText(path) =>
                    {
                        writeln!(file, "{:?}", path).expect("Failed to write into a file");
                    }
                }
            }
        }
    }
}

impl Occurience{
    fn file_name(&self) -> &str {
        match self {Occurience::File(path) | Occurience::Directory(path) | Occurience::TextFile(path) | Occurience::InFileText(path) => path}
    }
}

fn find_in_file(path: &str, nemo_to_find: &str, print_list: & mut Vec<Occurience>) -> Result<(), std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    for (index, line) in text.lines().enumerate() {
        if line.contains(nemo_to_find) {
            print_list.push(Occurience::InFileText(format!("{}: Line {}: {}", path, index + 1, line)));
        }
    }
    Ok(())
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let puppy = "d";
    let my_path = &args[1];
    let mut nemo_to_find: Option<&str> = None;
    let mut output_file: Option<&str> = None;
    let mut in_file_flag = false;
    let mut sort_flag = false;
    let mut print_list = Vec::new();
    let mut in_text_list = Vec::new();

    if let Some(index) = args.iter().position(|value| value == "--find") {
        nemo_to_find = args.get(index + 1).map(|s| s.as_str());
    };

    if let Some(index) = args.iter().position(|value| value == "-f") {
        output_file = args.get(index + 1).map(|s| s.as_str());
    };

    if args.iter().any(|val| val == "--in-file") {
        in_file_flag = true;
    }

    if args.iter().any(|val| val == "--sort") {
        sort_flag = true;
    }

    DirSeeker.make_search(my_path, &mut print_list);

    if in_file_flag {
        for printable in &print_list{
            if let Occurience::TextFile(path) = printable {
                if let Err(e) = find_in_file(path, nemo_to_find.unwrap(), &mut in_text_list) {
                    eprintln!("Error reading file {}: {}", path, e);
                }
            }
        }
        print_list = in_text_list;
    } else {
        if let Some(name) = nemo_to_find {
            print_list.retain(|printable| {
                let printable_path = match printable {
                    Occurience::File(path) | Occurience::Directory(path) | Occurience::TextFile(path) | Occurience::InFileText(path) => path
                };
                Path::new(printable_path).file_name().and_then(|fname| fname.to_str()).map(|fname| fname == name).unwrap_or(false)
            });
        }
    }

    if sort_flag {
        bubble(&mut print_list);
    }

    let print_strategy: Box<dyn PrintStrategy> = match output_file {
        None => {Box::new(Outputo)},
        Some(name) => {Box::new(Filo{ fname: name.to_string() })}
    };

    print_strategy.make_print(&print_list);


    // Joke
    println!("Some Yoda Thoughts It Was");
}
