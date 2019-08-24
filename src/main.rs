use std::fs;

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

    let paths = fs::read_dir("/usr/share/cowsay/cows").unwrap();

    let mut files = paths
        .filter_map(Result::ok)
        .filter(|path| path.file_type().unwrap().is_file());

    let file = files.find(|file| file
                                .path()
                                .file_stem()
                                .unwrap() == desired_file);

    let contents = fs::read_to_string(file.unwrap().path()).unwrap();

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

    draw_bubble(&text);
    println!("{}", image);
}
