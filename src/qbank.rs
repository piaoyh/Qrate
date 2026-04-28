// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::ChoiceAnswer;
use crate::Header;
use crate::Question;

/// Represents a Question Bank, containing a header and a vector of questions.
#[derive(Debug, Clone)]
pub struct QBank
{
    header: Header,
    questions: Vec<Question>,
}

impl QBank
{
    // pub fn new_empty() -> Self
    /// Creates a new, empty `QBank` with an empty header.
    ///
    /// # Output
    /// `Self` - A new, empty `QBank` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_empty();
    /// assert!(qbank.get_questions().is_empty());
    /// ```
    #[inline]
    pub fn new_empty() -> Self
    {
        QBank
        {
            header: Header::new_empty(),
            questions: Vec::new(),
        }
    }

    // pub fn new_with_default() -> Self
    /// Creates a new `QBank` with a default header.
    ///
    /// # Output
    /// `Self` - A new `QBank` instance with a default header.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_with_default();
    /// assert_eq!(qbank.get_header().get_title(), "Examination");
    /// ```
    #[inline]
    pub fn new_with_default() -> Self
    {
        QBank
        {
            header: Header::new_with_default(),
            questions: Vec::new(),
        }
    }

    // pub fn new_with_header(header: Header) -> Self
    /// Creates a new `QBank` with a provided `Header`.
    ///
    /// # Arguments
    /// * `header` - The `Header` to be used for the new `QBank`.
    ///
    /// # Output
    /// `Self` - A new `QBank` instance with the specified header.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Header };
    /// let custom_header = Header::new_empty();
    /// let qbank = QBank::new_with_header(custom_header);
    /// assert!(qbank.get_questions().is_empty());
    /// ```
    #[inline]
    pub fn new_with_header(header: Header) -> Self
    {
        QBank
        {
            header,
            questions: Vec::new(),
        }
    }

    // pub fn get_header(&self) -> &Header
    /// Gets a reference to the `Header`.
    ///
    /// # Output
    /// `&Header` - A reference to the `Header` of the question bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_with_default();
    /// assert_eq!(qbank.get_header().get_title(), "Examination");
    /// ```
    #[inline]
    pub fn get_header(&self) -> &Header
    {
        &self.header
    }

    // pub fn set_header(&mut self, header: Header)
    /// Sets the `Header`.
    ///
    /// # Arguments
    /// * `header` - The new `Header` to set for the question bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Header };
    /// let mut qbank = QBank::new_empty();
    /// let mut new_header = Header::new_empty();
    /// new_header.set_title("My Custom Exam".to_string());
    /// qbank.set_header(new_header);
    /// assert_eq!(qbank.get_header().get_title(), "My Custom Exam");
    /// ```
    #[inline]
    pub fn set_header(&mut self, header: Header)
    {
        self.header = header;
    }

    // pub fn get_questions(&self) -> &Vec<Question>
    /// Gets a reference to the vector of `Question`s.
    ///
    /// # Output
    /// `&Vec<Question>` - A reference to the vector of `Question`s in the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new_empty());
    /// assert_eq!(qbank.get_questions().len(), 1);
    /// ```
    #[inline]
    pub fn get_questions(&self) -> &Vec<Question>
    {
        &self.questions
    }

    // pub fn set_questions(&mut self, questions: Vec<Question>)
    /// Sets the vector of `Question`s.
    ///
    /// # Arguments
    /// * `questions` - The new vector of `Question`s to set for the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.set_questions(vec![Question::new_empty(), Question::new_empty()]);
    /// assert_eq!(qbank.get_questions().len(), 2);
    /// ```
    #[inline]
    pub fn set_questions(&mut self, questions: Vec<Question>)
    {
        self.questions = questions;
    }

