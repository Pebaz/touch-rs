/*
touch-rs

Simplified version of the Unix command line utility: touch.

Create files and their containing folder paths quickly and easily.

Usage:
touch foo.py src/bar.rs ../baz/quz.md.html

1. Creates the file `foo.py` within the current working directory.
2. Creates `bar.rs` within the `src` directory relative to the CWD.
3. Creates `quz.md.html` in the `baz` directory in the parent of the CWD.
*/

use std::env;
use std::path::Path;
use std::fs::{File, create_dir_all};

fn main() {
    if env::args().len() < 2 {
        println!("touch - Usage: touch <filename> ...");
    } else {
        let mut args = env::args().collect::<Vec<String>>();
        args.remove(0);  // Remove executable name from path list

        for arg in args {
            let parent = Path::new(&arg).parent().unwrap();

            if !parent.exists() {
                // Create any necessary parent paths
                create_dir_all(parent).unwrap();
            }

            File::create(&arg).unwrap();
        }
    }
}
