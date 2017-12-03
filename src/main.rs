use std::env;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    let mut path: PathBuf = match env::home_dir() {
        Some(ref p) => PathBuf::from(p),
        None => PathBuf::default(),
    };

    path.push(".config/fish/config.fish");

    let arguments: Vec<String> = env::args().collect();

    let variable = format!("{}", &arguments[1]);

    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(ref mut file) => {
            writeln!(file, "set -gx PATH $PATH {}", variable).unwrap();
        }
        Err(err) => {
            panic!("Failed to open file: {}", err);
        }
    }

}
