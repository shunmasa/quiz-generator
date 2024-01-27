use reqwest::blocking;
use serde::Deserialize;
use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Deserialize)]
struct OpenTriviaResponse {
    results: Vec<Question>,
}

#[derive(Debug, Deserialize)]
struct Question {
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

struct QuizOptions {
    num_questions: usize,
    difficulty: String,
    category: usize,
}

impl QuizOptions {
    fn new() -> QuizOptions {
        let num_questions = read_input("Enter the number of questions: ").parse().unwrap_or_default();

        let difficulty = loop {
            let input = read_input("Enter the difficulty (easy, medium, hard): ");
            match input.as_str() {
                "easy" | "medium" | "hard" => break input,
                _ => eprintln!("Error: Invalid difficulty. Choose from easy, medium, or hard."),
            }
        };

        let category = loop {
            println!("Choose a category:");
            println!("9. General Knowledge");
            println!("14. TV");
            println!("10. Books");
            println!("12. Music");
            println!("11. Film");

            let input = read_input("Enter the category number: ").parse();
            match input {
                Ok(category) if [9, 14, 10, 12, 11].contains(&category) => break category,
                _ => eprintln!("Error: Invalid category. Choose a valid category number."),
            }
        };

        QuizOptions {
            num_questions,
            difficulty,
            category,
        }
    }
}

fn main() {
    let options = QuizOptions::new();

    let quiz_api_url = format!(
        "https://opentdb.com/api.php?amount={}&category={}&difficulty={}&type=multiple",
        options.num_questions, options.category, options.difficulty
    );
    let quiz_questions = fetch_quiz_questions(&quiz_api_url);

    run_quiz(quiz_questions);
}

fn fetch_quiz_questions(api_url: &str) -> Vec<Question> {
    let response = blocking::get(api_url).expect("Failed to fetch quiz questions");

    match response.json::<OpenTriviaResponse>() {
        Ok(parsed_response) => parsed_response.results,
        Err(e) => {
            eprintln!("Failed to parse JSON response: {}", e);
            Vec::new()
        }
    }
}

fn run_quiz(questions: Vec<Question>) {
    let mut score = 0;
    let mut correct_answers = 0;
    let mut incorrect_answers = 0;

    for (i, question) in questions.iter().enumerate() {
        println!("Question {}: {}", i + 1, question.question);
        let mut choices = question.incorrect_answers.clone();
        choices.push(question.correct_answer.clone());
        let mut rng = thread_rng();
        choices.shuffle(&mut rng);

        for (j, choice) in choices.iter().enumerate() {
            println!("{}. {}", j + 1, choice);
        }

        print!("Your Answer: ");
        io::stdout().flush().unwrap();

        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).expect("Failed to read line");

        let user_answer = user_answer.trim();

        if let Ok(index) = user_answer.parse::<usize>() {
            if index >= 1 && index <= choices.len() {
                let selected_choice = choices[index - 1].as_str();
                if selected_choice == question.correct_answer {
                    println!("Correct!\n");
                    score += 1;
                    correct_answers += 1;
                } else {
                    println!("Incorrect. The correct answer is: {}\n", question.correct_answer);
                    incorrect_answers += 1;
                }
            } else {
                println!("Invalid choice. Skipping question.\n");
                incorrect_answers += 1;
            }
        } else {
            println!("Invalid input. Skipping question.\n");
            incorrect_answers += 1;
        }
    }

    let total_questions = questions.len() as f64;
    let correct_percentage = (correct_answers as f64 / total_questions) * 100.0;
    let incorrect_percentage = (incorrect_answers as f64 / total_questions) * 100.0;

    println!(
        "Quiz completed. Your score: {}/{} ({}% correct, {}% incorrect)",
        score, total_questions, correct_percentage, incorrect_percentage
    );
}


fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}
