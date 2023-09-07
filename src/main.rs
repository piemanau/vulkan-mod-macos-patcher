use copy_dir::copy_dir;
use glob::glob;
use std::error::Error;
use std::{fs, env, result};
use std::io::Write;
use std::path::PathBuf;
use std::{
    fs::{File, OpenOptions},
    io::Read,
};

fn modify_shaders(input: &String, output: &String) {
    for entry in
        glob(&(input.to_owned() + "assets/vulkanmod/shaders/**/*.vsh")).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                generate_shader_file(path, input, output);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    for entry in
        glob(&(input.to_owned() + "assets/vulkanmod/shaders/**/*.fsh")).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                generate_shader_file(path, input, output);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Successfully converted the files!");
}

fn generate_shader_file(path: PathBuf, input: &String, output: &String) {
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

        let inner_folder = path.as_path().to_str().unwrap().rsplit_once(input.rsplit_once("/").unwrap().1).unwrap().1;
        fs::create_dir_all(&inner_folder.rsplit_once("/").unwrap().0).unwrap();
        let mut output = File::create(path).unwrap();
        let line = buffer;
        let _ = write!(output, "{}", line);
    }
}

#[derive(Debug)]
enum CustomError {
    IncorrectArguments,
}

fn main() -> Result<(), CustomError>{
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: <EXEC> <INPUT> <OUTPUT>");
        return Err(CustomError::IncorrectArguments);
    }
    
    let input = &args[1];
    let output = &args[2];

    modify_shaders(input, output);
    add_new_jars(input, output);
    Ok(())
}

fn add_new_jars(input: &String, output: &String) {

    fs::create_dir_all(output.to_owned() + r#"/META-INF/jars"#).unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(input.to_owned() + "/fabric.mod.json")
        .unwrap();

    let mut buffer = String::new();
    let _length = file.read_to_string(&mut buffer).unwrap();

    // println!("{}", buffer);

    let mut files = String::from(r#"  "jars": ["#);

    fs::create_dir_all(output.to_owned() + r#"/META-INF/jars"#).unwrap();

    for entry in glob(&(input.to_owned() + "/META-INF/jars/*.jar")).expect("Failed to read glob pattern") {
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
                    output.to_owned() + "/META-INF/jars/"
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

    let mut output = File::create(output.to_owned() + r#"/fabric.mod.json"#).unwrap();
    let line = string;
    let _ = write!(output, "{}", line);
}
