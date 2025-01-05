use rustyline::{Editor, error::ReadlineError};
use rustyline::history::DefaultHistory;
use dotenv::dotenv;
use std::env;

mod fetch_notice;
mod pomodoro;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let telegram_token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN environment variable not set");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID environment variable not set");

    let mut rl = Editor::<(), DefaultHistory>::new()?; // DefaultHistory 추가
    println!("🔧 Welcome to SSU CLI! Type `help` to see available commands.");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                
                if let Err(err) = rl.add_history_entry(input) {
                  eprintln!("Falied to add to history: {:?}", err);
                }

                match input {
                    "help" => {
                        println!("fetch-notice : 숭실대학교 소프트웨어학부 공지사항을 가져와 telegram으로 전송합니다.");
                        println!("timer        : 공부하는 타이머를 실행합니다.");
                        println!("exit         : 해당 프로젝트를 종료합니다.");
                    }
                    "fetch-notice" => {
                        fetch_notice::fetch_and_send(&telegram_token, &chat_id).await?;
                    }
                    "timer" => {
                        pomodoro::run_pomodoro_timer(25, 5).await;
                    }
                    "exit" => {
                        println!("Exiting the CLI. Goodbye!");
                        break;
                    }
                    _ => {
                        println!("Unknown command: {}. Type `help` for a list of commands.", input);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C detected, exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D detected, exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
