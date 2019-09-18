use std::env;
use std::fs::{File, create_dir_all};

fn main() {
    if env::args().len() < 2 {
        println!("touch - Usage: touch <filename>");
    } else {
        let mut args = env::args().collect::<Vec<String>>();
        args.remove(0);
        for arg in args {
            create_dir_all(arg.clone()).unwrap();
        }
    }
}
