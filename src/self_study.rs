// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::{ Choices, QBank, SBank, Shuffler, Student };

/// Represents the scoring rules for self-study.
/// - `NegativeMarkingPartialCredit`: Applies negative marking with partial
///   credit for multiple-choice questions.
/// - `NegativeMarkingNoPartialCredit`: Applies negative marking without
///   partial credit for multiple-choice questions.
/// - `NoNegativeMarkingPartialCredit`: No negative marking, but partial credit
///   is awarded for multiple-choice questions.
/// - `NoNegativeMarkingNoPartialCredit`: No negative marking and no partial
///   credit for multiple-choice questions.
/// For single-choice questions, negative marking is applied if the selected
/// answer is incorrect, regardless of the scoring rule.
/// For short-answer questions, only full credit is awarded for correct answers,
/// and no negative marking is applied.
/// The scoring rules are applied as follows:
/// - For single-choice questions:
///   - If the selected answer is correct, full points are awarded.
///   - If the selected answer is incorrect, negative marking is applied based
///     on the number of choices.
/// - For multiple-choice questions:
///   - If all selected answers are correct, full points are awarded.
///   - If some selected answers are correct and some are incorrect, the score
///     is calculated based on the number of correct and incorrect selections,
///     and the scoring rule is applied accordingly.
///   - If  all selected answers are incorrect, negative marking is applied
///     based on the scoring rule.
/// - For short-answer questions:
///   - If the answer matches any of the correct answers (ignoring
///     leading/trailing whitespace and case), full points are awarded.
///   - If the answer does not match any of the correct answers, no points are
///     awarded and no negative marking is applied.
/// The scoring rules are designed to encourage careful selection of answers and
/// to penalize random guessing, while also allowing for partial credit in
/// multiple-choice questions when applicable.  
/// The `SelfStudy` struct manages the self-study session, including question
/// shuffling, user answers, and scoring based on the defined rules.
/// The `score` method calculates the total score for the self-study session
/// based on the user's answers and the defined scoring rules, ensuring that
/// the final score is between 0 and 100, rounded to two decimal places.
/// 
/// Example usage:
/// ```
/// let mut self_study = SelfStudy::new(&qbank, 1, 100, 20, seeds).unwrap();
/// self_study.set_scoring_rule(ScoringRule::NegativeMarkingPartialCredit);
/// // User answers questions...
/// let total_score = self_study.score();
/// println!("Total Score: {}", total_score);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScoringRule
{
    /// Applies negative marking with partial credit for multiple-choice questions.
    /// Applies negative marking without partial credit for multiple-choice questions.
    /// No negative marking, but partial credit is awarded for multiple-choice questions.
    /// No negative marking and no partial credit for multiple-choice questions.
    /// For single-choice questions, negative marking is applied if the selected answer is incorrect, regardless of the scoring rule.
    /// For short-answer questions, only full credit is awarded for correct answers, and no negative marking is applied.
    /// The scoring rules are applied as follows:
    /// - For single-choice questions:
    ///   - If the selected answer is correct, full points are awarded.
    ///   - If the selected answer is incorrect, negative marking is applied based on the number of choices.
    /// - For multiple-choice questions:
    ///   - If all selected answers are correct, full points are awarded.
    ///   - If some selected answers are correct and some are incorrect, the score is calculated based on the number of correct and incorrect selections, and the scoring rule is applied accordingly.
    ///   - If  all selected answers are incorrect, negative marking is applied based on the scoring rule.
    /// - For short-answer questions:
    ///   - If the answer matches any of the correct answers (ignoring leading/trailing whitespace and case), full points are awarded.  
    ///   - If the answer does not match any of the correct answers, no points are awarded and no negative marking is applied.
    NegativeMarkingPartialCredit,

    /// Applies negative marking without partial credit for multiple-choice questions.
    /// Applies negative marking with partial credit for multiple-choice questions.
    /// No negative marking, but partial credit is awarded for multiple-choice questions.
    /// No negative marking and no partial credit for multiple-choice questions.
    /// For single-choice questions, negative marking is applied if the selected answer is incorrect, regardless of the scoring rule.
    /// For short-answer questions, only full credit is awarded for correct answers, and no negative marking is applied.
    /// The scoring rules are applied as follows:
    /// - For single-choice questions:
    ///   - If the selected answer is correct, full points are awarded.
    ///   - If the selected answer is incorrect, negative marking is applied based on the number of choices.
    /// - For multiple-choice questions:
    ///   - If all selected answers are correct, full points are awarded.
    ///   - If some selected answers are correct and some are incorrect, the score is calculated based on the number of correct and incorrect selections, and the scoring rule is applied accordingly.
    ///   - If  all selected answers are incorrect, negative marking is applied based on the scoring rule.
    /// - For short-answer questions:    
    ///   - If the answer matches any of the correct answers (ignoring leading/trailing whitespace and case), full points are awarded.  
    ///   - If the answer does not match any of the correct answers, no points are awarded and no negative marking is applied.
    NegativeMarkingNoPartialCredit,

    /// No negative marking, but partial credit is awarded for multiple-choice questions.
    /// Applies negative marking with partial credit for multiple-choice questions.
    /// Applies negative marking without partial credit for multiple-choice questions.
    /// No negative marking and no partial credit for multiple-choice questions.
    /// For single-choice questions, negative marking is applied if the selected answer is incorrect, regardless of the scoring rule.
    /// For short-answer questions, only full credit is awarded for correct answers, and no negative marking is applied.
    /// The scoring rules are applied as follows:
    /// - For single-choice questions:
    ///   - If the selected answer is correct, full points are awarded.
    ///   - If the selected answer is incorrect, negative marking is applied based on the number of choices.
    /// - For multiple-choice questions:
    ///   - If all selected answers are correct, full points are awarded.
    ///   - If some selected answers are correct and some are incorrect, the score is calculated based on the number of correct and incorrect selections, and the scoring rule is applied accordingly.
    ///   - If  all selected answers are incorrect, negative marking is applied based on the scoring rule.
    /// - For short-answer questions:    
    ///   - If the answer matches any of the correct answers (ignoring leading/trailing whitespace and case), full points are awarded.  
    ///   - If the answer does not match any of the correct answers, no points are awarded and no negative marking is applied.
    NoNegativeMarkingPartialCredit,

    /// No negative marking and no partial credit for multiple-choice questions.
    /// Applies negative marking with partial credit for multiple-choice questions.
    /// Applies negative marking without partial credit for multiple-choice questions.
    /// No negative marking, but partial credit is awarded for multiple-choice questions.
    /// For single-choice questions, negative marking is applied if the selected answer is incorrect, regardless of the scoring rule.
    /// For short-answer questions, only full credit is awarded for correct answers, and no negative marking is applied.
    /// The scoring rules are applied as follows:
    /// - For single-choice questions:
    ///   - If the selected answer is correct, full points are awarded.
    ///   - If the selected answer is incorrect, negative marking is applied based on the number of choices.
    /// - For multiple-choice questions:
    ///   - If all selected answers are correct, full points are awarded.
    ///   - If some selected answers are correct and some are incorrect, the score is calculated based on the number of correct and incorrect selections, and the scoring rule is applied accordingly.
    ///   - If  all selected answers are incorrect, negative marking is applied based on the scoring rule.
    /// - For short-answer questions:    
    ///   - If the answer matches any of the correct answers (ignoring leading/trailing whitespace and case), full points are awarded.  
    ///   - If the answer does not match any of the correct answers, no points are awarded and no negative marking is applied.
    NoNegativeMarkingNoPartialCredit,
}

