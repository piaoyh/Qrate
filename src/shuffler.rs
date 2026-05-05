// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::collections::HashMap;
use cryptocol::random::{ Random_PRNG_Creator, Random };
use crate::{ QBank, SBank, SBankHelper,
            ShuffledQSet, ShuffledQSets, ShuffledQuestion, ShuffledQuestions };


/// The `Shuffler` struct is responsible for managing the shuffling of questions
/// and the generation of shuffled question sets for students.
/// It holds references to the original question bank and student bank,
/// as well as the generated shuffled question sets.
pub struct Shuffler
{
    qbank: QBank,
    sbank: SBank,
    shuffled_qsets: ShuffledQSets,
    prng: Random,
}

impl Shuffler
{
    // pub fn new(qbank: QBank, sbank: SBank) -> Self
    /// Creates a new `Shuffler` instance
    /// with the provided question bank and student bank.
    /// 
    /// # Arguments
    /// * `qbank` - The question bank containing the questions to be shuffled.
    /// * `sbank` - The student bank containing the student information
    ///   for whom the exams will be generated.
    /// 
    /// # Returns
    /// `Self` - A new `Shuffler` instance initialized with the given question
    /// bank and student bank, and an empty set of shuffled question sets.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, SBank, shuffler::Shuffler };
    /// let qbank = QBank::new(); // Assume this creates a QBank instance
    /// let sbank = SBank::new(); // Assume this creates an SBank instance
    /// let shuffler = Shuffler::new(qbank, sbank);
    /// assert!(shuffler.shuffled_qsets.is_empty());
    /// ```
    pub fn new(qbank: QBank, sbank: SBank) -> Self
    {
        let mut me = Shuffler { qbank, sbank, shuffled_qsets: Vec::new(), prng: Random_PRNG_Creator::create() };
        me.qbank.optimize();
        me.sbank.optimize();
        me
    }

    // pub fn make_exams(&mut self, number_of_questions: u16) -> ShuffledQSets
    /// Generates shuffled question sets for all students in the student bank.
    /// 
    /// # Arguments
    /// * `number_of_questions` - The number of questions for each student's exam.
    /// 
    /// # Returns
    /// `ShuffledQSets` - A vector of question sets, one for each student.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, SBank, shuffler::Shuffler };
    /// let qbank = QBank::new_with_default();
    /// let sbank = SBank::new_with_default();
    /// let mut shuffler = Shuffler::new(qbank, sbank);
    /// let exams = shuffler.make_exams(5);
    /// assert_eq!(exams.len(), shuffler.sbank.len());
    /// ```
    pub fn make_exams(&mut self, number_of_questions: u16) -> ShuffledQSets
    {
        self.shuffled_qsets.clear();
        let students = self.sbank.clone();
        for student in students
        {
            let selected = self.choose_questions(number_of_questions as usize);
            let shuffled = self.shuffle_choices(selected);
            let qset = ShuffledQSet::from_parts(student, shuffled);
            self.shuffled_qsets.push(qset);
        }
        self.shuffled_qsets.clone()
    }

    // pub fn choose_questions(&mut self, number_of_questions: usize) -> ShuffledQuestions
    /// Selects a specified number of questions from the bank randomly,
    /// ensuring no two questions come from the same group.
    /// 
    /// # Arguments
    /// * `number_of_questions` - The number of questions to select.
    /// 
    /// # Returns
    /// `ShuffledQuestions` - A vector of shuffled questions.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, SBank, shuffler::Shuffler };
    /// let qbank = QBank::new_with_default(); // Assume it has 10 groups
    /// let sbank = SBank::new_with_default();
    /// let mut shuffler = Shuffler::new(qbank, sbank);
    /// let selected = shuffler.choose_questions(5);
    /// assert_eq!(selected.len(), 5);
    /// ```
    pub fn choose_questions(&mut self, mut number_of_questions: usize) -> ShuffledQuestions
    {
        let max_groups = self.qbank.get_number_of_groups();
        if number_of_questions > max_groups
            { number_of_questions = max_groups; }

        let mut groups: HashMap<u16, Vec<usize>> = HashMap::new();
        for (idx, q) in self.qbank.get_questions().iter().enumerate()
        {
            groups.entry(q.get_group()).or_insert_with(Vec::new).push(idx);
        }

        let mut group_ids: Vec<u16> = groups.keys().cloned().collect();
        for i in (1..group_ids.len()).rev()
        {
            let j = self.prng.random_under_uint_(i + 1);
            group_ids.swap(i, j);
        }

        let mut shuffled_questions = ShuffledQuestions::new();
        for i in 0..number_of_questions
        {
            let group_id = group_ids[i];
            let question_indices = &groups[&group_id];
            let chosen_idx = question_indices[self.prng.random_under_uint_(question_indices.len())];
            let question = &self.qbank.get_questions()[chosen_idx];
            let shuffled_question = ShuffledQuestion::new(question.get_id(), question.get_choices().len() as u8);
            shuffled_questions.push(shuffled_question);
        }
        shuffled_questions
    }

    // pub fn shuffle_choices(&mut self, mut shuffled_questions: ShuffledQuestions) -> ShuffledQuestions
    /// Shuffles the order of choices for each question in the provided set.
    /// 
    /// # Arguments
    /// * `shuffled_questions` - The set of questions whose choices are to be shuffled.
    /// 
    /// # Returns
    /// `ShuffledQuestions` - The same set of questions but with their choices shuffled.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, SBank, shuffler::Shuffler };
    /// let qbank = QBank::new_with_default();
    /// let sbank = SBank::new_with_default();
    /// let mut shuffler = Shuffler::new(qbank, sbank);
    /// let selected = shuffler.choose_questions(5);
    /// let shuffled = shuffler.shuffle_choices(selected);
    /// ```
    pub fn shuffle_choices(&mut self, mut shuffled_questions: ShuffledQuestions) -> ShuffledQuestions
    {
        for sq in &mut shuffled_questions
        {
            sq.shuffle();
        }
        shuffled_questions
    }

    pub fn shuffle(&mut self, _start: u16, _end: u16, _number_of_questions: u16)
    {
        
    }


}
