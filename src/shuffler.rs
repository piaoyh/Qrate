// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


// use std::io;
// use std::fs::File;
// use std::io::Write;
use std::collections::HashMap;

use cryptocol::random::Random_PRNG_Creator;

use crate::{ QBank, Student };


// const NUMBER_QUESTIONS: usize = 48; // 51;
/// The number of questions to be selected for a single exam paper.
pub const NUMBER_SELECTED_QUESTIONS: usize = 25;
// const SAVE_PAPER_SPACE: &str = "";  // "\t"

/// A type alias for a vector of `ShuffledQuestion`s, representing a set of shuffled questions.
pub type ShuffledQuestions = Vec<ShuffledQuestion>;

/// A type alias for a vector of `ShuffledQSet`s, representing a collection of exam sets for multiple students.
pub type ShuffledQSets = Vec<ShuffledQSet>;


/// Represents a question with its choices shuffled.
#[derive(Debug, Clone)]
pub struct ShuffledQuestion
{
    question: u16,      // 1-based index into the original QBank.
    choices: Vec<u8>,   // 1-based indices representing the shuffled order of choices.
}

impl ShuffledQuestion
{
    // pub fn new(question: u16, number_of_choices: u8) -> Self
    /// Creates a new `ShuffledQuestion` with an ordered list of choices.
    /// 
    /// # Arguments
    /// * `question` - The 1-based index of the question in the `QBank`.
    /// * `number_of_choices` - The total number of choices for this question.
    /// 
    /// # Output
    /// `Self` - A new `ShuffledQuestion` instance.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let sq = ShuffledQuestion::new(10, 4);
    /// assert_eq!(sq.get_question(), 10);
    /// assert_eq!(sq.how_many_choices(), 4);
    /// ```
    pub fn new(question: u16, number_of_choices: u8) -> Self
    {
        let mut choices = Vec::new();
        for i in 1..=number_of_choices
            { choices.push(i); }
        ShuffledQuestion { question, choices }
    }

    // pub fn get_question(&self) -> u16
    /// Gets the 1-based index of the original question.
    /// 
    /// # Output
    /// `u16` - The index of the question.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let sq = ShuffledQuestion::new(5, 4);
    /// assert_eq!(sq.get_question(), 5);
    /// ```
    #[inline]
    pub fn get_question(&self) -> u16
    {
        self.question
    }

    // pub fn set_question(&mut self, question: u16)
    /// Sets the 1-based index of the original question.
    /// 
    /// # Arguments
    /// * `question` - The new 1-based index for the question.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let mut sq = ShuffledQuestion::new(1, 4);
    /// sq.set_question(2);
    /// assert_eq!(sq.get_question(), 2);
    /// ```
    #[inline]
    pub fn set_question(&mut self, question: u16)
    {
        self.question = question;
    }

    // pub fn get_choice(&self, idx: usize) -> u8
    /// Gets the shuffled 1-based index of a choice.
    /// 
    /// # Arguments
    /// * `idx` - The 1-based index into the shuffled choice vector.
    /// 
    /// # Output
    /// `u8` - The original 1-based index of the choice at the shuffled position.
    /// Returns 0 if `idx` is out of bounds.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let mut sq = ShuffledQuestion::new(1, 4);
    /// sq.set_choices(vec![4, 1, 3, 2]);
    /// assert_eq!(sq.get_choice(1), 4);
    /// ```
    pub fn get_choice(&self, idx: usize) -> u8
    {
        if idx > 0 && idx <= self.choices.len()
            { self.choices[idx - 1] }
        else
            { 0 }
    }

    // pub fn set_choice(&mut self, idx: usize, choice: u8) -> bool
    /// Sets the shuffled 1-based index of a choice at a specific position.
    /// 
    /// # Arguments
    /// * `idx` - The 1-based index in the choices vector to modify.
    /// * `choice` - The new original 1-based choice index to place at `idx`.
    /// 
    /// # Output
    /// `bool` - Returns `true` if the choice was successfully set, `false` otherwise.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let mut sq = ShuffledQuestion::new(1, 4);
    /// sq.set_choice(1, 3);
    /// assert_eq!(sq.get_choice(1), 3);
    /// ```
    #[inline]
    pub fn set_choice(&mut self, idx: usize, choice: u8) -> bool
    {
        if idx == 0
            { return false; }
        self.choices[idx - 1] = choice;
        true
    }

