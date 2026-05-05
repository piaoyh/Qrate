// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use cryptocol::random::Random_PRNG_Creator;

/// A type alias for a vector of `ShuffledQuestion`s, representing a set of shuffled questions.
pub type ShuffledQuestions = Vec<ShuffledQuestion>;

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
        for last in (1..max).rev()
        {
            let chosen = prng.random_under_uint_(last + 1);
            (self.choices[last], self.choices[chosen]) = (self.choices[chosen], self.choices[last]);
        }
    }
}