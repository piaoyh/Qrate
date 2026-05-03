// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


pub type ChoiceAnswer = (String, bool);
pub type Choices = Vec<ChoiceAnswer>;
pub type Questions = Vec<Question>;

/// Represents a single question with its properties.
#[derive(Debug, Clone)]
pub struct Question
{
    id: u16,        // 1-based unique identifier. Should be in order as class progress
    group: u16,     // The questions that belong to the same group will not appear in an exam set.
    category: u8,   // 1-based category: 1 for single answer of multiple-choice, 2 for mutiple answers of multiple-choice, 3 for short answer, and 4 for essay.
    question: String,   // The text of the question
    choices: Choices,   // For categories 3 and 4, all choices.1 is set to true.
}

impl Question
{
    // pub fn new_empty() -> Self
    /// Creates a new, empty `Question`.
    ///
    /// # Returns
    /// `Self` - A new, empty `Question` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new_empty();
    /// assert_eq!(question.get_id(), 1); // Changed from 0 to 1
    /// assert_eq!(question.get_question(), "");
    /// ```
    #[inline]
    pub fn new_empty() -> Self
    {
        Self
        {
            id: 1,
            group: 1,
            category: 1,
            question: String::new(),
            choices: Choices::new(),
        }
    }

    // pub fn new(id: u16, category: u8, question: String, choices: Choices) -> Self
    /// Creates a new `Question` instance with the given properties.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the question (1-based).
    /// * `group` - The questions that belong to the same group will not appear in an exam set (1-based).
    /// * `category` - The category of the question (1-based)
    ///   -- 1: single answer of multiple-choice
    ///   -- 2: multiple answers of multiple-choice
    ///   -- 3: short answer
    ///   -- 4: essay
    /// * `question` - The text of the question.
    /// * `choices` - A vector of `ChoiceAnswer` tuples for the question.
    ///
    /// # Returns
    /// `Self` - A new `Question` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::{Question, Choices};
    /// let question = Question::new(1, 1, 1, "What is Rust?".to_string(), vec![("A language".to_string(), true)]);
    /// assert_eq!(question.get_id(), 1);
    /// assert_eq!(question.get_question(), "What is Rust?");
    /// ```
    #[inline]
    pub fn new(id: u16, group: u16, category: u8, question: String, choices: Choices) -> Self
    {
        Self { id, group, category, question, choices }
    }

    // pub fn get_id(&self) -> u16
    /// Gets the ID of the question.
    ///
    /// # Returns
    /// `u16` - The ID of the question, 1-based.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new_empty();
    /// assert_eq!(question.get_id(), 1); // Changed from 0 to 1
    /// ```
    #[inline]
    pub fn get_id(&self) -> u16
    {
        self.id
    }

    // pub fn set_id(&mut self, id: u16)
    /// Sets the ID of the question.
    ///
    /// # Arguments
    /// * `id` - The new ID for the question, 1-based.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_id(5);
    /// assert_eq!(question.get_id(), 5);
    /// ```
    #[inline]
    pub fn set_id(&mut self, id: u16)
    {
        self.id = id;
    }

    // pub fn get_group(&self) -> u16
    /// Gets the group of the question.
    ///
    /// # Returns
    /// `u16` - The group of the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new_empty();
    /// assert_eq!(question.get_group(), 1); // Changed from 0 to 1
    /// ```
    #[inline]
    pub fn get_group(&self) -> u16
    {
        self.group
    }

    // pub fn set_group(&mut self, group: u16)
    /// Sets the group of the question.
    ///
    /// # Arguments
    /// * `group` - The new group for the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_group(2); // Multi-choice
    /// assert_eq!(question.get_group(), 2);
    /// ```
    #[inline]
    pub fn set_group(&mut self, group: u16)
    {
        self.group = group;
    }

    // pub fn get_category(&self) -> u8
    /// Gets the category of the question.
    ///
    /// # Returns
    /// `u8` - The category of the question, 1-based.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new_empty();
    /// assert_eq!(question.get_category(), 1);
    /// ```
    #[inline]
    pub fn get_category(&self) -> u8
    {
        self.category
    }

    // pub fn set_category(&mut self, category: u8)
    /// Sets the category of the question.
    ///
    /// # Arguments
    /// * `category` - The new category for the question, 1-based.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_category(2); // Multi-choice
    /// assert_eq!(question.get_category(), 2);
    /// ```
    #[inline]
    pub fn set_category(&mut self, category: u8)
    {
        self.category = category;
    }

    // pub fn get_question(&self) -> &String
    /// Gets a reference to the question text.
    ///
    /// # Returns
    /// `&String` - A reference to the question text.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new(1, 1, 1, "Hello".to_string(), vec![]);
    /// assert_eq!(question.get_question(), "Hello");
    /// ```
    #[inline]
    pub fn get_question(&self) -> &String
    {
        &self.question
    }

    // pub fn set_question(&mut self, question: String)
    /// Sets the question text.
    ///
    /// # Arguments
    /// * `question` - The new text for the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_question("New Question Text".to_string());
    /// assert_eq!(question.get_question(), "New Question Text");
    /// ```
    #[inline]
    pub fn set_question(&mut self, question: String)
    {
        self.question = question;
    }

