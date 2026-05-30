// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::collections::HashMap;

use cryptocol::random::Random;

use crate::{ QBank, Student, ShuffledQuestion };

/// A type alias for a vector of `ShuffledQuestion`s, representing a set of shuffled questions.
pub type ShuffledQuestions = Vec<ShuffledQuestion>;

/// A type alias for a vector of `ShuffledQSet`s, representing a collection of exam sets for multiple students.
pub type ShuffledQSets = Vec<ShuffledQSet>;


/// Represents a complete set of shuffled questions for a single student.
/// 
/// The `ShuffledQSet` struct contains a `Student` object representing the student,
/// and a vector of `ShuffledQuestion` objects representing the shuffled questions.
#[derive(Debug, Clone)]
pub struct ShuffledQSet
{
    student: Student,
    questions: ShuffledQuestions,
}

impl ShuffledQSet
{
    // pub fn new(qbank: &QBank, number_of_questions: usize, student: Student, prng: &mut Random) -> Option<Self>
    /// Creates a new set of shuffled questions for a student by randomly
    /// selecting a specified number of questions from a `QBank` within a given
    /// range. Each selected question will belong to a unique group. The choices
    /// for each question are shuffled upon creation.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` to draw questions from,
    ///   and should be optimized by the method optimize().
    /// * `number_of_questions` - The number of questions to randomly select.
    ///   Each selected question will have a unique group ID.
    /// * `student` - The `Student` for whom this question set is,
    ///   and should be optimized by the method optimize().
    /// * `prng` - A random number generator.
    ///
    /// # Returns
    /// `Option<Self>` - A new `ShuffledQSet` instance, or `None`
    /// if `qbank` is empty,
    /// if `number_of_questions` is 0, or
    /// if the number of available unique question groups is
    /// less than `number_of_questions`.
    ///
    /// # Examples
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator;
    /// use qrate::{QBank, Student, Question, shuffler::ShuffledQSet};
    /// 
    /// let mut rand = Random_PRNG_Creator::create();
    /// let mut qbank = QBank::new_with_default();
    /// // Add questions with specific groups.
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![]));
    /// qbank.push_question(Question::new(2, 1, 1, "Q2".to_string(), vec![]));
    /// qbank.push_question(Question::new(3, 2, 1, "Q3".to_string(), vec![]));
    /// qbank.push_question(Question::new(4, 3, 1, "Q4".to_string(), vec![]));
    /// qbank.push_question(Question::new(5, 4, 1, "Q5".to_string(), vec![]));
    ///
    /// let student = Student::new("Test".to_string(), "123".to_string());
    /// // Select 3 questions from unique groups within the bank.
    /// let qset = ShuffledQSet::new(&qbank, 3, student.clone(), &mut rand).unwrap();
    /// assert_eq!(qset.get_student().get_name(), "Test");
    /// assert_eq!(qset.get_shuffled_questions().len(), 3);
    ///
    /// // Try to select more than available unique groups (4 groups total).
    /// let qset_none = ShuffledQSet::new(&qbank, 5, student.clone(), &mut rand);
    /// assert!(qset_none.is_none());
    ///
    /// // Selection count is 0.
    /// let qset_zero = ShuffledQSet::new(&qbank, 0, student, &mut rand);
    /// assert!(qset_zero.is_none());
    /// ```
    pub fn new(qbank: &QBank, number_of_questions: usize, student: Student, prng: &mut Random) -> Option<Self>
    {
        let number_of_groups = qbank.get_number_of_groups();
        if (qbank.get_length() == 0) || (number_of_questions > number_of_groups) || (number_of_questions == 0)
            { return None }

        let mut groups: HashMap<u16, Vec<usize>> = HashMap::new();
        for (idx, q) in qbank.get_questions().iter().enumerate()
            { groups.entry(q.get_group()).or_insert_with(Vec::new).push(idx); }

        let mut group_ids: Vec<u16> = groups.keys().cloned().collect();
        for i in (1..number_of_groups).rev()
        {
            let j = prng.random_under_uint_(i + 1);
            group_ids.swap(i, j);
        }

        let mut questions = ShuffledQuestions::new();
        for i in 0..number_of_questions
        {
            let group_id = group_ids[i];
            let question_indices = &groups[&group_id];
            let chosen_idx = question_indices[prng.random_under_uint_(question_indices.len())];
            let question = qbank.get_question(chosen_idx + 1).unwrap();
            let mut shuffled_question = ShuffledQuestion::new((chosen_idx + 1) as u16, question.get_number_of_choices() as u8);
            shuffled_question.shuffle(prng);
            questions.push(shuffled_question);
        }
        let mut me = Self { student, questions };
        me.shuffle(prng);
        Some(me)
    }

    // pub fn shuffle(&mut self, prng: &mut Random)
    /// Shuffles the order of the questions within the set.
    /// 
    /// # Arguments
    /// * `prng` - A random number generator.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator;
    /// use qrate::{ QBank, Student, ShuffledQSet, Question };
    /// 
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![])); // Add a question
    /// qbank.push_question(Question::new(2, 1, 1, "Q2".to_string(), vec![]));
    /// qbank.push_question(Question::new(3, 1, 1, "Q3".to_string(), vec![]));
    /// qbank.push_question(Question::new(4, 1, 1, "Q4".to_string(), vec![]));
    /// qbank.push_question(Question::new(5, 1, 1, "Q5".to_string(), vec![]));
    /// let student = Student::new("Test".to_string(), "123".to_string());
    /// let mut qset = ShuffledQSet::new(&qbank, &student, 1, 5).unwrap();
    /// let original_order: Vec<u16> = qset.get_shuffled_questions().iter().map(|q| q.get_question()).collect();
    /// qset.shuffle(Random_PRNG_Creator::create());
    /// let shuffled_order: Vec<u16> = qset.get_shuffled_questions().iter().map(|q| q.get_question()).collect();
    /// assert_eq!(original_order.len(), shuffled_order.len());
    /// ```
    pub fn shuffle(&mut self, prng: &mut Random)
    {
        let max = self.questions.len();
        for last in (1..max).rev()
        {
            let chosen = prng.random_under_uint_(last + 1);
            (self.questions[last], self.questions[chosen]) = (self.questions[chosen].clone(), self.questions[last].clone());
        }
    }

    // pub fn get_student(&self) -> &Student
    /// Gets a reference to the `Student` associated with this question set.
    /// 
    /// # Returns
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

    // pub fn set_student(&mut self, student: Student)
    /// Sets the `Student` for this question set.
    /// 
    /// # Arguments
    /// * `student` - A new `Student` object.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{QBank, Student, shuffler::ShuffledQSet};
    /// use qrate::Question;
    /// let mut qbank = QBank::new_with_default();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![])); // Add a question
    /// let mut student1 = Student::new("Test1".to_string(), "123".to_string());
    /// let student2 = Student::new("Test2".to_string(), "456".to_string());
    /// let mut qset = ShuffledQSet::new(qbank, student1, 1, 1).unwrap();
    /// qset.set_student(student2);
    /// assert_eq!(qset.get_student().get_name(), "Test2");
    /// ```
    #[inline]
    pub fn set_student(&mut self, student: Student)
    {
        self.student = student;
    }

    // pub fn get_shuffled_questions(&self) -> &ShuffledQuestions
    /// Gets a reference to the shuffled questions.
    /// 
    /// # Returns
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

    // pub fn get_shuffled_questions_mut(&mut self) -> &mut ShuffledQuestions
    /// Gets a mutable reference to the shuffled questions.
    /// 
    /// # Returns
    /// `&mut ShuffledQuestions` - A mutable reference to the vector of `ShuffledQuestion`s.
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
    /// let mut qset = ShuffledQSet::new(&qbank, &student, 1, 3).unwrap();
    /// let questions_mut = qset.get_shuffled_questions_mut();
    /// assert_eq!(questions_mut.len(), 3);
    /// questions_mut.push(ShuffledQuestion::new(4, 4)); // Add a new question
    /// assert_eq!(qset.get_shuffled_questions().len(), 4);
    /// ```
    #[inline]
    pub fn get_shuffled_questions_mut(&mut self) -> &mut ShuffledQuestions
    {
        &mut self.questions
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
    /// # Returns
    /// `Option<&ShuffledQuestion>` - An `Option` containing a reference to the
    /// `ShuffledQuestion` if found, or
    /// `None` if the `question_number` is invalid (e.g., 0).
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
        if (question_number == 0) || (question_number > self.questions.len() as u16)
            { None }
        else
            { Some(&self.questions[question_number as usize - 1]) }
    }

    // pub fn from_parts(student: Student, questions: ShuffledQuestions) -> Self
    /// Creates a new `ShuffledQSet` from a `Student` and
    /// a pre-selected set of `ShuffledQuestions`.
    /// 
    /// # Arguments
    /// * `student` - The `Student` for whom this question set is.
    /// * `questions` - The pre-selected and potentially shuffled questions.
    /// 
    /// # Returns
    /// `Self` - A new `ShuffledQSet` instance.
    #[inline]
    pub fn from_parts(student: Student, questions: ShuffledQuestions) -> Self
    {
        Self { student, questions }
    }
}