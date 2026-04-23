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
        if (question_number <= self.questions.len()) && question_number > 0
        {
            let question = self.get_question_mut(question_number).unwrap();
            let choice_length = question.get_choices().len();
            if (choice_number <= choice_length) && choice_number > 0
                { return question.set_choice(choice_number, choice_answer); }
        }
        false
    }

    #[inline]
    pub fn get_max_choices(&self) -> usize
    {
        self.get_questions().iter().map(|q| q.get_choices().len()).max().unwrap_or(0)
    }

    #[inline]
    pub fn get_length(&self) -> usize
    {
        self.get_questions().len()
    }

    #[inline]
    pub fn get_choices_length(&self, question_number: usize) -> usize
    {
        match self.get_question(question_number)
        {
            Some(question) => question.get_choices().len(),
            None => 0
        }
    }
}
