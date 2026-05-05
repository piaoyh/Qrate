// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::collections::HashMap;

use cryptocol::random::Random_PRNG_Creator;

use crate::{ QBank, Student, ShuffledQuestion };

/// A type alias for a vector of `ShuffledQuestion`s, representing a set of shuffled questions.
pub type ShuffledQuestions = Vec<ShuffledQuestion>;

/// A type alias for a vector of `ShuffledQSet`s, representing a collection of exam sets for multiple students.
pub type ShuffledQSets = Vec<ShuffledQSet>;


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
        if (start == 0) || (start > end) || (selected == 0)
            { return None }

        // Filter questions by range
        let questions_in_range: Vec<crate::Question> = qbank.get_questions()
            .iter()
            .filter(|q| q.get_id() >= start && q.get_id() <= end)
            .cloned() // Clone to get owned Question objects
            .collect();

        if questions_in_range.is_empty()
            { return None; }

        // Group questions by group id
        let mut grouped_questions: HashMap<u16, Vec<crate::Question>> = HashMap::new();
        for question in questions_in_range
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
        for last in (1..max).rev()
        {
            let chosen = prng.random_under_uint_(last + 1);
            (self.questions[last], self.questions[chosen]) = (self.questions[chosen].clone(), self.questions[last].clone());
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

    // pub fn from_parts(student: Student, questions: ShuffledQuestions) -> Self
    /// Creates a new `ShuffledQSet` from a `Student` and a pre-selected set of `ShuffledQuestions`.
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