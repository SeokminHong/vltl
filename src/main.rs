mod converter;
mod fish_parser;
#[cfg(target_os = "macos")]
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
    /// 문자열에 한국어가 포함되어 있는지 확인 (exit code 0: 포함, 1: 미포함)
    HasKorean {
        word: String,
    },
    /// Fish 명령어 줄에서 프로그램 이름을 추출
    ExtractPrograms {
        command_line: String,
    },
    /// 커서 위치가 명령어 이름 위치인지 확인 (exit code 0: 명령어 위치, 1: 아님)
    IsCommandPosition {
        command_line: String,
        cursor: usize,
    },
    /// 대소문자 후보를 고려하여 한국어 입력에 매칭되는 명령어를 찾음 (stdin에서 명령어 목록 읽기)
    FindCommand {
        word: String,
    },
    #[cfg(target_os = "macos")]
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
        Commands::HasKorean { word } => {
            // 한국어가 포함되어 있으면 exit code 0, 아니면 1
            if converter::contains_korean(&word) {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }
        Commands::ExtractPrograms { command_line } => {
            let names = fish_parser::extract_program_names(&command_line);
            for name in names {
                println!("{}", name);
            }
        }
        Commands::IsCommandPosition {
            command_line,
            cursor,
        } => {
            if fish_parser::is_command_position(&command_line, cursor) {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }
        Commands::FindCommand { word } => {
            use std::io::{self, BufRead};
            let stdin = io::stdin();
            let commands: Vec<String> = stdin
                .lock()
                .lines()
                .filter_map(|line| line.ok())
                .filter(|line| !line.is_empty())
                .collect();
            let cmd_refs: Vec<&str> = commands.iter().map(|s| s.as_str()).collect();
            let matches = converter::find_matching_commands(&word, &cmd_refs);
            for m in matches {
                println!("{}", m);
            }
        }
        #[cfg(target_os = "macos")]
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
