use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::env;

fn main() {

    let home = env::var("HOME").unwrap();

    let mut image: String = "".to_string();

    let current_path = home.clone() + "/.background/current.txt";
    if std::env::args().count() == 1 || std::env::args().nth(1).unwrap() == "current" {
        let current = File::open(&current_path).unwrap();
        let lines = std::io::BufReader::new(current).lines();
        for line in lines {
            image = line.unwrap();
        }
    } else {
        image = std::env::args().nth(1).unwrap();
    }

    let conf_file_path = home.clone() + "/.background/conf.json";
    let conf_file = File::open(conf_file_path).unwrap();
    let conf_vals: HashMap<String, String> = serde_json::from_reader(conf_file).unwrap();
    let images_dir = conf_vals.get("images_dir").unwrap().replace("$HOME", &home);
    let images_path = Path::new(&images_dir);
    

//    let file = File::open("/home/delson/scripts/bg_key.json").unwrap();
//    let images: HashMap<String, String> = serde_json::from_reader(file).unwrap();
//    for (key, value) in images.iter() {
//        if key == &image {
//            image = value.to_string();
//        }
//    }
    let image = find_file(&images_path, &image);

    Command::new("feh")
        .arg("--bg-fill")
        .arg(images_dir.to_owned() + &image)
        .spawn()
        .expect("feh failed to start"); 

    let mut current = File::create(current_path).unwrap();
    current.write_all(image.as_bytes()).unwrap();

}

fn find_file(dir: &Path, file: &String) -> String {
    match dir.read_dir() {
        Ok(entries) => {
            for entry in entries {
                if entry.as_ref().unwrap().path().is_dir() {
                    return find_file(&entry.unwrap().path(), file);
                } else {
                    let file_stem = entry
                        .as_ref()
                        .unwrap()
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    let file_name = entry
                        .as_ref()
                        .unwrap()
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    if file == &file_stem || file == &file_name {
                        return file_name;
                    }
                }
            }
        },
        Err(err) => println!("Could not open directory '{}': {}", &dir.to_str().unwrap().to_string(), err),
    }
    return "".to_string();
}