    // pub fn get_choices(&self) -> &Vec<u8>
    /// Gets a reference to the vector of shuffled choice indices.
    /// 
    /// # Output
    /// `&Vec<u8>` - A reference to the shuffled choices.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let mut sq = ShuffledQuestion::new(1, 4);
    /// assert_eq!(sq.get_choices(), &vec![1, 2, 3, 4]);
    /// ```
    #[inline]
    pub fn get_choices(&self) -> &Vec<u8>
    {
        &self.choices
    }

    // pub fn set_choices(&mut self, choices: Vec<u8>)
    /// Replaces the entire vector of shuffled choice indices.
    /// 
    /// # Arguments
    /// * `choices` - The new vector of 1-based choice indices.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let mut sq = ShuffledQuestion::new(1, 4);
    /// sq.set_choices(vec![4, 3, 2, 1]);
    /// assert_eq!(sq.get_choices(), &vec![4, 3, 2, 1]);
    /// ```
    #[inline]
    pub fn set_choices(&mut self, choices: Vec<u8>)
    {
        self.choices = choices;
    }

    // pub fn how_many_choices(&self) -> usize
    /// Returns the number of choices for the question.
    /// 
    /// # Output
    /// `usize` - The number of choices.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let sq = ShuffledQuestion::new(1, 5);
    /// assert_eq!(sq.how_many_choices(), 5);
    /// ```
    #[inline]
    pub fn how_many_choices(&self) -> usize
    {
        self.choices.len()
    }

    // pub fn shuffle(&mut self)
    /// Shuffles the order of the choices in place.
    /// 
    /// # Examples
    /// ```
    /// use qrate::shuffler::ShuffledQuestion;
    /// let mut sq = ShuffledQuestion::new(1, 4);
    /// let original_choices = sq.get_choices().clone();
    /// sq.shuffle();
    /// // The order is random, so we just check that the elements are the same
    /// let mut shuffled_choices = sq.get_choices().clone();
    /// shuffled_choices.sort();
    /// assert_eq!(original_choices, shuffled_choices);
    /// ```
    pub fn shuffle(&mut self)
    {
        let mut prng = Random_PRNG_Creator::create();
        let max = self.how_many_choices();
        for _ in 0..3
        {
            for i in 0..max
            {
                let j = prng.random_under_uint_(max);
                (self.choices[i], self.choices[j]) = (self.choices[j], self.choices[i]);
            }
        }
    }
}


/// Represents a complete set of shuffled questions for a single student.
#[derive(Debug, Clone)]
pub struct ShuffledQSet
{
    student: Student,
    questions: ShuffledQuestions,
}