impl ScoringRule
{
    // pub fn from_str(s: &str) -> Self
    /// Creates a `ScoringRule` from a string representation.
    /// 
    /// # Arguments
    /// * `s` - A string slice representing the scoring rule.
    /// 
    /// # Features
    /// The input string should be one of the following:
    /// - "negative-marking-partial-credit"
    /// - "negative-marking-no-partial-credit"
    /// - "no-negative-marking-partial-credit"
    /// - "no-negative-marking-no-partial-credit"
    /// 
    /// # Returns
    /// A `ScoringRule` enum variant corresponding to the input string.
    /// If the input string does not match any of the expected values,
    /// it defaults to `NoNegativeMarkingPartialCredit
    /// 
    /// # Examples
    /// ``` 
    /// let rule = ScoringRule::from_str("negative-marking-partial-credit");
    /// assert_eq!(rule, ScoringRule::NegativeMarkingPartialCredit);
    /// ```
    #[inline]
    pub fn from_str(s: &str) -> Self
    {
        match s
        {
            "negative-marking-partial-credit" => Self::NegativeMarkingPartialCredit,
            "negative-marking-no-partial-credit" => Self::NegativeMarkingNoPartialCredit,
            "no-negative-marking-partial-credit" => Self::NoNegativeMarkingPartialCredit,
            "no-negative-marking-no-partial-credit" => Self::NoNegativeMarkingNoPartialCredit,
            _ => Self::NoNegativeMarkingPartialCredit,
        }
    }
}


