use std::fmt;
#[derive(Debug)]
pub enum GameStatus {
    InProgress,
    Won,
    Lost,
}

#[derive(Debug)]
pub enum GameError {
    NotInAlphabet(char),
    WrongLength { expected: usize, actual: usize },
    GameIsOver(GameStatus),
}

#[derive(Debug)]
pub struct Game {
    pub status: GameStatus,
    pub attempts: u8,
    pub alphabet: String,
    pub win_word: String,
    pub guess_history: Vec<String>,
}

#[derive(Debug)]
pub struct Word {
    pub guessed: String,
    pub guessMatch: Vec<char>,
}
//Помощна функция която проверява дали дадена дума съдържа само символи от дадена азбука и
//връща Ок(true) ако е така и Err(GameError::NotInAlphabet) ако не така.
//Helper function that checks whether a given word contains only symbols from a given alphabet
//and returns Ok(true) if it does, or Err(GameError::NotInAlphabet) if it doesn’t.
pub fn chars_only_in_alpha(alphabet: &str, word: &str) -> Result<bool, GameError> {
    //Ако думата е празна, върнатият char ще е терминираща нула, т.е символ за празен низ
    //If the word is empty, the returned char will be the terminating null, i.e., an empty string symbol
    if word == "" {
        return Err(GameError::NotInAlphabet('\0'));
    }

    let mut found: bool;
    let word_vec = word.chars().collect::<Vec<_>>();
    let alphabet_vec = alphabet.chars().collect::<Vec<_>>();
    let mut ch: &char;

    for word_char in &word_vec {
        found = false;
        ch = word_char;

        for alphabet_char in &alphabet_vec {
            if word_char == alphabet_char {
                found = true;
                break; //ако сме намерили текущият символ от думата в азбуката няма смисъл да итерираме нататък
                       //if we found the current word character in the alphabet, no need to iterate further
            }
        }

        if !found {
            return Err(GameError::NotInAlphabet(*ch));
        }
    }
    Ok(true)
}