impl ShuffledQSet
{
    // pub fn new(qbank: &QBank, start: u16, end: u16, selected: usize, student: &Student) -> Option<Self>
    /// Creates a new set of shuffled questions for a student by randomly selecting a specified number of questions from a `QBank` within a given range.
    /// Each selected question will belong to a unique group. The choices for each question are shuffled upon creation.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` to draw questions from.
    /// * `start` - The 1-based starting index of questions to consider from the `QBank`.
    /// * `end` - The 1-based ending index of questions to consider from the `QBank`.
    /// * `selected` - The number of questions to randomly select. Each selected question will have a unique group ID.
    /// * `student` - The `Student` for whom this question set is.
    ///
    /// # Output
    /// `Option<Self>` - A new `ShuffledQSet` instance, or `None` if:
    ///                  - The question range is invalid (start > end, start > last, end > last, or selected is 0).
    ///                  - The number of available unique question groups is less than `selected`.
    ///
    /// # Examples
    /// ```
    /// use qrate::{QBank, Student, Question, shuffler::ShuffledQSet};
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![])); // id 1, group 1
    /// qbank.push_question(Question::new(2, 1, 1, "Q2".to_string(), vec![])); // id 2, group 1
    /// qbank.push_question(Question::new(3, 2, 1, "Q3".to_string(), vec![])); // id 3, group 2
    /// qbank.push_question(Question::new(4, 3, 1, "Q4".to_string(), vec![])); // id 4, group 3
    /// qbank.push_question(Question::new(5, 4, 1, "Q5".to_string(), vec![])); // id 5, group 4
    ///
    /// let student = Student::new("Test".to_string(), "123".to_string());
    /// // Select 3 questions from unique groups between ID 1 and 5
    /// let qset = ShuffledQSet::new(&qbank, 1, 5, 3, &student).unwrap();
    /// assert_eq!(qset.get_student().get_name(), "Test");
    /// assert_eq!(qset.get_shuffled_questions().len(), 3);
    ///
    /// // Try to select more questions than available unique groups (4 unique groups total)
    /// let qset_none = ShuffledQSet::new(&qbank, 1, 5, 5, &student);
    /// assert!(qset_none.is_none());
    ///
    /// // Invalid range
    /// let qset_invalid_range = ShuffledQSet::new(&qbank, 5, 1, 1, &student);
    /// assert!(qset_invalid_range.is_none());
    ///
    /// // Selected count is 0
    /// let qset_zero_selected = ShuffledQSet::new(&qbank, 1, 5, 0, &student);
    /// assert!(qset_zero_selected.is_none());
    /// ```
    pub fn new(qbank: &QBank, start: u16, end: u16, selected: usize, student: &Student) -> Option<Self>
    {
        let last = qbank.get_questions().len() as u16;
        if (start == 0) || (start > end) || (start > last) || (end > last) || (selected == 0)
            { return None }

        // Filter questions by range
        let questions_in_range: Vec<crate::Question> = qbank.get_questions()
            .iter()
            .filter(|q| q.get_id() >= start && q.get_id() <= end)
            .cloned() // Clone to get owned Question objects
            .collect();

        // Group questions by group id
        let mut grouped_questions: HashMap<u16, Vec<crate::Question>> = HashMap::new(); // Change Vec<&crate::Question> to Vec<crate::Question>
        for question in questions_in_range // question is now crate::Question
            { grouped_questions.entry(question.get_group()).or_default().push(question); }

        let mut available_groups_keys: Vec<u16> = grouped_questions.keys().cloned().collect();
        if available_groups_keys.len() < selected
            { return None; }

        let mut prng = Random_PRNG_Creator::create(); // Slapdash::new() returns a Random_Generic object
        let mut selected_shuffled_questions = ShuffledQuestions::new();

        for _ in 0..selected
        {
            // Randomly select a group key
            let group_key_index = prng.random_under_uint_(available_groups_keys.len());
            let selected_group_key = available_groups_keys.remove(group_key_index as usize);

            // From the selected group, pick one question randomly
            if let Some(questions_in_group) = grouped_questions.get(&selected_group_key)
            {
                if !questions_in_group.is_empty()
                {
                    let question_index = prng.random_under_uint_(questions_in_group.len());
                    let original_question = &questions_in_group[question_index as usize]; // original_question is now &crate::Question
                    let number_of_choices = original_question.get_choices().len() as u8;
                    let mut shuffled_question = ShuffledQuestion::new(original_question.get_id(), number_of_choices);
                    shuffled_question.shuffle();
                    selected_shuffled_questions.push(shuffled_question);
                }
            }
        }
        
        Some(Self{ student: student.clone(), questions: selected_shuffled_questions })
    }

    // pub fn shuffle(&mut self)
    /// Shuffles the order of the questions within the set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{QBank, Student, shuffler::ShuffledQSet};
    /// use qrate::Question;
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![])); // Add a question
    /// qbank.push_question(Question::new(2, 1, 1, "Q2".to_string(), vec![]));
    /// qbank.push_question(Question::new(3, 1, 1, "Q3".to_string(), vec![]));
    /// qbank.push_question(Question::new(4, 1, 1, "Q4".to_string(), vec![]));
    /// qbank.push_question(Question::new(5, 1, 1, "Q5".to_string(), vec![]));
    /// let student = Student::new("Test".to_string(), "123".to_string());
    /// let mut qset = ShuffledQSet::new(&qbank, &student, 1, 5).unwrap();
    /// let original_order: Vec<u16> = qset.get_shuffled_questions().iter().map(|q| q.get_question()).collect();
    /// qset.shuffle();
    /// let shuffled_order: Vec<u16> = qset.get_shuffled_questions().iter().map(|q| q.get_question()).collect();
    /// assert_eq!(original_order.len(), shuffled_order.len());
    /// ```
    pub fn shuffle(&mut self)
    {
        let mut prng = Random_PRNG_Creator::create();
        let max = self.questions.len();
        for _ in 0..3
        {
            for i in 0..max
            {
                let j = prng.random_under_uint_(max);
                (self.questions[i], self.questions[j]) = (self.questions[j].clone(), self.questions[i].clone());
            }
        }
    }

    // pub fn get_student(&self) -> &Student
    /// Gets a reference to the `Student` associated with this question set.
    /// 
    /// # Output
    /// `&Student` - A reference to the student.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{QBank, Student, shuffler::ShuffledQSet};
    /// use qrate::Question;
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![])); // Add a question
    /// let student = Student::new("Test".to_string(), "123".to_string());
    /// let qset = ShuffledQSet::new(&qbank, &student, 1, 1).unwrap();
    /// assert_eq!(qset.get_student().get_name(), "Test");
    /// ```
    #[inline]
    pub fn get_student(&self) -> &Student
    {
        &self.student
    }

    // pub fn set_student(&mut self, student: &Student)
    /// Sets the `Student` for this question set.
    /// 
    /// # Arguments
    /// * `student` - A reference to the new `Student`.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{QBank, Student, shuffler::ShuffledQSet};
    /// use qrate::Question;
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![])); // Add a question
    /// let mut student1 = Student::new("Test1".to_string(), "123".to_string());
    /// let student2 = Student::new("Test2".to_string(), "456".to_string());
    /// let mut qset = ShuffledQSet::new(&qbank, &student1, 1, 1).unwrap();
    /// qset.set_student(&student2);
    /// assert_eq!(qset.get_student().get_name(), "Test2");
    /// ```
    #[inline]
    pub fn set_student(&mut self, student: &Student)
    {
        self.student = student.clone();
    }

    // pub fn get_shuffled_questions(&self) -> &ShuffledQuestions
    /// Gets a reference to the shuffled questions.
    /// 
    /// # Output
    /// `&ShuffledQuestions` - A reference to the vector of `ShuffledQuestion`s.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{QBank, Student, shuffler::ShuffledQSet};
    /// use qrate::Question;
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![])); // Add a question
    /// qbank.push_question(Question::new(2, 1, 1, "Q2".to_string(), vec![]));
    /// qbank.push_question(Question::new(3, 1, 1, "Q3".to_string(), vec![]));
    /// let student = Student::new("Test".to_string(), "123".to_string());
    /// let qset = ShuffledQSet::new(&qbank, &student, 1, 3).unwrap();
    /// assert_eq!(qset.get_shuffled_questions().len(), 3);
    /// ```
    #[inline]
    pub fn get_shuffled_questions(&self) -> &ShuffledQuestions
    {
        &self.questions
    }

    // pub fn set_shuffled_questions(&mut self, questions: ShuffledQuestions)
    /// Replaces the shuffled questions in this set.
    /// 
    /// # Arguments
    /// * `questions` - The new vector of `ShuffledQuestion`s.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{QBank, Student, shuffler::{ShuffledQSet, ShuffledQuestion}};
    /// use qrate::Question;
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![])); // Add a question
    /// let student = Student::new("Test".to_string(), "123".to_string());
    /// let mut qset = ShuffledQSet::new(&qbank, &student, 1, 1).unwrap();
    /// let new_questions = vec![ShuffledQuestion::new(10, 4), ShuffledQuestion::new(11, 4)];
    /// qset.set_shuffled_questions(new_questions);
    /// assert_eq!(qset.get_shuffled_questions().len(), 2);
    /// assert_eq!(qset.get_shuffled_questions()[0].get_question(), 10);
    /// ```
    #[inline]
    pub fn set_shuffled_questions(&mut self, questions: ShuffledQuestions)
    {
        self.questions = questions;
    }

    // pub fn get_shuffled_question(&self, question_number: u16) -> Option<&ShuffledQuestion>
    /// Retrieves a reference to a `ShuffledQuestion` by its 1-based question number.
    ///
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to retrieve.
    ///
    /// # Output
    /// `Option<&ShuffledQuestion>` - An `Option` containing a reference to the `ShuffledQuestion` if found,
    ///                                 or `None` if the `question_number` is invalid (e.g., 0).
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Student, Question, ShuffledQSet, ShuffledQuestion };
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![]));
    /// let student = Student::new("Test".to_string(), "123".to_string());
    /// let qset = ShuffledQSet::new(&qbank, &student, 1, 1).unwrap();
    /// assert_eq!(qset.get_shuffled_question(1).unwrap().get_question(), 1);
    /// assert!(qset.get_shuffled_question(0).is_none());
    /// ```
    #[inline]
    pub fn get_shuffled_question(&self, question_number: u16) -> Option<&ShuffledQuestion>
    {
        if question_number == 0 { None } else { Some(&self.questions[(question_number - 1) as usize]) }
    }
}


