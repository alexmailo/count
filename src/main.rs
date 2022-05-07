use colored::*;
use std::fs::{self, DirEntry };
fn main() {
    for arg in std::env::args().skip(1) {
        let count = fs::read_dir(&arg)
            .expect("Could not read dir")
            .map(|x| x.unwrap())
            .collect::<Vec<DirEntry>>()
            .len();

        println!(
            "{} has {} files and/or directories.",
            fs::canonicalize(arg)
                .expect("could not get abs path")
                .to_str()
                .expect("could not get str")
                .green(),
            count.to_string().blue()
        );
    }
}