impl Game {
    pub fn new(alphabet: &str, word: &str) -> Result<Self, GameError> {
        match chars_only_in_alpha(alphabet, word) {
            Ok(true) => {
                //инициализираме историята на играта (guess_history) с празен опит, т.е с "|_|", колкото е броя на символите на думата на брой
                //initialize the game history (guess_history) with an empty attempt, i.e., "|_|" for each character in the word
                let mut guess_history: Vec<String> = Vec::new();
                let size = word.chars().count();
                let mut empty_guess: String = String::from("");

                for _i in 0..size {
                    empty_guess.push_str("|_|");
                }
                guess_history.push(empty_guess);

                return Ok(Self {
                    status: GameStatus::InProgress,
                    attempts: 0,
                    alphabet: String::from(alphabet),
                    win_word: String::from(word),
                    guess_history: guess_history,
                });
            }
            Err(GameError::NotInAlphabet(char)) => Err(GameError::NotInAlphabet(char)),
            _ => unreachable!(), //panic!("Something went wrong in chars_only_in_alpha() function SPOT1"),
        }
    }
    pub fn guess_word(&mut self, guess: &str) -> Result<Word, GameError> {
        
        //5 е максималният брой опити да се познае някоя дума, т.е ако на 5-тия опит не познаем, губим
        //5 is the maximum number of attempts to guess the word, i.e., if not guessed by the 5th try, we lose
        if self.attempts > 5 {
            self.status = GameStatus::Lost;
        }

        match self.status {
            GameStatus::Won => Err(GameError::GameIsOver(GameStatus::Won)),

            GameStatus::Lost => Err(GameError::GameIsOver(GameStatus::Lost)),

            GameStatus::InProgress => {
                let win_word_len = self.win_word.chars().count();
                let guess_len = guess.chars().count();

                if guess_len != win_word_len {
                    return Err(GameError::WrongLength {
                        expected: win_word_len,
                        actual: guess_len,
                    });
                } else {
                    match chars_only_in_alpha(&self.alphabet, guess) {
                        Err(GameError::NotInAlphabet(char)) => Err(GameError::NotInAlphabet(char)),

                        Ok(true) => {
                            let newWord = Word {
                                guessed: String::from(guess),
                                guessMatch: Word::how_it_matches(
                                    self.win_word.clone(),
                                    String::from(guess),
                                ),
                            };
                            //Добавяме текущият валиден опит в историята на играта
                            //Add the current valid attempt to the game history
                            self.guess_history.push(newWord.to_string());
                            //увеличаваме attemps с едно
                            //increase attempts by one
                            self.attempts += 1;
                            //ако guess съвпада с думата която търсим, играта е спечелена
                            //if the guess matches the target word, the game is won
                            if self.win_word == guess {
                                self.status = GameStatus::Won;
                            } else if self.attempts == 5 {
                                self.status = GameStatus::Lost;
                            }
                            Ok(newWord)
                        }
                        _ => unreachable!(), // panic!("Something went wrong in match chars_only_in_alpha(&self.alphabet,guess) statement"),
                    }
                }
            }
        }
    }
}
impl Word {
    pub fn how_it_matches(word: String, guess: String) -> Vec<char> {
        let vec1 = word.chars().collect::<Vec<_>>();
        let vec1_size = vec1.len(); //vec1 and vec2 should have the same sizes
        let mut vec1_index = 0;

        let vec2 = guess.chars().collect::<Vec<_>>();
        let mut vec2_index = 0;

        let mut result_vec: Vec<char> = Vec::new();
        for _i in 0..vec1_size {
            //Инициализираме резултатния вектор с no match 'n' в началото и после ще попълним пълните и частични съвпадения.
            //Навсякъде където не сме попълнили ще си остане no match 'n'
            //Initialize the result vector with no match 'n' initially, then fill in full and partial matches.
            //Everywhere not filled will remain 'n'.
            result_vec.push('n');
        }

        //вектор за използваните позиции на guess (думата опит)
        //vector for used positions in the guess word
        let mut vec_used_guess: Vec<bool> = Vec::new();
        for _i in 0..vec1_size {
            vec_used_guess.push(false);
        }

        //минаваме по думите и сравняваме за full match 'f' буквите на една и съща позиция тряба да съвпадат.
        //iterate through words and check for full match 'f' (letters at the same position must match)
        while vec2_index < vec1_size {
            if vec1[vec2_index] == vec2[vec2_index] {
                //тогава е full match 'f'
                //then it's a full match 'f'
                result_vec[vec2_index] = 'f';
                vec_used_guess[vec2_index] = true;
            }
            vec2_index += 1;
        }

        vec2_index = 0;

        //сега минаваме по думите и сравняваме за partial match 'p' буквата от едната дума
        //трябва да се съдържа на някоя позиция в другата и да не е ползвана за full match 'f'
        //now iterate to check for partial match 'p': the letter from one word should appear somewhere else
        //in the other word and not already be used for a full match 'f'
        while vec2_index < vec1_size {
            while vec1_index < vec1_size {
                if vec1[vec1_index] == vec2[vec2_index] && !vec_used_guess[vec2_index] {
                    vec_used_guess[vec2_index] = true;
                    result_vec[vec2_index] = 'p';
                    break;
                }
                vec1_index += 1;
            }
            vec1_index = 0;
            vec2_index += 1;
        }

        result_vec
    }
}
impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let guessedWord = self.guessed.chars().collect::<Vec<_>>(); //Вектора със символите на опита
                                                                    //Vector with the characters of the guess attempt
        let size = self.guessMatch.len();
        let mut pos = 0;

        while pos < size {
            if self.guessMatch[pos] == 'p' {
                write!(f, "({})", guessedWord[pos].to_uppercase());
            } else if self.guessMatch[pos] == 'f' {
                write!(f, "[{}]", guessedWord[pos].to_uppercase());
            } else if self.guessMatch[pos] == 'n' {
                write!(f, ">{}<", guessedWord[pos].to_uppercase());
            }
            pos += 1;
        }
        Ok(())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.guess_history.len();
        let mut index = 0;

        while index < size {
            write!(f, "{}", self.guess_history[index]);
            if index < size - 1 {
                write!(f, "{}", '\n');
            }
            index += 1;
        }
        Ok(())
    }
}

//Правим няколко теста за да провелим дари програмата се държи според очакванията.
//Run a few tests to check whether the program behaves as expected
#[test]
fn test_basic() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";

    // Конструираме по два различни начина, just in case -- няма причина да не работи и с двата.
    // Construct in two different ways, just in case — there’s no reason both shouldn’t work.
    assert!(Game::new(english_letters, "!!!").is_err());
    let mut game = Game::new(&String::from(english_letters), "abc").unwrap();

    assert!(matches!(game.status, GameStatus::InProgress));
    assert_eq!(game.attempts, 0);
    assert_eq!(game.to_string(), "|_||_||_|");
    assert_eq!(game.guess_word("abc").unwrap().to_string(), "[A][B][C]");
}

