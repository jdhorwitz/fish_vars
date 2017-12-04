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

    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(ref mut file) => {
            for v in arguments.into_iter().skip(1) {
                writeln!(file, "set -gx PATH $PATH {}", v).unwrap();
                println!("Successfully added {} to your path", v);
            }
        }
        Err(err) => {
            panic!("Failed to open file: {}", err);
        }
    }

}