    // pub fn get_question(&self, question_number: usize) -> Option<&Question>
    /// Gets a reference to a `Question` by its 1-based index.
    ///
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to retrieve.
    ///
    /// # Output
    /// `Option<&Question>` - An optional reference to the `Question` at the specified index.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Test Q".to_string(), vec![]));
    /// assert_eq!(qbank.get_question(1).unwrap().get_id(), 1);
    /// assert!(qbank.get_question(2).is_none());
    /// ```
    pub fn get_question(&self, question_number: usize) -> Option<&Question>
    {
        if (question_number <= self.questions.len()) && question_number > 0
            { Some(&self.questions[question_number - 1]) }
        else
            { None }
    }

    // pub fn get_question_mut(&mut self, question_number: usize) -> Option<&mut Question>
    /// Gets a mutable reference to a `Question` by its 1-based index.
    ///
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to retrieve.
    ///
    /// # Returns
    /// An optional mutable reference to the `Question` at the specified index.
    pub fn get_question_mut(&mut self, question_number: usize) -> Option<&mut Question>
    {
        if (question_number <= self.questions.len()) && question_number > 0
            { Some(&mut self.questions[question_number - 1]) }
        else
            { None }
    }

    // pub fn push_question(&mut self, question: Question)
    /// Adds a `Question` to the bank.
    ///
    /// # Arguments
    /// * `question` - The `Question` to add to the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// let question = Question::new_empty();
    /// qbank.push_question(question);
    /// assert_eq!(qbank.get_questions().len(), 1);
    /// ```
    #[inline]
    pub fn push_question(&mut self, question: Question)
    {
        self.questions.push(question);
    }

    // pub fn get_choice(&self, question_number: usize, choice_number: usize) -> Option<&ChoiceAnswer>
    /// Gets a reference to a choice `ChoiceAnswer` by question number and choice number (both 1-based).
    ///
    /// # Arguments
    /// * `question_number` - The 1-based index of the question.
    /// * `choice_number` - The 1-based index of the choice within the question.
    ///
    /// # Output
    /// `Option<&ChoiceAnswer>` - An optional reference to the `ChoiceAnswer` at the specified index.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// let question = Question::new(1, 1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]);
    /// qbank.push_question(question);
    /// assert_eq!(qbank.get_choice(1, 1).unwrap().0, "Choice A");
    /// assert_eq!(qbank.get_choice(1, 1).unwrap().1, false);
    /// assert!(qbank.get_choice(1, 3).is_none());
    /// assert!(qbank.get_choice(2, 1).is_none());
    /// ```
    pub fn get_choice(&self, question_number: usize, choice_number: usize) -> Option<&ChoiceAnswer>
    {
        if (question_number <= self.questions.len()) && question_number > 0
        {
            let question = self.get_question(question_number)?;
            let choice_length = question.get_choices().len();
            if (choice_number <= choice_length) && choice_number > 0
                { return question.get_choice(choice_number); }
        }
        None
    }

    // pub fn push_choice(&mut self, question_number: usize, choice: ChoiceAnswer) -> bool
    /// Adds a `ChoiceAnswer` to a specific question by its 1-based index.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to which the choice will be added.
    /// * `choice` - The `ChoiceAnswer` to add to the specified question.
    /// 
    /// # Returns
    /// * `true` if the choice was successfully added to the question.
    /// * `false` if the question number is out of bounds.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// let question = Question::new(1, 1, 1, "Q1".to_string(), vec![]);
    /// qbank.push_question(question);
    /// assert_eq!(qbank.push_choice(1, ChoiceAnswer::new("Choice A".to_string(), false)), true);
    /// assert_eq!(qbank.push_choice(2, ChoiceAnswer::new("Choice B".to_string(), false)), false);
    /// ```
    pub fn push_choice(&mut self, question_number: usize, choice: ChoiceAnswer) -> bool
    {
        if let Some(question) = self.get_question_mut(question_number)
        {
            question.push_choice(choice);
            return true;
        }
        false
    }