/*
pub struct Exam
{
    qbs: Vec::<Question>,
    selected_qb: Vec::<ShuffledQuestion>,
}

impl Exam
{
    pub fn instantiate() -> Exam
    {
        let mut e = Exam
        {
            qbs: Vec::<OldQuestion>::new(),
            selected_qb: Vec::<ShuffledQuestion>::new(),    
        };
        e.init_bank();
        e.select_questions();
        return e;
    }

    fn initialize_selected_questions(&self) -> [u16; NUMBER_QUESTIONS]
    {
        let mut sq = [0u16; NUMBER_QUESTIONS];
        for i in 0..NUMBER_QUESTIONS
        {
            sq[i] = (i+1) as u16;
        }

        let mut prng = PRNG::new();
        for _ in 0..NUMBER_QUESTIONS*3
        {
            let i = prng.random_under_uint_(NUMBER_SELECTED_QUESTIONS);
            let mut j = prng.random_under_uint_(NUMBER_QUESTIONS);
            while i == j
            {
                j = prng.random_under_uint_(NUMBER_QUESTIONS);
            }
            let tmp = sq[i];
            sq[i] = sq[j];
            sq[j] = tmp;
        }

        return sq;
    }

    fn shuffle_choices(&self) -> [u8; NUMBER_CHOICES]
    {
        let mut c: [u8; NUMBER_CHOICES] = [1, 2, 3, 4];
        let mut prng = PRNG::new();
        for _ in 0..(NUMBER_CHOICES * 3)
        {
            let i = prng.random_under_uint_(NUMBER_CHOICES);
            let mut j = prng.random_under_uint_(NUMBER_CHOICES);
            while i == j
            {
                j = prng.random_under_uint_(NUMBER_CHOICES);
            }
            let tmp = c[i];
            c[i] = c[j];
            c[j] = tmp;
        }

        return c;
    }

    fn select_questions(&mut self)
    {
        self.selected_qb.clear();
        let selected_questions = self.initialize_selected_questions();
    //    println!("{:?}", selected_questions);

        for &question in &selected_questions[0..NUMBER_SELECTED_QUESTIONS]
        {
            let q = ShuffledQuestion {
                question,
                choice: self.shuffle_choices(),
            };
            self.selected_qb.push(q);
        }
    //    println!("{:?}", selected_qb);
    }

    fn write_selected_questions(&self) -> String
    {
        let mut paper = String::new();
        let mut number = 1;
        for selected in &self.selected_qb
        {
            let qb = &self.qbs[(selected.question()-1) as usize];
            paper.push_str(&format!("{}. {}\n", number, qb.question()));
            for ch in 1..=4
            {
                paper.push_str(&format!("{}{}) {}\n", SAVE_PAPER_SPACE, ch, qb.choice(selected.choice(ch))));
            }
            number += 1;
            paper.push_str(&format!("\n"));
        }
        return paper;
    }
    
    pub fn print_selected_questions(&self) -> &Exam
    {
        println!("\n==== {} Randomly Chosen Questions ===\n", NUMBER_SELECTED_QUESTIONS);
        println!("{}", self.write_selected_questions());
        return self;
    }

    fn write_answers_of_selected(&self) -> String
    {
        let mut paper = String::new();
        let mut number = 1;
        for selected in &self.selected_qb
        {
            let qb = &self.qbs[(selected.question()-1) as usize];
            let (mut answer1, mut answer2) = qb.answer();
            for ch in 0..4
            {
                if selected.choice[ch] == answer1
                {
                    answer1 = (ch+1) as u8;
                    break;
                }
            }
            if qb.has_multianswers()
            {
                for ch in 0..4
                {
                    if selected.choice[ch] == answer2
                    {
                        answer2 = (ch+1) as u8;
                        break;
                    }
                }
                paper.push_str(&format!("{}. ({}, {})\t", number, answer1, answer2));
            }
            else
            {
                paper.push_str(&format!("{}. {}\t", number, answer1));
            }
            number += 1;
        }
        return paper;
    }

    pub fn print_answers_of_selected(&self)
    {
        println!("\n==== The Answers of The Above Questions ===\n");
        println!("{}", self.write_answers_of_selected());
        println!("\n");
    }

    // fn write_exams(&mut self, students: &Students) -> (String, String)
    // {
    //     let mut paper = String::new();
    //     let mut card = String::new();
    //     for st in students
    //     {
    //         self.select_questions();
    //         paper.push_str(is_question_bank::ISQuestionBank::get_header(&st).as_str());
    //         paper.push_str(&self.write_selected_questions());
    //         paper.push('\n');
    //         card.push_str(&format!("Name: {}\tID: {}\n", st.get_name(), st.get_id()));
    //         card.push_str(&self.write_answers_of_selected());
    //         card.push('\n');
    //     }
    //     return (paper, card);
    // }

    pub fn print_exams(&mut self, students: &Students)
    {
        let (paper, card) = self.write_exams(&students);
        println!("{}", paper);
        println!("{}", card);
    }

    pub fn save_exams(&mut self, students: &Students, path: &str)
    {
        let (paper, card) = self.write_exams(&students);
        let mut f = File::create(&path).unwrap();
        let _ = f.write(paper.as_bytes());
        let _ = f.write(card.as_bytes());
    }

    // fn init_bank(&mut self)
    // {
    //     self.qbs = is_question_bank::ISQuestionBank::new().questions;
    // }  
}
*/