/// Represents a user's answer to a question.
/// - `Choices(Vec<bool>)`: Represents the user's selected choices for
///   multiple-choice questions, where each boolean value indicates whether
///   a particular choice is selected.
/// - `ShortAnswer(String)`: Represents the user's answer for short-answer
///   questions as a string.
/// - `None`: Represents the absence of an answer, indicating that the user
///   has not provided an answer for the question.
#[derive(Debug, Clone, PartialEq)]
pub enum UserAnswer
{
    Choices(Vec<bool>),
    ShortAnswer(String),
    None,
}

/// Manages self-study sessions, including question shuffling and scoring.
/// The `SelfStudy` struct is responsible for managing the self-study session,
/// including question shuffling, user answers, and scoring based on the
/// defined rules.
pub struct SelfStudy
{
    shuffler: Shuffler,
    current_question_number: u16,
    user_answers: Vec<UserAnswer>,
    scoring_rule: ScoringRule,
}

impl SelfStudy
{
    const SINGLE_RESPONSE_MULTIPLE_CHOICE: u8 =  1; // single-response multiple-choice
    const MULTIPLE_RESPONSE_MULTIPLE_CHOICE: u8 = 2; // multiple-response multiple-choice
    const SHORT_ANSWER_SUBJECTIVE: u8 = 3; // short-answer subjective

    // pub fn new(qbank: &QBank, start: u16, end: u16, number_of_questions: usize, seeds: [u64; 16]) -> Option<Self>
    /// Creates a new `SelfStudy` instance.
    ///
    /// # Arguments
    /// * `qbank` - The question bank.
    /// * `start` - Starting question index (1-based).
    /// * `end` - Ending question index (1-based).
    /// * `number_of_questions` - Number of questions to select.
    /// * `seeds` - Random seeds.
    /// 
    /// # Returns
    /// An `Option<Self>` which is `Some(SelfStudy)` if the shuffling and exam
    /// generation are successful,
    /// or `None` if there was an error during the process.
    /// 
    /// # Examples
    /// ```
    /// let self_study = SelfStudy::new(&qbank, 1, 100, 20, seeds).unwrap();
    /// ```
    pub fn new(qbank: &QBank, start: u16, end: u16, number_of_questions: usize, seeds: [u64; 16]) -> Option<Self>
    {
        let student = Student::new("Self Study".to_string(), "-".to_string());
        let sbank = SBank::new_with_students(vec![student]);
        let mut shuffler = Shuffler::new_with_seeds(qbank, start, end, &sbank, seeds);
        
        if shuffler.make_exams(number_of_questions)
        {
            shuffler.shuffle_choices();
            let num_q = shuffler.get_qbank_length();
            Some(
                Self
                {
                    shuffler,
                    current_question_number: 0,
                    user_answers: vec![UserAnswer::None; num_q],
                    scoring_rule: ScoringRule::NoNegativeMarkingPartialCredit,
                }
            )
        }
        else
        {
            None
        }
    }

    // pub fn get_number_of_questions(&self) -> usize
    /// Returns the total number of questions in the session.
    /// 
    /// # Returns
    /// The total number of questions available in the question bank for the self-study session.
    /// 
    /// # Examples
    /// ```
    /// let total_questions = self_study.get_number_of_questions();
    /// println!("Total Questions: {}", total_questions);
    /// ```
    #[inline]
    pub fn get_number_of_questions(&self) -> usize
    {
        self.shuffler.get_qbank_length()
    }

