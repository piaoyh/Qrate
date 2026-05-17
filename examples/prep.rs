use std::io;
use std::num::ParseIntError;

use qrate::{ SQLiteDB, QBDB, Generator };

fn main()
{
    let db = SQLiteDB::open("./Information_Security".to_string()).expect("Failed to open database. Make sure 'Information_Security.qbdb' exists.");
    let qb = db.read_qbank().expect("Error: Could not read QBank from database. Ensure it's not empty or corrupted.");
    let last = qb.get_questions().len() as u16;
    if last == 0
    {
        println!("Error: The QBank is empty. No questions to display.");
        return;
    }
    let mut generator = Generator::new_one_set(&qb, 1, last, last as usize).expect("Failed to create generator for QBank.");
    exam(&mut generator);
}

pub fn exam(generator: &mut Generator)
{
    // let categories = generator.origin.get_header().get_categories(); // 'cat' is now provided by generator.next()
    let mut score: i8 = 0;
    let note = generator.get_notice();
    println!("{}", note);

    while let Some((question_number, _, cat, question_text, choices)) = generator.next()
    {
        println!("{}. [{}]   {}", question_number, cat, question_text);

        let mut correct_answers_count = 0;
        let mut correct_answer_numbers = Vec::new(); // Store 1-based indices of correct choices
        let max_choice = choices.len(); // Maximum valid choice number

        for (choice_number, choice) in choices.iter().enumerate()
        {
            let current_choice_num = choice_number + 1;
            println!("\t{}) {}", current_choice_num, choice.0);
            if choice.1
            {
                correct_answers_count += 1;
                correct_answer_numbers.push(current_choice_num as u8);
            }
        }
        println!();

        // Get user's answers using the helper function
        let mut user_answers = get_user_answers(correct_answers_count, max_choice);
        user_answers.sort_unstable(); // Sort user's answers for easy comparison
        correct_answer_numbers.sort_unstable(); // Sort correct answers for easy comparison

        // Calculate score
        if correct_answers_count == 1
        {
            if user_answers[0] == correct_answer_numbers[0]
            {
                println!("Correct!");
                score += 3;
            }
            else
            {
                println!("Incorrect!");
                score -= 1;
            }
            println!("The answer is {}.", correct_answer_numbers[0]);
        }
        else // multiple answers
        {
            if user_answers == correct_answer_numbers
            {
                println!("All answers are Correct!");
                score += 3;
            }
            else
            {
                let mut correct_matches = 0;
                for ans in user_answers.iter()
                {
                    if correct_answer_numbers.contains(ans)
                        { correct_matches += 1; }
                }

                if correct_matches == 0
                {
                    println!("All answers are Incorrect!");
                    score -= 3;
                }
                else // correct_matches == 1
                {
                    println!("One answer is Correct but the other answer is Incorrect!");
                    score += 0;
                }
            }
            // For multiple answers, display all correct answers.
            print!("The answers are ");
            for (i, ans) in correct_answer_numbers.iter().enumerate()
            {
                if i > 0
                    { print!(", "); }
                print!("{}", ans);
            }
            println!(".");
        }

        println!("Your score is {} points at the moment!", score);
        println!("\n-------------------------------------\n");
    }
    println!("You've got {} points!", score);
}


// Helper function to get user's answers
fn get_user_answers(expected_count: usize, max_choice: usize) -> Vec<u8>
{
    use std::io::Write; // Import Write trait for flush

    loop
    {
        let mut input = String::new();
        if expected_count == 1
            { print!("Enter your answer (1-{}): ", max_choice); }
        else
            { print!("Enter {} answers separated by space (1-{}): ", expected_count, max_choice); }
        
        // Ensure the prompt is displayed before reading input
        io::stdout().flush().expect("flush failed!");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        let parsed_answers: Result<Vec<u8>, ParseIntError> = input.split_whitespace().map(|s| s.parse::<u8>()).collect();
        match parsed_answers
        {
            Ok(mut answers) =>
            {
                if answers.len() != expected_count
                {
                    println!("Error: Please enter exactly {} answers.", expected_count);
                    continue;
                }

                // Check if all answers are within valid range and unique
                let mut invalid_input = false;
                for ans in answers.iter()
                {
                    if *ans == 0 || (*ans as usize) > max_choice
                    {
                        println!("Error: Answer '{}' is out of valid range (1-{}).", ans, max_choice);
                        invalid_input = true;
                        break;
                    }
                }
                if invalid_input
                    { continue; }

                // Check for duplicate answers if multiple answers are expected
                if expected_count > 1
                {
                    answers.sort_unstable(); // Sort to easily check for duplicates
                    for i in 0..(answers.len() - 1)
                    {
                        if answers[i] == answers[i+1]
                        {
                            println!("Error: Duplicate answers are not allowed.");
                            invalid_input = true;
                            break;
                        }
                    }
                }
                if invalid_input
                    { continue; }
                return answers;
            },
            Err(_) =>
            {
                println!("Error: Invalid input. Please enter numbers separated by spaces.");
                continue;
            }
        }
    }
}