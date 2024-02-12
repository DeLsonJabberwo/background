use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::fs::read_dir;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::env;
use rand::Rng;

fn main() {

    let home = env::var("HOME").unwrap();

    let mut image: String = "".to_string();

    let conf_file_path = home.clone() + "/.background/conf.json";
    let conf_file = File::open(conf_file_path).unwrap();
    let conf_vals: HashMap<String, String> = serde_json::from_reader(conf_file).unwrap();
    let images_dir = conf_vals.get("images_dir").unwrap().replace("$HOME", &home);
    let images_path = Path::new(&images_dir);

    let current_path = home.clone() + "/.background/current.txt";
    if std::env::args().count() == 1 || std::env::args().nth(1).unwrap() == "current" || std::env::args().nth(1).unwrap() == "-c" {
        let current = File::open(&current_path).unwrap();
        let lines = std::io::BufReader::new(current).lines();
        for line in lines {
            image = line.unwrap();
        }
    } else if std::env::args().nth(1).unwrap() == "--list" || std::env::args().nth(1).unwrap() == "--ls" {
        println!("");
        let mut paths: Vec<_> = read_dir(&images_dir).unwrap()
                                                        .map(|r| r.unwrap())
                                                        .collect();
        paths.sort_by_key(|dir| dir.path());
        for path in paths {
            let file_stem: String = path.path().file_stem().unwrap().to_str().unwrap().to_string();
            let file_name: String = "[".to_string() + path.file_name().to_str().unwrap() + "]";
            println!("{0: <25} {1: <35}", file_stem, file_name);
        }
        println!("");
        image = set_current();
    } else if std::env::args().nth(1).unwrap() == "--random" || std::env::args().nth(1).unwrap() == "--rand" || std::env::args().nth(1).unwrap() == "-r" {
        println!("");
        let paths: Vec<_> = read_dir(&images_dir).unwrap()
                                                        .map(|r| r.unwrap())
                                                        .collect();
        let image_count = paths.len();
        let random = rand::thread_rng().gen_range(0..image_count);
        image = paths[random].path().file_stem().unwrap().to_str().unwrap().to_string();
        println!("Randomly selected image: {}", image);
        println!("");
    } else {
        image = std::env::args().nth(1).unwrap();
    }

    

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

// I don't love this, but it works to fix the error and it's probably good enough for now.
// Might want to make it a bit more universal or portable later, though.
fn set_current() -> String {
    let home = env::var("HOME").unwrap();
    let current_path = home.clone() + "/.background/current.txt";
    let mut image: String = "".to_string();

    let current = File::open(&current_path).unwrap();
    let lines = std::io::BufReader::new(current).lines();
    for line in lines {
        image = line.unwrap();
    }
    return image;
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

