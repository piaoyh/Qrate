// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use cryptocol::random::{ Random_PRNG_Creator, Random };
use crate::{ Header, Student, QBank, SBank, SBankHelper, ShuffledQSet, ShuffledQSets };


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
    /// * `start` - The 1-based starting index of questions to consider from the `QBank`.
    /// * `end` - The 1-based ending index of questions to consider from the `QBank`.
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
    /// let mut qbank = QBank::new_with_default();
    /// let mut question = Question::new_empty();
    /// qbank.push_question(question.clone());
    /// question.set_id(2);
    /// question.set_group(2);
    /// qbank.push_question(question.clone());
    /// question.set_id(3);
    /// question.set_group(2);
    /// qbank.push_question(question);
    /// let sbank = SBank::new();
    /// let shuffler = Shuffler::new(qbank, 1, 2, sbank);
    /// assert!(shuffler.shuffled_qsets.is_empty());
    /// ```
    pub fn new(qbank: &QBank, start: u16, end: u16, sbank: &SBank) -> Self
    {
        let mut me = Self {
            qbank: qbank.select_questions(start, end),
            sbank: sbank.clone(),
            shuffled_qsets: Vec::new(),
            prng: Random_PRNG_Creator::create()
        };
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
    /// - `Some(ShuffledQSets)` which is a vector of question sets,
    ///   one for each student, wrapped by Some if succeeded.
    /// - `None` if failed.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, SBank, shuffler::Shuffler };
    /// let mut qbank = QBank::new_with_default();
    /// let mut question = Question::new_empty();
    /// qbank.push_question(question.clone());
    /// question.set_id(2);
    /// question.set_group(2);
    /// qbank.push_question(question.clone());
    /// question.set_id(3);
    /// question.set_group(2);
    /// qbank.push_question(question.clone());
    /// question.set_id(4);
    /// question.set_group(4);
    /// qbank.push_question(question);
    /// let sbank = SBank::new();
    /// sbank.push(Student::new("Alice", "1"));
    /// sbank.push(Student::new("Bob", "2"));
    /// sbank.push(Student::new("Caleb", "3"));
    /// let shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// let exams = shuffler.make_exams(2);
    /// assert_eq!(exams.unwrap().len(), shuffler.sbank.len());
    /// ```
    pub fn make_exams(&mut self, number_of_questions: u16) -> Option<ShuffledQSets>
    {
        self.shuffled_qsets.clear();
        for student in self.sbank.clone()
        {
            let qset = self.create_shuffled_qset(student, number_of_questions as usize)?;
            self.shuffled_qsets.push(qset);
        }
        Some(self.shuffled_qsets.clone())
    }

    // pub fn create_shuffled_qset(&mut self, student: Student, number_of_questions: usize) -> ShuffledQuestions
    /// Selects a specified number of questions from the bank randomly,
    /// ensuring no two questions come from the same group.
    /// 
    /// # Arguments
    /// * `number_of_questions` - The number of questions to select.
    /// 
    /// # Returns
    /// A new `ShuffledQSet` object.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, SBank, shuffler::Shuffler };
    /// let mut qbank = QBank::new_with_default();
    /// let mut question = Question::new_empty();
    /// qbank.push_question(question.clone());
    /// question.set_id(2);
    /// question.set_group(2);
    /// qbank.push_question(question.clone());
    /// question.set_id(3);
    /// question.set_group(2);
    /// qbank.push_question(question.clone());
    /// question.set_id(4);
    /// question.set_group(4);
    /// qbank.push_question(question);
    /// let student = Student::new();
    /// let shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// let qset = shuffler.create_shuffled_qset(student, 2).unwrap();
    /// assert_eq!(qset.get_shuffled_questions().len(), 2);
    /// ```
    #[inline]
    pub fn create_shuffled_qset(&mut self, student: Student, number_of_questions: usize) -> Option<ShuffledQSet>
    {
        ShuffledQSet::new(&(self.qbank), number_of_questions, student, &mut self.prng)
    }

    // pub fn shuffle_choices(&mut self)
    /// Shuffles the order of choices for each question in the provided set.
    /// 
    /// # Arguments
    /// * `shuffled_questions` - The set of questions whose choices are to be shuffled.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, SBank, shuffler::Shuffler };
    /// let mut qbank = QBank::new_with_default();
    /// let mut question = Question::new_empty();
    /// qbank.push_question(question.clone());
    /// question.set_id(2);
    /// question.set_group(2);
    /// question.push_choices(("abc", true));
    /// question.push_choices(("def", false));
    /// question.push_choices(("ghi", false));
    /// qbank.push_question(question.clone());
    /// question.set_id(3);
    /// question.set_group(2);
    /// question.push_choices(("가나다", false));
    /// question.push_choices(("라마바", false));
    /// question.push_choices(("사아자", true));
    /// qbank.push_question(question);
    /// let sbank = SBank::new();
    /// let mut shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// shuffler.shuffle.choices();
    /// ```
    pub fn shuffle_choices(&mut self)
    {
        for sq in &mut self.shuffled_qsets
        {
            for q in sq.get_shuffled_questions_mut()
                { q.shuffle(&mut self.prng); }
        }
    }

    #[inline]
    pub fn get_qbank(&self) -> &QBank
    {
        &self.qbank
    }

    #[inline]
    pub fn get_sbank(&self) -> &SBank
    {
        &self.sbank
    }

    #[inline]
    pub fn get_shuffled_qsets(&self) -> &ShuffledQSets
    {
        &self.shuffled_qsets
    }

    #[inline]
    pub fn get_prng(&self) -> &Random
    {
        &self.prng
    }

    #[inline]
    pub fn get_qbank_length(&self) -> usize
    {
        self.qbank.get_length()
    }

    #[inline]
    pub fn get_header(&self) -> &Header
    {
        self.qbank.get_header()
    }

    // pub fn get_shuffled_questions(&self, student_idx: usize) -> Option<ShuffledQSet>
    ///
    /// # Arguments
    /// * `student_idx` - 0-based index.
    #[inline]
    pub fn get_shuffled_questions(&self, student_idx: usize) -> Option<ShuffledQSet>
    {
        if student_idx >= self.sbank.len()
            { None }
        else
            { Some(self.shuffled_qsets[student_idx].clone()) }
    }

    // pub fn get_shuffled_question(&self, student_idx: usize, question_idx: usize) -> &Question
    ///
    /// # Arguments
    /// * `student_idx`: 0-based index of students.
    /// * `question_idx`: 0-based index of questions of the `student_idx`-th student.
    /// 
    /// # Returns
    /// The question ID (1-based) if `student_idx` is less than the number of
    /// students and `question_idx` is less than the number of questions for
    /// the `student_idx`-th student.
    /// 0, otherwise.
    #[inline]
    pub fn get_shuffled_question(&self, student_idx: usize, question_idx: usize) -> u16
    {
        if student_idx < self.sbank.len() && question_idx < self.shuffled_qsets[student_idx].get_shuffled_questions().len()
            { self.shuffled_qsets[student_idx].get_shuffled_questions()[question_idx].get_question() }
        else
            { 0 }
    }

    // pub fn get_length_of_shuffled_questions(&self, student_idx: usize) -> Option<ShuffledQSet>
    ///
    /// # Arguments
    /// * `student_idx` - 0-based index.
    /// 
    /// # Returns
    /// The number of questions of the `student_idx`-th student
    #[inline]
    pub fn get_length_of_shuffled_questions(&self, student_idx: usize) -> usize
    {
        if student_idx >= self.sbank.len()
            { 0 }
        else
            { self.shuffled_qsets[student_idx].get_shuffled_questions().len() }
    }

    // pub fn get_student(&self, student_idx: usize) -> Option<Student>
    /// 
    /// # Arguments
    /// * `student_idx` - 0-based index.
    /// 
    /// # Returns
    /// * `Option<Student>` - `student_idx`-th student
    ///   if `student_idx` is less than the number of students
    /// * None, otherwise
    #[inline]
    pub fn get_student(&self, student_idx: usize) -> Option<Student>
    {
        if student_idx < self.sbank.len()
            { Some(self.sbank[student_idx].clone()) }
        else
            { None }
    }

    // pub fn get_sbank_length(&self) -> usize
    /// Gets the number of students
    /// 
    /// # Returns
    /// The number of students
    #[inline]
    pub fn get_sbank_length(&self) -> usize
    {
        self.sbank.len()
    }


}