    // pub fn get_choice(&self, choice_number: usize) -> Option<&ChoiceAnswer>
    /// Gets a reference to a choice by its 1-based index.
    ///
    /// # Arguments
    /// * `choice_number` - The 1-based index of the choice to retrieve.
    ///
    /// # Returns
    /// `Option<&ChoiceAnswer>` - An optional reference to the `ChoiceAnswer` at the specified index.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new(1, 1, 1, "Q".to_string(), vec![("Opt A".to_string(), false), ("Opt B".to_string(), true)]);
    /// assert_eq!(question.get_choice(1).unwrap().0, "Opt A");
    /// assert!(question.get_choice(3).is_none());
    /// ```
    pub fn get_choice(&self, choice_number: usize) -> Option<&ChoiceAnswer>
    {
        if (choice_number <= self.choices.len()) && (choice_number > 0)
            { Some(&self.choices[choice_number - 1]) }
        else
            { None }
    }

    // pub fn set_choice(&mut self, choice_number: usize, choice_answer: ChoiceAnswer) -> bool
    /// Sets a choice at a specific 1-based index.
    ///
    /// # Arguments
    /// * `choice_number` - The 1-based index of the choice to set.
    /// * `choice_answer` - The new `ChoiceAnswer` to set at the specified index.
    ///
    /// # Returns
    /// `bool` - `true` if the choice was successfully set, `false`
    /// if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new(1, 1, 1, "Q".to_string(), vec![("Opt A".to_string(), false)]);
    /// assert!(question.set_choice(1, ("Opt A Updated".to_string(), true)));
    /// assert_eq!(question.get_choice(1).unwrap().0, "Opt A Updated");
    /// assert!(!question.set_choice(2, ("Opt B".to_string(), false))); // Out of bounds
    /// ```
    pub fn set_choice(&mut self, choice_number: usize, choice_answer: ChoiceAnswer) -> bool
    {
        if (choice_number <= self.choices.len()) && (choice_number > 0)
        {
            self.choices[choice_number - 1] = choice_answer;
            true
        }
        else
        {
            false
        }
    }

    // pub fn push_choice(&mut self, choice: ChoiceAnswer)
    /// Adds a new choice to the question.
    ///
    /// # Arguments
    /// * `choice` - The `ChoiceAnswer` to add to the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.push_choice(("Option A".to_string(), false));
    /// assert_eq!(question.get_choices().len(), 1);
    /// ```
    #[inline]
    pub fn push_choice(&mut self, choice: ChoiceAnswer)
    {
        self.choices.push(choice);
    }

    // pub fn get_choices(&self) -> &Choices
    /// Gets a reference to the vector of choices.
    ///
    /// # Returns
    /// `&Choices` - A reference to the vector of `ChoiceAnswer`s.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new(1, 1, 1, "Q".to_string(), vec![("A".to_string(), false), ("B".to_string(), false)]);
    /// assert_eq!(question.get_choices().len(), 2);
    /// ```
    #[inline]
    pub fn get_choices(&self) -> &Choices
    {
        &self.choices
    }

    // pub fn set_choices(&mut self, choices: Choices)
    /// Sets the entire vector of choices.
    ///
    /// # Arguments
    /// * `choices` - The new vector of `ChoiceAnswer`s to set.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_choices(vec![("New A".to_string(), false), ("New B".to_string(), false)]);
    /// assert_eq!(question.get_choices().len(), 2);
    /// ```
    #[inline]
    pub fn set_choices(&mut self, choices: Choices)
    {
        self.choices = choices;
    }

    // pub fn determine_category(&mut self)
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
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// 
    /// question.determine_category();
    /// assert_eq!(question.get_category(), 4); // No choices, so category is 4 (essay)
    /// 
    /// question.push_choice(("Option A".to_string(), false));
    /// question.determine_category();
    /// assert_eq!(question.get_category(), 4); // One incorrect choice, so category is 4 (essay)
    /// 
    /// question.set_choice(0, ("Option B".to_string(), true));
    /// question.determine_category();
    /// assert_eq!(question.get_category(), 3); // One correct choice, so category is 3 (short answer)
    /// 
    /// question.push_choice(("Option C".to_string(), false));
    /// question.determine_category();
    /// assert_eq!(question.get_category(), 1); // Multiple choices with one correct, so category is 1 (single answer of multiple-choice)
    /// 
    /// question.push_choice(("Option D".to_string(), true));
    /// question.determine_category();
    /// assert_eq!(question.get_category(), 2); // Multiple choices with multiple correct, so category is 2 (multiple answers of multiple-choice)
    /// 
    /// question.set_choice(0, ("Option A".to_string(), true));
    /// question.set_choice(1, ("Option B".to_string(), true));
    /// question.set_choice(2, ("Option C".to_string(), true));
    /// question.set_choice(3, ("Option D".to_string(), true));
    /// question.determine_category();
    /// assert_eq!(question.get_category(), 3); // Multiple choices with all correct, so category is 3 (short answer)
    /// ```
    pub fn determine_category(&mut self)
    {
        let choice_count = self.choices.len();
        if choice_count == 0
            { self.category = 4; }
        else if choice_count == 1
            { self.category = 4 - self.choices[0].1 as u8; }
        else
        {
            let correct_count = self.choices.iter().filter(|c| c.1).count();
            if correct_count == 1
                { self.category = 1; }
            else if correct_count == choice_count
                { self.category = 3; }
            else
                { self.category = 2; }
        }
    }
}
