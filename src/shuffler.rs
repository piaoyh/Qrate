// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use cryptocol::random::{ Random_PRNG_Creator, Random };
use crate::{ Header, Student, QBank, SBank, ShuffledQSet, ShuffledQSets };


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
    //  pub fn new(qbank: &QBank, start: u16, end: u16, sbank: &SBank) -> Self
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
            prng: Random_PRNG_Creator::create(),
        };
        me.qbank.optimize();
        me.sbank.optimize();
        me
    }

    // pub fn new_with_seeds(qbank: &QBank, start: u16, end: u16, sbank: &SBank, seeds:[u64; 16]) -> Self
    /// Creates a new `Shuffler` instance
    /// with the provided question bank and student bank.
    /// 
    /// # Arguments
    /// * `qbank` - The question bank containing the questions to be shuffled.
    /// * `start` - The 1-based starting index of questions to consider from the `QBank`.
    /// * `end` - The 1-based ending index of questions to consider from the `QBank`.
    /// * `sbank` - The student bank containing the student information
    ///   for whom the exams will be generated.
    /// * `seeds` - A seed array, each element of which is of u64.
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
    /// let seeds = [0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    /// let shuffler = Shuffler::new_with_seeds(qbank, 1, 2, sbank, seeds);
    /// assert!(shuffler.shuffled_qsets.is_empty());
    /// ```
    pub fn new_with_seeds(qbank: &QBank, start: u16, end: u16, sbank: &SBank, seeds:[u64; 16]) -> Self
    {
        let mut seed = [0u64; 8];
        let mut aux = [0u64; 8];
        seed.clone_from_slice(&seeds[0..8]);
        aux.clone_from_slice(&seeds[8..16]);
        let mut me = Self {
            qbank: qbank.select_questions(start, end),
            sbank: sbank.clone(),
            shuffled_qsets: Vec::new(),
            prng: Random_PRNG_Creator::create_with_seed_arrays(seed, aux),
        };
        me.qbank.optimize();
        me.sbank.optimize();
        me
    }

    // pub fn make_exams(&mut self, number_of_questions: usize) -> bool
    /// Generates shuffled question sets for all students in the student bank.
    /// 
    /// # Arguments
    /// * `number_of_questions` - The number of questions for each student's exam.
    /// 
    /// # Returns
    /// - `true` if succeeded.
    /// - `false` if failed.
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
    /// assert!(exams);
    /// ```
    pub fn make_exams(&mut self, number_of_questions: usize) -> bool
    {
        self.shuffled_qsets.clear();
        for student in self.sbank.get_students().clone()
        {
            if let Some(qset) = self.create_shuffled_qset(student, number_of_questions as usize)
            {
                self.shuffled_qsets.push(qset);
            }
            else
            {
                self.shuffled_qsets.clear();
                return false;
            }
        }
        true
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

    // pub fn get_qbank(&self) -> &QBank
    /// Gets a reference to the question bank used by the shuffler.
    /// 
    /// # Returns
    /// A reference to the `QBank` instance containing the questions.
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
    /// sbank.push(Student::new("Alice", "1"));
    /// sbank.push(Student::new("Bob", "2"));
    /// sbank.push(Student::new("Caleb", "3"));
    /// let shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// let qbank_ref = shuffler.get_qbank();
    /// assert_eq!(qbank_ref.get_length(), 3);
    /// ```
    #[inline]
    pub fn get_qbank(&self) -> &QBank
    {
        &self.qbank
    }

    // pub fn get_sbank(&self) -> &SBank
    /// Gets a reference to the student bank used by the shuffler.
    /// 
    /// # Returns
    /// A reference to the `SBank` instance containing the students.
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
    /// sbank.push(Student::new("Alice", "1"));
    /// sbank.push(Student::new("Bob", "2"));
    /// sbank.push(Student::new("Caleb", "3"));
    /// let shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// let sbank_ref = shuffler.get_sbank();
    /// assert_eq!(sbank_ref.get_length(), 3);
    /// ```
    #[inline]
    pub fn get_sbank(&self) -> &SBank
    {
        &self.sbank
    }

    // pub fn get_shuffled_qsets(&self) -> &ShuffledQSets
    /// Gets a reference to the shuffled question sets generated by the shuffler.
    /// 
    /// # Returns
    /// A reference to the `ShuffledQSets` instance containing the shuffled question sets for each student.
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
    /// let mut shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// shuffler.make_exams(2);
    /// let shuffled_qsets_ref = shuffler.get_shuffled_qsets();
    /// assert_eq!(shuffled_qsets_ref.len(), 3);
    /// ```
    #[inline]
    pub fn get_shuffled_qsets(&self) -> &ShuffledQSets
    {
        &self.shuffled_qsets
    }

    // pub fn get_prng(&self) -> &Random
    /// Gets a reference to the pseudo-random number generator (PRNG) used by the shuffler.
    /// 
    /// # Returns
    /// A reference to the `Random` instance representing the PRNG used for shuffling questions and choices.
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
    /// sbank.push(Student::new("Alice", "1"));
    /// sbank.push(Student::new("Bob", "2"));
    /// sbank.push(Student::new("Caleb", "3"));
    /// let shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// let prng_ref = shuffler.get_prng();
    /// assert!(prng_ref.is_valid());
    /// ```
    #[inline]
    pub fn get_prng(&self) -> &Random
    {
        &self.prng
    }

    // pub fn get_qbank_length(&self) -> usize
    /// Gets the number of questions in the question bank used by the shuffler.
    /// 
    /// # Returns
    /// The number of questions in the `QBank` instance.
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
     /// sbank.push(Student::new("Alice", "1"));
     /// sbank.push(Student::new("Bob", "2"));
     /// sbank.push(Student::new("Caleb", "3"));
     /// let shuffler = Shuffler::new(qbank, 1, 3, sbank);
     /// let qbank_length = shuffler.get_qbank_length();
     /// assert_eq!(qbank_length, 3);
     /// ```
    #[inline]
    pub fn get_qbank_length(&self) -> usize
    {
        self.qbank.get_length()
    }

    // pub fn get_header(&self) -> &Header
    /// Gets a reference to the header information of the question bank used by the shuffler.
    /// 
    /// # Returns
    /// A reference to the `Header` instance containing the header information of the question bank.
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
     /// sbank.push(Student::new("Alice", "1"));
     /// sbank.push(Student::new("Bob", "2"));
     /// sbank.push(Student::new("Caleb", "3"));
     /// let shuffler = Shuffler::new(qbank, 1, 3, sbank);
     /// let header_ref = shuffler.get_header();
     /// assert_eq!(header_ref.get_total_questions(), 3);
     /// ```
    #[inline]
    pub fn get_header(&self) -> &Header
    {
        self.qbank.get_header()
    }

    // pub fn get_shuffled_questions(&self, student_idx: usize) -> Option<ShuffledQSet>
    ///
    /// # Arguments
    /// * `student_idx` - 0-based index.
    /// 
    /// # Returns
    /// * `Option<ShuffledQSet>` - Shuffled question set for the `student_idx`-th student
    ///   if `student_idx` is less than the number of students
    /// * None, otherwise
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
    /// let mut shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// shuffler.make_exams(2);
    /// let qset = shuffler.get_shuffled_questions(1).unwrap();
    /// assert_eq!(qset.get_shuffled_questions().len(), 2);
    /// ```
    #[inline]
    pub fn get_shuffled_questions(&self, student_idx: usize) -> Option<ShuffledQSet>
    {
        if student_idx >= self.sbank.get_length() || student_idx >= self.shuffled_qsets.len()
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
    /// let mut shuffler = Shuffler::new(qbank, 1, 3, sbank);
    /// shuffler.make_exams(2);
    /// let question_id = shuffler.get_shuffled_question(1, 0);
    /// assert!(question_id > 0);
    /// ```
    #[inline]
    pub fn get_shuffled_question(&self, student_idx: usize, question_idx: usize) -> u16
    {
        if student_idx < self.sbank.get_length() && question_idx < self.shuffled_qsets[student_idx].get_shuffled_questions().len()
            { self.shuffled_qsets[student_idx].get_shuffled_questions()[question_idx].get_question() }
        else
            { 0 }
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
    /// let student = shuffler.get_student(1).unwrap();
    /// assert_eq!(student.get_name(), "Bob");
    /// assert_eq!(student.get_id(), "2");
    /// ```
    #[inline]
    pub fn get_student(&self, student_idx: usize) -> Option<Student>
    {
        if student_idx < self.sbank.get_length()
            { Some(self.sbank.get_student(student_idx).unwrap()) }
        else
            { None }
    }

    // pub fn get_sbank_length(&self) -> usize
    /// Gets the number of students
    /// 
    /// # Returns
    /// The number of students
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
    /// let sbank_length = shuffler.get_sbank_length();
    /// assert_eq!(sbank_length, 3);
    /// ```
    #[inline]
    pub fn get_sbank_length(&self) -> usize
    {
        self.sbank.get_length()
    }
}
