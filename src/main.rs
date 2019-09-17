use std::fs;
use std::env;
use std::io;
use clap::{Arg, App};
use std::io::Read;
use std::str::from_utf8;
use std::collections::HashMap;


fn load_built_in_files() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("alpaca".to_string(), from_utf8(include_bytes!("cows/alpaca.cow")).unwrap().to_string());
    map.insert("default".to_string(), from_utf8(include_bytes!("cows/default.cow")).unwrap().to_string());
    map.insert("dragon".to_string(), from_utf8(include_bytes!("cows/dragon.cow")).unwrap().to_string());
    map.insert("tux".to_string(), from_utf8(include_bytes!("cows/tux.cow")).unwrap().to_string());
    map.insert("vader".to_string(), from_utf8(include_bytes!("cows/vader.cow")).unwrap().to_string());
    map
}

/// Prints speech bubble for a given text
/// For now nat handling multiple lines
fn draw_bubble(text: &str) {
    let length = text.len() + 2;
    let mut first_line = String::from(" ");
    first_line.push_str(&String::from("_").repeat(length));
    println!("{}", first_line);

    println!("{}{}{}", "< ", text, " >");

    let mut last_line = String::from(" ");
    last_line.push_str(&String::from("-").repeat(length));
    println!("{}", last_line);
}

fn draw_cow_file(contents: &str) {
    let image = contents
                .lines()
                .filter(|line| !line.starts_with("#"))
                .filter(|line| !line.contains("EOC"))
                .collect::<Vec<&str>>();

    let image = image
                .join("\n")
                .replace("$thoughts", "\\")
                .replace("$eyes", "oo")
                .replace("$tongue", "  ")
                .replace("\\\\", "\\")
                .replace("\\@", "@");

    println!("{}", image);
}

fn list_files(folder_path: &str) -> Result<(), io::Error> {
    let paths = fs::read_dir(folder_path)?;

    let files = paths
        .filter_map(Result::ok)
        .filter(|path| path.file_type().map(|file_type| file_type.is_file()).unwrap_or(false));

    for file in files {
        println!("   {:#?}", file
                            .path()
                            .file_stem()
                            .unwrap_or_default());
    }
    Ok(())
}

fn get_cow_file(filename: &str, folder_path: &str) -> Result<String, io::Error>{
    let paths = fs::read_dir(folder_path)?;

    let mut files = paths
        .filter_map(Result::ok)
        .filter(|path| path.file_type().map(|file_type| file_type.is_file()).unwrap_or(false));

    let file = files.find(|file| file
                                .path()
                                .file_stem()
                                .unwrap_or_default() == filename);


    if let Some(file) = file {
        if let Ok(contents) = fs::read_to_string(file.path()) {
            return Ok(contents)
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
}

fn main() {

    let matches = App::new("Calfsay")
                    .arg(Arg::with_name("cow-file")
                        .short("f")
                        .help("selects custom cow file")
                        .takes_value(true))
                    .arg(Arg::with_name("no-builtin")
                        .short("b")
                        .help("skips built in cow files"))
                    .arg(Arg::with_name("print-cows")
                        .short("a")
                        .help("Prints out a list of all available cow files (both built in and installed)"))
                    .arg(Arg::with_name("input")
                        .multiple(true)
                        .help("Text to be printed in text box"))
                    .get_matches();

    let desired_file = matches.value_of("cow-file").unwrap_or("default");

    let no_builtin = matches.is_present("no-builtin");

    let env_var = env::var("COWPATH")
        .unwrap_or(String::from("cows:/usr/share/cowsay/cows"));
    let paths = env_var.split(":");

    if matches.is_present("print-cows") {
        let built_in = load_built_in_files();
        println!("Built in files");
        for key in built_in.keys() {
            println!("   {}", key);
        }
        println!("\nFiles");
        for path in paths {
            println!("{}", path);
            if let Err(_) = list_files(&path) {
                println!("   No files found in this location!");
            }
            println!("");
        }
        return;
    }
    
    let text = match matches.values_of("input") {
        Some(text) => text.collect::<Vec<_>>().join(" "),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).expect("I can only read utf-8");
            buffer.trim().to_owned()
        }
    };
    
    for path in paths {
        let cow_file_content = get_cow_file(desired_file, &path);

        if let Ok(contents) = cow_file_content {
            draw_bubble(&text);
            draw_cow_file(&contents);
            return;
        }
    }

    if !no_builtin {
        let built_in_files = load_built_in_files();

        let contents = built_in_files.get(desired_file);

        if let Some(contents) = contents {
            draw_bubble(&text);
            draw_cow_file(&contents);
            return;
        }
    }
    eprintln!("Couldn't find file {}", desired_file);
}
