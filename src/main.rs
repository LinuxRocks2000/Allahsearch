// Search the poetry stored in Okaz (or any similarly-structured project) for strings and print out all matches.
// By Tyler Clarke (weird_pusheen, LinuxRocks2000)
use std::fs;
use regex::Regex;


fn input(prompt: &str) -> String {
    use std::io;
    use std::io::{BufRead, Write};
    print!("{}", prompt);
    io::stdout().flush().expect("Input failed!");
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned()).expect("Input failed!")
}


#[derive(serde::Serialize, serde::Deserialize)]
struct OkazPoem {
    poem_id : String,
    poem_link : String,
    poem_style : String,
    poem_text : String,
    poem_title : String,
    poet_cat : String,
    poet_id : String,
    poet_link : String,
    poet_name : String
}


fn search_dir(dir : &str, query : &Regex) {
    let paths = fs::read_dir(dir).unwrap();
    for file in paths {
        let path = file.unwrap().path();
        println!("Searching {}", path.display());
        let json_reader = fs::File::open(path).expect("Error opening file.");
        let file : Vec<OkazPoem> = serde_json::from_reader(json_reader).expect("Error parsing JSON!");
        for poem in file {
            if query.is_match(&poem.poem_text) {
                println!("{} (id {}) by {} is a match!", poem.poem_title, poem.poem_id, poem.poet_name);
                println!("{}", poem.poem_text);
            }
        }
    }
}


fn main() {
    println!("Welcome to Allahsearch, a tool made by Pusheen for searching the Okaz project for strings.");
    println!("Query strings are in Regex; plain text searching will work, but if you need fancier lookups check out the regex syntax.");
    loop {
        let query = input("Query string ('q' to quit): ");
        if query == "q" {
            break;
        }
        if query.len() < 5 {
            println!("WARNING: You have opted to search for a query string under five characters. This probably will fill your termianl with useless junk.");
            if !(input("Proceed anyways (y/n): ") == "y") {
                println!("Cancelled.");
                continue;
            }
        }
        println!("Searching Okaz for {}...", query);
        let re = regex::Regex::new(&query).expect("Bad Regex syntax!");
        search_dir("abbaci", &re);
        search_dir("islam", &re);
        search_dir("pre_islam", &re);
    }
}
