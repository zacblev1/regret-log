use chrono::Local;
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, fs::OpenOptions, io::Write, path::PathBuf};

/// regret-log: a quiet CLI app to record and review regrets
#[derive(Parser)]
#[command(name = "regret-log")]
#[command(about = "Log and review regrets in your terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Log a new regret
    Now,
    /// Review recent regrets
    Review,
    /// Show stats (tags and mood frequency)
    Stats,
}

#[derive(Serialize, Deserialize, Debug)]
struct RegretEntry {
    timestamp: String,
    text: String,
    tags: Vec<String>,
    mood: Option<u8>,
}

fn get_log_path() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".regret-log");
    fs::create_dir_all(&path).expect("Failed to create ~/.regret-log directory");
    path.push("log.yaml");
    path
}

fn load_log() -> Vec<RegretEntry> {
    let path = get_log_path();
    if !path.exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(path).unwrap_or_default();
    serde_yaml::from_str(&content).unwrap_or_else(|_| Vec::new())
}

fn save_log(log: &[RegretEntry]) {
    let path = get_log_path();
    let yaml = serde_yaml::to_string(log).expect("Failed to serialize log");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open log file");
    file.write_all(yaml.as_bytes())
        .expect("Failed to write log file");
}

fn command_now() {
    let mut log = load_log();

    let text: String = Input::new()
        .with_prompt("What happened?")
        .interact_text()
        .unwrap();

    let tags_input: String = Input::new()
        .with_prompt("Optional tags (comma-separated)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let mood: Option<u8> = {
        let moods = vec!["1 (Terrible)", "2", "3", "4", "5 (Neutral)", "Skip"];
        let selection = Select::new()
            .with_prompt("Mood?")
            .default(5)
            .items(&moods)
            .interact()
            .unwrap();
        if selection < 5 {
            Some((selection + 1) as u8)
        } else {
            None
        }
    };

    let entry = RegretEntry {
        timestamp: Local::now().to_rfc3339(),
        text,
        tags: tags_input
            .split(',')
            .map(|t| t.trim().to_lowercase())
            .filter(|t| !t.is_empty())
            .collect(),
        mood,
    };

    log.push(entry);
    save_log(&log);
    println!("✅ Logged.");
}

fn command_review() {
    let log = load_log();
    if log.is_empty() {
        println!("No regrets logged yet.");
        return;
    }

    for entry in log.iter().rev().take(5) {
        println!(
            "[{}] {}\n  tags: {:?}  mood: {}\n",
            entry.timestamp,
            entry.text,
            entry.tags,
            entry
                .mood
                .map(|m| m.to_string())
                .unwrap_or_else(|| "-".to_string())
        );
    }
}

fn command_stats() {
    let log = load_log();
    if log.is_empty() {
        println!("No regrets to analyze.");
        return;
    }

    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    let mut mood_counts: HashMap<u8, usize> = HashMap::new();

    for entry in &log {
        for tag in &entry.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
        if let Some(mood) = entry.mood {
            *mood_counts.entry(mood).or_insert(0) += 1;
        }
    }

    println!("Top Tags:");
    for (tag, count) in tag_counts.iter().take(10) {
        println!("  {}: {}", tag, count);
    }

    println!("\nMood Frequency:");
    for mood in 1..=5 {
        let count = mood_counts.get(&mood).unwrap_or(&0);
        println!("  {}: {}", mood, count);
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Now => command_now(),
        Commands::Review => command_review(),
        Commands::Stats => command_stats(),
    }
}