    // pub fn get_question_by_number(&mut self, num: u16) -> Option<(u16, u8, String, String, Choices)>
    /// Retrieves a question by its number (1-based).
    /// 
    /// # Arguments
    /// * `num` - The question number (1-based).
    /// 
    /// # Returns
    /// `Option<(u16, u8, String, String, Choices)>` - An `Option` containing a tuple with the following elements if the question exists:
    /// - `u16`: The question number.
    /// - `u8`: The category ID of the question.
    /// - `String`: The category name.
    /// - `String`: The question text.
    /// - `Choices`: The shuffled choices for the question.
    /// If the question number is out of range or if there is an error retrieving the question, it returns `None`.
    /// 
    /// # Examples
    /// ```
    /// if let Some((num, category_id, category_str, question_text, choices)) = self_study.get_question_by_number(1) {
    ///     println!("Question {}: {} (Category: {})", num, question_text, category_str);
    ///     for (idx, choice) in choices.iter().enumerate()
    ///     { println!("  {}. {}", idx + 1, choice);}
    /// }
    /// else
    /// {
    ///     println!("Question not found.");
    /// }
    /// ```
    pub fn get_question_by_number(&mut self, num: u16) -> Option<(u16, u8, String, String, Choices)>
    {
        self.current_question_number = num;
        let shuffled_qset = self.shuffler.get_shuffled_qsets().get(0)?;
        if num as usize > shuffled_qset.get_shuffled_questions().len() || num == 0
            { return None; }

        let shuffled_question = shuffled_qset.get_shuffled_question(num)?;
        let real_question_number = shuffled_question.get_question();
        let shuffled_indices = shuffled_question.get_choices();

        let origin_question = self.shuffler.get_qbank().get_question(real_question_number as usize)?;
        let category_id = origin_question.get_category();
        let category_str = self.shuffler.get_header().get_category(category_id)?.clone();
        let question_text = origin_question.get_question().clone();
        let origin_choices = origin_question.get_choices();

        let mut choices = Choices::new();
        for &shuffled_index in shuffled_indices
        {
            if let Some(choice) = origin_choices.get((shuffled_index - 1) as usize)
                { choices.push(choice.clone()); }
            else
                { return None; }
        }
        Some((num, category_id, category_str, question_text, choices))
    }

    // pub fn next(&mut self) -> Option<(u16, u8, String, String, Choices)>
    /// Advances to the next question.
    /// 
    /// # Returns
    /// `Option<(u16, u8, String, String, Choices)>` - An `Option` containing a tuple with the following elements if the next question exists:
    /// - `u16`: The question number.
    /// - `u8`: The category ID of the question.
    /// - `String`: The category name.
    /// - `String`: The question text.
    /// - `Choices`: The shuffled choices for the question.
    /// If there are no more questions available, it returns `None`.
    /// 
    /// # Examples
    /// ```
    /// while let Some((num, category_id, category_str, question_text, choices)) = self_study.next() {
    ///     println!("Question {}: {} (Category: {})", num, question_text, category_str);
    ///     for (idx, choice) in choices.iter().enumerate()
    ///     { println!("  {}. {}", idx + 1, choice);}
    /// }
    /// println!("No more questions available.");
    /// ```
    pub fn next(&mut self) -> Option<(u16, u8, String, String, Choices)>
    {
        self.current_question_number += 1;
        self.get_question_by_number(self.current_question_number)
    }

    // pub fn prev(&mut self) -> Option<(u16, u8, String, String, Choices)>
    /// Moves to the previous question.
    /// 
    /// # Returns
    /// `Option<(u16, u8, String, String, Choices)>` - An `Option` containing a tuple with the following elements if the previous question exists:
    /// - `u16`: The question number.    
    /// - `u8`: The category ID of the question.
    /// - `String`: The category name.
    /// - `String`: The question text.
    /// - `Choices`: The shuffled choices for the question.
    /// If there are no more questions available, it returns `None`.
    /// 
    /// # Examples
    /// ```
    /// while let Some((num, category_id, category_str, question_text, choices)) = self_study.prev() {
    ///     println!("Question {}: {} (Category: {})", num, question_text, category_str);
    ///     for (idx, choice) in choices.iter().enumerate()
    ///     { println!("  {}. {}", idx + 1, choice);}
    /// }
    /// println!("No previous questions available.");
    /// ```
    pub fn prev(&mut self) -> Option<(u16, u8, String, String, Choices)>
    {
        if self.current_question_number > 0
        {
            self.current_question_number -= 1;
            self.get_question_by_number(self.current_question_number)
        }
        else
        {
            None
        }
    }

