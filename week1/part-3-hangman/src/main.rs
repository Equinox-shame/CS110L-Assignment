// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn crate_current_char<'a>(secret_chars: &Vec<char>, current_chars:&'a mut String, char:char) ->  &'a String{
    let mut index = 0;
    let mut flag = false; // 是否已经猜过该字符

    // 找到对应的字符位置信息
    for i in secret_chars{
        if *i == char{
            flag = true;
            // 替换对应位置的字符
            current_chars.replace_range(index..index+1, &char.to_string());  
        }
        index += 1;
    }
    // 如果猜错该字符，直接返回
    if !flag{
        return current_chars
    }
    current_chars
}


fn main() {
    // 生成随机单词
    let secret_word = pick_a_random_word();
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    
    let mut guess_times_left = NUM_INCORRECT_GUESSES;
    let mut guessed_letters: String = String::new();
    let mut current_chars = "-".repeat(secret_word_chars.len());

    // println!("random word: {}", secret_word); // 用于测试
    println!("Welcom to CS 110L Game Hangman!");
    println!("The word so far is {}", current_chars);
    println!("You have guessed the following letters: ");
    println!("You have {} guesses left", guess_times_left);

    while guess_times_left > 0{
        print!("Please guess a letter: ");
        // 读取用户输入
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
            
        let char_guess = guess.chars().next().unwrap();
        guessed_letters.push_str(&guess.trim());

        if !secret_word_chars.contains(&char_guess){
            println!("Sorry, that letter is not in the word.");
            println!();
            guess_times_left -= 1;
        }

        println!("The word so far is {}", crate_current_char(&secret_word_chars, &mut current_chars, char_guess));
        println!("You have guessed the following letters: {:?}", guessed_letters);
        println!("You have {} guesses left", guess_times_left);
        println!();

        if current_chars.eq(&secret_word) && guess_times_left > 0{
            println!("\nCongratulations, you won!");
            return;
        }
    }
        
    println!("Sorry, you ran out of guesses!");
    println!("The word was: {}", secret_word);
}