    // pub fn set_choice(&mut self, question_number: usize, choice_number: usize, choice_answer: ChoiceAnswer) -> bool
    /// Sets or changes `choice_number`-th choice of the question of `question_number`.
    /// 
    /// # Arguments
    /// * `question_number` - Is the (1-based) question number of the question
    ///   that this choice belongs to, and is of `unsize` type.
    /// * `choice_number` - Is the (1-based) choice number of the choice
    ///   that you want to set or change, and is of `unsize` type.
    /// * `choice_answer` - Is a pair of choice sentence (String) and whether
    ///   or not it is answer (bool), and is of `ChoiceAnswer` type.
    ///   If it is an answer, it is `true`. Otherwise, `false`.
    /// 
    /// # Returns
    /// * `true` if it succeeded to set or change.
    /// * `false` if `question_number` is `0` or greater than the total number
    ///   of questions.
    /// 
    pub fn set_choice(&mut self, question_number: usize, choice_number: usize, choice_answer: ChoiceAnswer) -> bool
    {
        if let Some(question) = self.get_question_mut(question_number)
            { question.set_choice(choice_number, choice_answer) }
        else
            { false }
    }

    // pub fn get_max_choices(&self) -> usize
    /// Gets the maximum number of choices among all questions in the bank.
    /// 
    /// # Returns
    /// The maximum number of choices among all questions in the bank as `usize`.
    /// If there are no questions, returns `0`.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]));
    /// qbank.push_question(Question::new(2, 1, 1, "Q2".to_string(), vec![("Choice A".to_string(), false)]));
    /// assert_eq!(qbank.get_max_choices(), 2);
    /// ```
    #[inline]
    pub fn get_max_choices(&self) -> usize
    {
        self.get_questions().iter().map(|q| q.get_choices().len()).max().unwrap_or(0)
    }

    // pub fn get_length(&self) -> usize
    /// Gets the total number of questions in the bank.
    ///
    /// # Returns
    /// The total number of questions in the bank as `usize`.
    /// If there are no questions, returns `0`.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new_empty());
    /// qbank.push_question(Question::new_empty());
    /// assert_eq!(qbank.get_length(), 2);
    /// ```
    #[inline]
    pub fn get_length(&self) -> usize
    {
        self.get_questions().len()
    }

    // pub fn get_choices_length(&self, question_number: usize) -> usize
    /// Gets the number of choices for a specific question by its 1-based index.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to check.
    /// 
    /// # Returns
    /// The number of choices for the specified question as `usize`.
    /// If the question number is out of bounds, returns `0`.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]));
    /// assert_eq!(qbank.get_choices_length(1), 2);
    /// assert_eq!(qbank.get_choices_length(2), 0);
    /// ```
    pub fn get_choices_length(&self, question_number: usize) -> usize
    {
        match self.get_question(question_number)
        {
            Some(question) => question.get_choices().len(),
            None => 0
        }
    }

    // pub fn get_group(&self, question_number: usize) -> u16
    /// Gets the group number of a specific question by its 1-based index.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to check.
    /// 
    /// # Returns
    /// The group number of the specified question as `u16`.
    /// If the question number is out of bounds, returns `0`.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]));
    /// assert_eq!(qbank.get_group(1), 1);
    /// assert_eq!(qbank.get_group(2), 0);
    /// ```
    pub fn get_group(&self, question_number: usize) -> u16
    {
        match self.get_question(question_number)
        {
            Some(question) => question.get_group(),
            None => 0
        }
    }

    // pub fn set_group(&mut self, question_number: usize, group: u16) -> bool
    /// Sets the group number of a specific question by its 1-based index.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to modify.
    /// * `group` - The new group number to set for the specified question.
    /// 
    /// # Returns
    /// * `true` if the group number was successfully set.
    /// * `false` if the question number is out of bounds.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]));
    /// assert!(qbank.set_group(1, 2));
    /// assert_eq!(qbank.get_group(1), 2);
    /// assert!(!qbank.set_group(2, 1));
    /// ```
    pub fn set_group(&mut self, question_number: usize, group: u16) -> bool
    {
        if let  Some(question) =  self.get_question_mut(question_number)
        {
            question.set_group(group);
            true
        }
        else
        {
            false
        }
    }