    // pub fn set_answer(&mut self, num: u16, answer: UserAnswer)
    /// Sets the user's answer for a question.
    /// 
    /// # Arguments
    /// * `num` - The question number (1-based).
    /// * `answer` - The user's answer, represented as a `UserAnswer` enum variant.
    /// 
    /// # Examples
    /// ```
    /// self_study.set_answer(1, UserAnswer::Choices(vec![true, false, true]));
    /// self_study.set_answer(2, UserAnswer::ShortAnswer("Example answer".to_string()));
    /// ```
    pub fn set_answer(&mut self, num: u16, answer: UserAnswer)
    {
        if num > 0 && (num as usize) <= self.user_answers.len()
            { self.user_answers[(num - 1) as usize] = answer; }
    }

    // pub fn get_answer(&self, num: u16) -> UserAnswer
    /// Retrieves the user's answer for a question.
    /// 
    /// # Arguments
    /// * `num` - The question number (1-based).
    /// 
    /// # Returns
    /// The user's answer, represented as a `UserAnswer` enum variant.
    /// If the question number is out of range, it returns `UserAnswer::None`.
    /// 
    /// # Examples
    /// ```
    /// let answer = self_study.get_answer(1);
    /// match answer {
    ///     UserAnswer::Choices(choices) => println!("Selected choices: {:?}", choices),
    ///     UserAnswer::ShortAnswer(ans) => println!("Short answer: {}", ans),
    ///     UserAnswer::None => println!("No answer provided."),
    /// }
    /// ```
    pub fn get_answer(&self, num: u16) -> UserAnswer
    {
        if num > 0 && (num as usize) <= self.user_answers.len()
            { self.user_answers[(num - 1) as usize].clone() }
        else
            { UserAnswer::None }
    }

    // pub fn set_scoring_rule(&mut self, rule: ScoringRule)
    /// Sets the scoring rule.
    /// 
    /// # Arguments
    /// * `rule` - The `ScoringRule` to be applied for scoring the self-study session.
    /// 
    /// # Examples
    /// ```
    /// self_study.set_scoring_rule(ScoringRule::NegativeMarkingPartialCredit);
    /// ```
    #[inline]
    pub fn set_scoring_rule(&mut self, rule: ScoringRule)
    {
        self.scoring_rule = rule;
    }

