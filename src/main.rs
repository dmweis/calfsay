use std::fs;
use std::env;
use std::io;
use clap::{Arg, App};

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
                .filter(|line| !line.starts_with("##"))
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
                        .takes_value(true))
                    .arg(Arg::with_name("input")
                        .multiple(true))
                    .get_matches();

    let desired_file = matches.value_of("cow-file").unwrap_or("default");
    let text = matches.values_of("input").unwrap_or_default().collect::<Vec<_>>().join(" ");

    let env_var = env::var("COWPATH")
        .unwrap_or(String::from("cows:/usr/share/cowsay/cows"));
    let paths = env_var.split(":");
    
    for path in paths {
        let cow_file_content = get_cow_file(desired_file, &path);

        if let Ok(contents) = cow_file_content {
            draw_bubble(&text);
            draw_cow_file(&contents);
            return;
        }
    }
    eprintln!("Couldn't find file {}", desired_file);
}
