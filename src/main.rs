use clap::{Arg, command};
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use reqwest;

#[derive(Debug, Deserialize)]
struct Config {
    themes: Vec<Theme>,
}

#[derive(Debug, Deserialize)]
struct Theme {
    theme: String,
    feeds: Vec<Feed>,
}

#[derive(Debug, Deserialize)]
struct Feed {
    url: String,
    title: String,
}

fn main() {
    let mut file = File::open("feeds.yaml").expect("Failed to open FEEDS.YAML file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read FEEDS.YAML file");

    let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse FEEDS.YAML file");
    let mut theme_options: Vec<&str> = vec![];
    for theme in &config.themes {
        theme_options.push(&theme.theme);
    }
    
    let matches = command!()
        .arg(
            Arg::new("theme")
                .short('t')
                .long("theme")                
            )
        
        .get_matches();
    
    let selected_theme = matches.get_one::<String>("theme").unwrap();

    let theme = config.themes.iter().find(|t| t.theme == *selected_theme);
    if let Some(theme) = theme {
        println!("Selected theme: {}", theme.theme);
        for (index, feed) in theme.feeds.iter().enumerate() {
            println!("{}. {}", index + 1, feed.title);
        }
        let feed_index: usize = read_user_input("Enter feed number: ") - 1;

        if let Some(feed) = theme.feeds.get(feed_index) {
            let client = reqwest::blocking::Client::new();
            let response = client
                .get(&feed.url)
                .send()
                .expect("Failed to send request");

            println!("RSS feed {}", response.text().expect("Failed to parse RSS feed"));            
        } else {
            println!("Feed {} not found", feed_index + 1);
        }
    } else {
        println!("Theme {} not found", selected_theme);
    }
}

fn read_user_input(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}