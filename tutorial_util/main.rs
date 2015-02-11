#![feature(core)]

extern crate md_rel;

use md_rel::transform_file;

fn main() {
    let markdown_files = ["getting-started/readme.dev.md"];
    for &file in markdown_files.iter() {
        match transform_file(file) {
            Ok(_) => (),
            Err(x) => println!(
                "Failed to compile file {} with error {:?}", file, x)
        }
    }
}
