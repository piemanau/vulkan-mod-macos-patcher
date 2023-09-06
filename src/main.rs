use glob::glob;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::{
    fs::{File, OpenOptions},
    io::Read,
};

fn main() {
    for entry in glob("input/**/*.vsh").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                generate_file(path);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    for entry in glob("input/**/*.fsh").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                generate_file(path);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Successfully converted the files!");
}

fn generate_file(path: PathBuf) {
    if path.is_file() {
        let mut file = OpenOptions::new()
            .read(true)
            .write(false) // <--------- this
            .create(false)
            .open(&path)
            .unwrap();

        let mut buffer = String::new();
        let _length = file.read_to_string(&mut buffer).unwrap();
        buffer = buffer.replace("main", "main0");

        if path.file_name().unwrap() != "terrain.vsh" {
            buffer = buffer
                + "
void main() {
    main0();
}";
        } else {
            buffer = buffer
                + "
void main() {
}";
        }

        // println!("{}", buffer);

        let path = "output/".to_string()
            + path
                .as_path()
                .to_str()
                .unwrap()
                .split("input")
                .last()
                .unwrap();
        fs::create_dir_all(&path.rsplit_once("/").unwrap().0).unwrap();
        let mut output = File::create(path).unwrap();
        let line = buffer;
        let _ = write!(output, "{}", line);
    }
}
