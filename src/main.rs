#![cfg(target_os = "macos")]

mod converter;
mod ime;

use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(version, name = "vltl")]
#[command(about = "한국어 IME로 잘못 입력된 명령어를 영어로 변환하고 IME를 영어로 전환하는 도구")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    /// 한국어로 입력된 명령어를 영어로 변환
    Convert {
        word: String,
    },
    /// IME를 영어로 전환
    SwitchToEnglish,
}

const INIT_STR: &str = include_str!("../init.fish");

// CLI 진입점
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("{}", INIT_STR);
        }
        Commands::Convert { word } => {
            let converted = converter::convert_korean_to_english(&word);
            println!("{}", converted);
        }
        Commands::SwitchToEnglish => {
            match ime::switch_to_english() {
                Ok(()) => {
                    // 성공적으로 전환됨
                }
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }
    }
}