    // pub fn score(&self) -> f64
    /// Calculates the total score.
    /// 
    /// # Returns
    /// The total score for the self-study session, rounded to two decimal places.
    /// 
    /// # Examples
    /// ```
    /// let total_score = self_study.score();
    /// println!("Total score: {:.2}", total_score);
    /// ```
    pub fn score(&self) -> f64
    {
        let total_questions = self.get_number_of_questions();
        if total_questions == 0 { return 0.0; }

        let points_per_question = 100.0 / (total_questions as f64);
        let mut total_score = 0.0;

        for i in 0..total_questions
        {
            let num = (i + 1) as u16;
            let shuffled_qset = match self.shuffler.get_shuffled_qsets().get(0)
            {
                Some(s) => s,
                None => continue,
            };
            let shuffled_question = match shuffled_qset.get_shuffled_question(num)
            {
                Some(q) => q,
                None => continue,
            };
            let real_question_number = shuffled_question.get_question();
            let shuffled_indices = shuffled_question.get_choices();

            let origin_question = match self.shuffler.get_qbank().get_question(real_question_number as usize)
            {
                Some(q) => q,
                None => continue,
            };
            let category = origin_question.get_category();
            let origin_choices = origin_question.get_choices();
            let user_ans = &self.user_answers[i];

            // Extract correct answer information (based on shuffled choices)
            let mut correct_indices = Vec::new();
            for (idx, &shuffled_idx) in shuffled_indices.iter().enumerate()
            {
                if let Some(choice) = origin_choices.get((shuffled_idx - 1) as usize)
                {
                    if choice.1
                        { correct_indices.push(idx); }
                }
            }
            let choices_len = shuffled_indices.len();
            
            let mut question_score = 0.0;
            match category
            {
                Self::SINGLE_RESPONSE_MULTIPLE_CHOICE => {
                    if let UserAnswer::Choices(ans_vec) = user_ans
                        { question_score = self.do_single_response_multiple_choice(&ans_vec, &correct_indices, choices_len, points_per_question); }
                },
                Self::MULTIPLE_RESPONSE_MULTIPLE_CHOICE => {
                    if let UserAnswer::Choices(ans_vec) = user_ans
                        { question_score = self.do_multiple_response_multiple_choice(&ans_vec, &correct_indices, choices_len, points_per_question); }
                },
                Self::SHORT_ANSWER_SUBJECTIVE => {
                    if let UserAnswer::ShortAnswer(ans_str) = user_ans
                        { question_score = self.do_short_answer_subjective(ans_str, origin_choices, points_per_question); }
                },
                _ => {} // category 4 or others: 0
            }
            total_score += question_score;
        }

        if total_score < 0.0 { 0.0 } else { (total_score * 100.0).round() / 100.0 }
    }

    // fn do_single_response_multiple_choice(&self, ans_vec: &Vec<bool>, correct_indices: &Vec<usize>, choices_len: usize, points_per_question: f64) -> f64
    /// Evaluates the user's answer for a single-response multiple-choice
    /// question.
    /// 
    /// # Arguments
    /// * `ans_vec` - A vector of boolean values representing the user's
    ///   selected choices, where each boolean value indicates whether a
    ///   particular choice is selected.
    /// * `correct_indices` - A vector of indices representing the correct
    ///   choices for the question, based on the shuffled choices.
    /// * `choices_len` - The total number of choices available
    ///   for the question.
    /// * `points_per_question` - The number of points allocated
    ///   for the question.
    /// 
    /// # Returns
    /// The score for the question based on the user's answer,
    /// which is calculated as follows:
    /// - If the user selects exactly one choice and it is correct, the full
    ///   points for the question are awarded.
    /// - If the user selects exactly one choice and it is incorrect, negative
    ///   marking is applied based on the scoring rule and the number of
    ///   choices.
    /// - If the user selects no choices or more than one choice,
    ///   the score for the question is 0.
    /// 
    /// # Examples
    /// ```
    /// let score = self_study.do_single_response_multiple_choice(&vec![true, false, false], &vec![0], 3, 5.0);
    /// println!("Score for single-response multiple-choice question: {}", score);
    /// ```
    fn do_single_response_multiple_choice(&self, ans_vec: &Vec<bool>, correct_indices: &Vec<usize>, choices_len: usize, points_per_question: f64) -> f64
    {
        let mut question_score = 0.0_f64;
        let selected_indices: Vec<usize> = ans_vec.iter().enumerate()
            .filter(|&(_, &v)| v)
            .map(|(idx, _)| idx)
            .collect();
        
        if selected_indices.len() == 1 && selected_indices[0] == correct_indices[0]
        {
            question_score = points_per_question;
        }
        else if selected_indices.len() == 1 && selected_indices[0] != correct_indices[0]
        {
            match self.scoring_rule
            {
                ScoringRule::NegativeMarkingPartialCredit | ScoringRule::NegativeMarkingNoPartialCredit => {
                    if choices_len > 1
                        { question_score = -(points_per_question / (choices_len as f64 - 1.0)); }
                },
                _ => {}
            }
        }
        question_score
    }

