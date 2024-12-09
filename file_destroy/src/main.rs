use file_destroy::destroy;
use std::env;

struct Props {
    pub file_path: String,
    pub quality: Option<usize>,
    pub rename: bool,
    pub help: bool,
}

impl Props {
    pub fn new() -> Self {
        Self { file_path: "".to_string(), quality: Some(3), rename: false, help: false }
    }
}

fn main() {
    let mut props = Props::new();
    let args: Vec<String> = env::args().collect();
    let (n, mut c) = (args.len(), 1);
    loop {
        if c >= n { break; }
        match args[c].as_str() {
            "-f" => {
                if c + 1 >= n { break; }
                props.file_path = args[c + 1].to_string();
                c += 2;
            }
            "-q" => {
                if c + 1 >= n { break; }
                props.quality = match args[c + 1].parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                c += 2;
            }
            "-r" => {
                props.rename = true;
                c += 1;
            }
            "-h" => {
                props.help = true;
                c += 1;
            }
            _ => { c += 1; }
        }
    }

    if props.help {
        println!("-f [file path]: File path you want to delete. (REQUIRED)");
        println!("-q [integer >= 1]: Number of times to overwrite file contents.");
        println!("-r: Rename the file to random string before deleting.");
        println!("-h: Show help.");
        return;
    }
    if props.file_path.len() == 0 {
        eprintln!("Specify the file name after -f.");
        eprintln!("You can refer to help with -h.");
        return;
    }
    if props.quality.is_none() {
        eprintln!("Specify -q as an integer value.");
        eprintln!("You can refer to help with -h.");
        return;
    }

    match destroy(&props.file_path, props.quality.unwrap(), props.rename) {
        Ok(_) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}