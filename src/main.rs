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
    /// 한 번의 호출로 한글 감지, 명령어 위치 판별, 두벌식 변환 수행
    Resolve {
        token: String,
        command_line: String,
        cursor: usize,
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
        Commands::Resolve {
            token,
            command_line,
            cursor,
        } => {
            if let Some(converted) = resolve_token(&token, &command_line, cursor) {
                println!("{converted}");
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

fn resolve_token(token: &str, command_line: &str, cursor: usize) -> Option<String> {
    if !converter::contains_korean(token) || !fish_parser::is_command_position(command_line, cursor)
    {
        return None;
    }

    let converted = converter::convert_korean_to_english(token);
    (converted != token).then_some(converted)
}

#[cfg(test)]
mod tests {
    use super::resolve_token;

    #[test]
    fn resolves_korean_command_in_one_step() {
        assert_eq!(resolve_token("햣", "햣 status", 1).as_deref(), Some("git"));
    }

    #[test]
    fn ignores_arguments_and_ascii_tokens() {
        assert_eq!(resolve_token("ㅎ", "echo ㅎ", 6), None);
        assert_eq!(resolve_token("git", "git status", 3), None);
    }
}
