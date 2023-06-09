#[allow(unused_variables)]

use clap::{arg, command};
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use reqwest;

#[derive(Clone)]
#[derive(Debug, Deserialize)]
struct Config {
    themes: Vec<Theme>,
}

#[derive(Clone)]
#[derive(Debug, Deserialize)]
struct Theme {
    theme: String,
    feeds: Vec<Feed>,
}

#[derive(Clone)]
#[derive(Debug, Deserialize)]
struct Feed {
    url: String,
    title: String,
}

fn main() {
    let mut file = File::open("feeds.yaml").expect("Failed to open FEEDS.YAML file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read FEEDS.YAML file");

    let config: Config = serde_yaml::from_str(&contents)
        .expect("Failed to parse FEEDS.YAML file");

    let theme_options: Vec<String> = config
        .themes
        .into_iter()
        .map(|theme| theme.theme)
        .collect();

    let help_message = format!(
        "Themes to use: {}", theme_options.join(", ")
    );

    let matches = command!()
        .arg(
            arg!(-t --theme <theme>)
                .help(help_message)
                .required(true),
        )
        .get_matches();

    let selected_theme  = matches.get_one::<String>("theme").expect("Select a proper theme from existing list. Use help for more info.");
    println!("Selected theme: {}", selected_theme );

    let selected_theme_clone = selected_theme.clone();
    if let Some(theme) = config.themes.iter().find(|t| &t.theme == selected_theme.as_str()) {
        println!("Selected theme: {}", theme.theme);
        for (index, feed) in theme.feeds.iter().enumerate() {
            println!("{}. {}", index + 1, feed.title);
        }
    
        if let Some(feed) = theme.feeds.get(0) {
            let client = reqwest::blocking::Client::new();
            let response = client
                .get(&feed.url)
                .send()
                .expect("Failed to send request");
    
            println!("RSS feed {}", response.text().expect("Failed to parse RSS feed"));
        } else {
            println!("No feeds found for the selected theme");
        }
    } else {
        println!("Theme {} not found", selected_theme);
    }
}

