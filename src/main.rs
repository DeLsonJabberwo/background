use std::process::Command;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

fn main() {

    let mut image: String = "".to_string();

    if std::env::args().count() == 1 || std::env::args().nth(1).unwrap() == "current" {
        let current = File::open("/home/delson/scripts/current.txt").unwrap();
        let lines = std::io::BufReader::new(current).lines();
        for line in lines {
            image = line.unwrap();
        }
    } else {
        image = std::env::args().nth(1).unwrap();
    }

    let file = File::open("/home/delson/scripts/bg_key.json").unwrap();
    let images: HashMap<String, String> = serde_json::from_reader(file).unwrap();
    for (key, value) in images.iter() {
        if key == &image {
            image = value.to_string();
        }
    }


    Command::new("feh")
        .arg("--bg-fill")
        .arg("/home/delson/Pictures/Backgrounds/".to_owned() + &image)
        .spawn()
        .expect("feh failed to start"); 

    let mut current = File::create("/home/delson/scripts/current.txt").unwrap();
    current.write_all(image.as_bytes()).unwrap();

}

