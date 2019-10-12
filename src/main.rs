use std::env;
use std::path::Path;
use std::fs::{File, create_dir_all};

fn main() {
    if env::args().len() < 2 {
        println!("touch - Usage: touch <filename>");
    } else {
        let mut args = env::args().collect::<Vec<String>>();

        // Remove executable name
        args.remove(0);

        for arg in args {
            let parent = Path::new(&arg).parent().unwrap();

            // Path is CWD
            if parent.to_str().unwrap() == String::from("") {
                File::create(arg).unwrap();
            }

            // Path is a folder path
            else if !parent.exists() {
                create_dir_all(parent).unwrap();
                File::create(arg).unwrap();
            }
        }
    }
}