#[test]
fn character_not_in_alphabet_test() {
    let alphabet = "abcdefghijklmnopqrstuvwxy";

    //testing when the word is empty
    assert!(Game::new(alphabet, "").is_err());

    //testing when the word contains characters that are not from the given alphabet
    assert!(Game::new(alphabet, "баница с боза>баница с айрян").is_err()); //cyrillic
    assert!(Game::new(alphabet, "Ich liebe Bier Sprotten und Kartoffeln").is_err()); //German
    assert!(Game::new(alphabet, "5").is_err());
    assert!(Game::new(alphabet, "abc=?").is_err());
    assert!(Game::new(alphabet, "    ").is_err());
}

#[test]
fn wrong_word_len_test() {
    let alphabet = "abcdefghijklmnopqrstuvwxy";
    let mut game = Game::new(&String::from(alphabet), "abc").unwrap();
    assert!(matches!(
        game.guess_word("a"),
        Err(GameError::WrongLength {
            expected: 3,
            actual: 1
        })
    ));
    assert!(matches!(
        game.guess_word("aбc"),
        Err(GameError::NotInAlphabet('б'))
    ));
    //assert!(game.guess_word("a").is_err()); //грешна дължина на думата
    //assert!(game.guess_word("azc").is_err()); //знак не е в азбуката
    //assert!(game.guess_word("a").is_err()); //wrong word length
    //assert!(game.guess_word("azc").is_err()); //character not in alphabet
}

#[test]
fn word_guessing_and_matching_english() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "rebus").unwrap();

    assert_eq!(
        game.guess_word("piers").unwrap().to_string(),
        ">P<>I<(E)(R)[S]"
    );
    assert_eq!(
        game.guess_word("rules").unwrap().to_string(),
        "[R](U)>L<(E)[S]"
    );
    assert_eq!(
        game.guess_word("route").unwrap().to_string(),
        "[R]>O<(U)>T<(E)"
    );
    assert_eq!(
        game.guess_word("rebus").unwrap().to_string(),
        "[R][E][B][U][S]"
    );

    let mut game2 = Game::new(english_letters, "foobar").unwrap();
    assert_eq!(
        game2.guess_word("oopsie").unwrap().to_string(),
        "(O)[O]>P<>S<>I<>E<"
    );
}
#[test]
fn word_guessing_and_matching_cyrillic() {
    let cyrillic_letters = "абвгдежзийклмнопрстуфхцчшщъьюя";
    let mut game2 = Game::new(cyrillic_letters, "котка").unwrap();
    assert_eq!(
        game2.guess_word("лодка").unwrap().to_string(),
        ">Л<[О]>Д<[К][А]"
    );
    assert_eq!(
        game2.guess_word("аакта").unwrap().to_string(),
        "(А)(А)(К)(Т)[А]"
    );
}
#[test]
fn word_guessing_and_matching_german() {
    let german_letters = "abcdefghijklmnopqrstuvwxyzäöüß";
    let mut game2 = Game::new(german_letters, "süß").unwrap();
    assert_eq!(game2.guess_word("füß").unwrap().to_string(), ">F<[Ü][SS]");
}
#[test]
fn game_ending_secenerios() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "rebus").unwrap();

    game.guess_word("ababs");
    assert!(matches!(game.status, GameStatus::InProgress));
    assert_eq!(game.attempts, 1);

    game.guess_word("rebus");
    assert!(matches!(game.status, GameStatus::Won));

    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game2 = Game::new(english_letters, "aaaa").unwrap();
    game2.guess_word("bbbb");
    game2.guess_word("bbbb");
    game2.guess_word("bbbb");
    game2.guess_word("bbbb");
    game2.guess_word("bbbb");
    assert_eq!(game2.attempts, 5);
    assert!(matches!(game2.status, GameStatus::Lost));
}

#[test]
fn display_test_for_game() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "rebus").unwrap();

    game.guess_word("route").unwrap();
    game.guess_word("rebus").unwrap();

    assert_eq!(
        game.to_string(),
        "|_||_||_||_||_|\n[R]>O<(U)>T<(E)\n[R][E][B][U][S]"
    );
}
