use copy_dir::copy_dir;
use glob::glob;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::{
    fs::{File, OpenOptions},
    io::Read,
};

fn modify_shaders() {
    for entry in
        glob("input/assets/vulkanmod/shaders/**/*.vsh").expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                generate_shader_file(path);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    for entry in
        glob("input/assets/vulkanmod/shaders/**/*.fsh").expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                generate_shader_file(path);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Successfully converted the files!");
}

fn generate_shader_file(path: PathBuf) {
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

fn main() {
    modify_shaders();
    add_new_jars();
}

fn add_new_jars() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false) // <--------- this
        .create(false)
        .open("input/fabric.mod.json")
        .unwrap();

    let mut buffer = String::new();
    let _length = file.read_to_string(&mut buffer).unwrap();

    // println!("{}", buffer);

    let mut files = String::from(r#"  "jars": ["#);

    fs::create_dir_all(r#"output/META-INF/jars"#).unwrap();

    for entry in glob("input/META-INF/jars/*.jar").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                files = files
                    + &format!(
                        "
    {{
      \"file\": \"{}\"
    }},",
                        path.file_name().unwrap().to_str().unwrap()
                    );
                let a = copy_dir(
                    path.as_path(),
                    "output/META-INF/jars/".to_string()
                        + path.file_name().unwrap().to_str().unwrap(),
                );
            }
            Err(e) => println!("{:?}", e),
        }
    }

    files = files[0..files.len() - 1].to_string() + "\n  ]";

    let (before_jars, after_start_of_jars) = buffer.split_once(r#"  "jars": ["#).unwrap();
    let (_part_of_jars, after_jars) = after_start_of_jars.split_once("]").unwrap();

    let string = before_jars.to_string() + &files + after_jars;

    // println!("{}", string);

    let mut output = File::create(r#"output/fabric.mod.json"#).unwrap();
    let line = string;
    let _ = write!(output, "{}", line);
}