    // fn do_multiple_response_multiple_choice(&self, ans_vec: &Vec<bool>, correct_indices: &Vec<usize>, choices_len: usize, points_per_question: f64) -> f64
    /// Evaluates the user's answer for a multiple-response multiple-choice
    /// question.
    /// 
    /// # Arguments
    /// * `ans_vec` - A vector of boolean values representing the user's
    ///   selected choices, where each boolean value indicates whether a
    ///   particular choice is selected.
    /// * `correct_indices` - A vector of indices representing the correct
    ///   choices for the question, based on the shuffled choices.
    /// * `choices_len` - The total number of choices available
    ///   for the question.
    /// * `points_per_question` - The number of points allocated
    ///   for the question.
    /// 
    /// # Returns
    /// The score for the question based on the user's answer,
    /// which is calculated as follows:
    /// - If the user selects all and only the correct choices,
    ///   the full points for the question are awarded.
    /// - If the user selects some correct choices and some incorrect choices,
    ///   the score is calculated based on the number of correct and incorrect
    ///   selections, and the scoring rule is applied accordingly.
    /// - If the user selects no choices or only incorrect choices,
    ///   negative marking is applied based on the scoring rule and the number
    ///   of choices.
    /// 
    /// # Examples
    /// ```
    /// let score = self_study.do_multiple_response_multiple_choice(&vec![true, false, true], &vec![0, 2], 3, 5.0);
    /// println!("Score for multiple-response multiple-choice question: {}", score);
    /// ```
    fn do_multiple_response_multiple_choice(&self, ans_vec: &Vec<bool>, correct_indices: &Vec<usize>, choices_len: usize, points_per_question: f64) -> f64
    {
        let mut question_score = 0.0_f64;
        let selected_indices: Vec<usize> = ans_vec.iter().enumerate()
            .filter(|&(_, &v)| v)
            .map(|(idx, _)| idx)
            .collect();
        
        let is_all_correct = selected_indices.len() == correct_indices.len() && 
            selected_indices.iter().all(|idx| correct_indices.contains(idx));

        if is_all_correct
        {
            question_score = points_per_question;
        }
        else if selected_indices.len() == correct_indices.len()
        {
            let s_c = selected_indices.iter().filter(|idx| correct_indices.contains(idx)).count() as f64;
            let s_w = selected_indices.len() as f64 - s_c;
            let c = correct_indices.len() as f64;
            let w = choices_len as f64 - c;

            match self.scoring_rule
            {
                ScoringRule::NegativeMarkingPartialCredit => question_score = (w * s_c - c * s_w) * points_per_question / (choices_len as f64),
                ScoringRule::NegativeMarkingNoPartialCredit => {
                    if choices_len > 1
                        { question_score = -(points_per_question / (choices_len as f64 * (choices_len as f64 - 1.0))); }
                },
                ScoringRule::NoNegativeMarkingPartialCredit => question_score = points_per_question * s_c / c,
                ScoringRule::NoNegativeMarkingNoPartialCredit => {},
            }
        }
        question_score
    }

    // fn do_short_answer_subjective(&self, ans_str: &String, origin_choices: &Vec<(String, bool)>, points_per_question: f64) -> f64
    /// Evaluates the user's answer for a short-answer subjective question.
    /// 
    /// # Arguments
    /// * `ans_str` - The user's answer as a string.
    /// * `origin_choices` - A vector of tuples containing the original choices
    ///   and their correctness (e.g., `Vec<(String, bool)>`).
    /// * `points_per_question` - The number of points allocated for the question.
    /// 
    /// # Returns
    /// The score for the question based on the user's answer, which is either
    /// the full points if the answer matches any of the correct answers
    /// (ignoring leading/trailing whitespace and case), or 0 if the answer does
    /// not match any of the correct answers.
    /// 
    /// # Examples
    /// ```
    /// let score = self_study.do_short_answer_subjective("Example answer".to_string(), vec![("Correct Answer".to_string(), true)], 5.0);
    /// println!("Score for short-answer question: {}", score);
    /// ```
    fn do_short_answer_subjective(&self, ans_str: &String, origin_choices: &Vec<(String, bool)>, points_per_question: f64) -> f64
    {
        let mut question_score = 0.0_f64;
        let trimmed_ans = ans_str.trim().to_lowercase();
        if !trimmed_ans.is_empty()
        {
            let mut matched = false;
            for choice in origin_choices
            {
                if choice.1 && choice.0.trim().to_lowercase() == trimmed_ans
                {
                    matched = true;
                    break;
                }
            }
            if matched { question_score = points_per_question; }
        }
        question_score
    }
}