    // pub fn get_category(&self, question_number: usize) -> u8
    /// Gets the category of a specific question by its 1-based index.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to check.
    /// 
    /// # Returns
    /// The category of the specified question as `u8`.
    /// If the question number is out of bounds, returns `0`.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]));
    /// assert_eq!(qbank.get_category(1), 1);
    /// assert_eq!(qbank.get_category(2), 0);
    /// ```
    pub fn get_category(&self, question_number: usize) -> u8
    {
        match self.get_question(question_number)
        {
            Some(question) => question.get_category(),
            None => 0
        }
    }

    // pub fn set_category(&mut self, question_number: usize, category: u8) -> bool
    /// Sets the category of a specific question by its 1-based index.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to modify.
    /// * `category` - The new category to set for the specified question.
    /// 
    /// # Returns
    /// * `true` if the category was successfully set.
    /// * `false` if the question number is out of bounds.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]));
    /// assert!(qbank.set_category(1, 2));
    /// assert_eq!(qbank.get_category(1), 2);
    /// assert!(!qbank.set_category(2, 1));
    /// ```
    pub fn set_category(&mut self, question_number: usize, category: u8) -> bool
    {
        if let Some(question) = self.get_question_mut(question_number)
        {
            question.set_category(category);
            true
        }
        else
        {
            false
        }
    }

    // pub fn determine_category(&mut self, question_number: usize) -> bool
    /// Determines the category of the question based on the number of choices
    /// and their correctness.
    /// 
    /// The category is set as follows:
    /// - If there are no choices, the category is set to 4 (essay).
    /// - If there is one choice and the choice answer mark is false,
    ///   the category is set to 4.
    /// - If there is one choice and the choice answer mark is true,
    ///   the category is set to 3.
    /// - If there is multiple choices and there is multiple correct choices
    ///   less than the total number of choices, the category is set to 2.
    /// - If there is multiple choices and there is multiple correct choices
    ///   equal to the total number of choices, the category is set to 3.
    /// - If there is multiple choices and there is exactly one correct choice,
    ///   the category is set to 1.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to determine
    ///   the category for.
    /// 
    /// # Returns
    /// * `true` if the category was successfully determined and set.
    /// * `false` if the question number is out of bounds.
    /// 
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Q1".to_string(), vec![]));
    /// 
    /// qbank.determine_category(1);
    /// assert_eq!(qbank.get_category(1), 4); // No choices, so category is 4 (essay)
    /// 
    /// qbank.push_question(Question::new(1, 1, 1, "Q2".to_string(), vec![("Option A".to_string(), false)]));
    /// qbank.determine_category(2);
    /// assert_eq!(qbank.get_category(2), 4); // One incorrect choice, so category is 4 (essay)
    /// 
    /// qbank.set_choice(2, 1, ("Option B".to_string(), true));
    /// qbank.determine_category(2);
    /// assert_eq!(qbank.get_category(2), 3); // One correct choice, so category is 3 (short answer)
    /// 
    /// qbank.push_choice(2, ("Option C".to_string(), false));
    /// qbank.determine_category(2);
    /// assert_eq!(qbank.get_category(2), 1); // Multiple choices with one correct, so category is 1 (single answer of multiple-choice)
    /// 
    /// qbank.push_choice(2, ("Option D".to_string(), true));
    /// qbank.determine_category(2);
    /// assert_eq!(qbank.get_category(2), 2); // Multiple choices with multiple correct, so category is 2 (multiple answers of multiple-choice)
    /// 
    /// qbank.set_choice(2, 1, ("Option A".to_string(), true));
    /// qbank.set_choice(2, 2, ("Option B".to_string(), true));
    /// qbank.set_choice(2, 3, ("Option C".to_string(), true));
    /// qbank.set_choice(2, 4, ("Option D".to_string(), true));
    /// qbank.determine_category(2);
    /// assert_eq!(qbank.get_category(2), 3); // Multiple choices with all correct, so category is 3 (short answer)
    /// ```
    pub fn determine_category(&mut self, question_number: usize) -> bool
    {
        if let Some(question) = self.get_question_mut(question_number)
        {
            question.determine_category();
            true
        }
        else
        {
            false
        }
    }
}
