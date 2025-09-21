use std::env;
use std::io::{self, Write};

use worlde::{Game, GameStatus}; // ← comes from [lib] name = "worlde"

fn print_legend() {
    println!("Legend:");
    println!("  [A] = Full match (correct letter, correct position)");
    println!("  (A) = Partial match (letter exists, wrong position)");
    println!("  >A< = No match (letter not in the word)");
    println!();
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn main() {
    // Defaults
    let mut alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut target   = String::from("rebus"); // simple demo word

    // Very simple arg parsing:
    //   --word <word>         (sets target word)
    //   --alphabet <alphabet> (sets allowed alphabet)
    // Examples:
    //   cargo run -- --word hello
    //   cargo run -- --alphabet abcdefghijklmnopqrstuvwxyzäöüß --word süß
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i + 1 < args.len() {
        match args[i].as_str() {
            "--word" => {
                target = args[i + 1].clone();
                i += 2;
            }
            "--alphabet" => {
                alphabet = args[i + 1].clone();
                i += 2;
            }
            _ => i += 1,
        }
    }

    println!("=== Worlde (Rust) ===");
    println!("Alphabet: {alphabet}");
    println!("(Hints will follow each guess. You have up to 5 attempts.)\n");
    print_legend();

    let mut game = match Game::new(&alphabet, &target) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Failed to start game: {:?}", e);
            std::process::exit(1);
        }
    };

    println!("{}", game); // prints the initial |_| placeholders
    loop {
        let guess = match read_line("Enter guess: ") {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Failed to read input.");
                continue;
            }
        };

        match game.guess_word(&guess) {
            Ok(word_result) => {
                println!("{}", word_result);
                println!();

                match game.status {
                    GameStatus::Won => {
                        println!("🎉 You won! The word was: {}", game.win_word);
                        println!("\nFull history:\n{}", game);
                        break;
                    }
                    GameStatus::Lost => {
                        println!("💀 You lost. The word was: {}", game.win_word);
                        println!("\nFull history:\n{}", game);
                        break;
                    }
                    GameStatus::InProgress => {
                        println!("Attempts used: {}", game.attempts);
                        println!("{}", game);
                    }
                }
            }
            Err(err) => {
                match err {
                    worlde::GameError::WrongLength { expected, actual } => {
                        println!(
                            "❗ Wrong length: expected {expected}, got {actual}. Try again."
                        );
                    }
                    worlde::GameError::NotInAlphabet(ch) => {
                        if ch == '\0' {
                            println!("❗ Empty guess is not allowed.");
                        } else {
                            println!(
                                "❗ Character '{ch}' is not in the allowed alphabet. Try again."
                            );
                        }
                    }
                    worlde::GameError::GameIsOver(status) => {
                        println!("Game is already over: {:?}", status);
                        break;
                    }
                }
            }
        }
    }
}
