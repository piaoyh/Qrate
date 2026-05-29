// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////

use std::fs::File;
use std::io::{ Cursor, Write };
use std::path::Path;

use docx_rs::{ Docx, Paragraph, Run, BreakType, PageMargin, AlignmentType,
                Footer, InstrText, InstrPAGE, InstrNUMPAGES, FieldCharType };

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use hwpers::{HwpWriter, HwpxWriter};
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use hwpers::style::TextStyle;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use hwpers::hwpx::{HwpxTextStyle, StyledText};

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use genpdfi::error::Error;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use genpdfi::{ Document, elements, fonts, style, Element, SimplePageDecorator, Alignment };

use crate::{ Choices, QBank, Questions, check_path };
use crate::{ SBank, Student };
use crate::Shuffler;


pub struct Generator
{
    shuffler: Shuffler,
    current_question_number: u16,
    body_font_size: f32,
    title_font_size: f32,
    answer_sheet_font_size: f32,
    footer_font_size: f32,
    attributes: u16,
    margin_left_in_mm: f32,
    margin_right_in_mm: f32,
    margin_top_in_mm: f32,
    margin_buttom_in_mm: f32,
    line_spacing: f32,
    answer_sheet_title: String,
}

impl Generator
{
    const BODY_BOLD: u16 = 0b_1;
    const BODY_ITALIC: u16 = 0b_10;
    const BODY_UNDERLINE: u16 = 0b_100;
    const BODY_STRIKE: u16 = 0b_1000;

    const TITLE_BOLD: u16 = 0b_1_0000;
    const TITLE_ITALIC: u16 = 0b_10_0000;
    const TITLE_UNDERLINE: u16 = 0b_100_0000;
    const TITLE_STRIKE: u16 = 0b_1000_0000;

    const ANSWER_SHEET_BOLD: u16 = 0b_1_0000_0000;
    const ANSWER_SHEET_ITALIC: u16 = 0b_10_0000_0000;
    const ANSWER_SHEET_UNDERLINE: u16 = 0b_100_0000_0000;
    const ANSWER_SHEET_STRIKE: u16 = 0b_1000_0000_0000;

    const FOOTER_BOLD: u16 = 0b_1_0000_0000_0000;
    const FOOTER_ITALIC: u16 = 0b_10_0000_0000_0000;
    const FOOTER_UNDERLINE: u16 = 0b_100_0000_0000_0000;
    const FOOTER_STRIKE: u16 = 0b_1000_0000_0000_0000;

    // pub fn new(qbank: &QBank, start: u16, end: u16, number_of_questions: usize, students: &SBank) -> Option<Self>
    /// Creates a new `Generator` instance for multiple shuffled sets,
    /// one for each student.
    ///
    /// This function generates shuffled question sets for each student based
    /// on the provided question bank, considering a specified range and number
    /// of randomly selected questions.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `start` - The 1-based starting index of questions to consider (inclusive).
    /// * `end` - The 1-based ending index of questions to consider (inclusive).
    /// * `number_of_questions` - The number of questions to be randomly
    ///   selected for each student.
    /// * `students` - A slice of `Student` instances for whom shuffled sets
    ///    will be generated.
    ///
    /// # Output
    /// An `Option<Self>` which is `Some(Generator)` if successful, or
    /// `None` if the generation fails (e.g., invalid question range,
    /// insufficient questions, or selected count).
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, SBank };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let student2 = Student::new_from_name("Bob".to_string());
    /// let students = SBank::new_with_students(vec![student1, student2]);
    ///
    /// // Generate exams with 2 questions selected for each student
    /// let generator = Generator::new(&qbank, 1, 2, 2, &students);
    /// assert!(generator.is_some());
    /// ```
    pub fn new(qbank: &QBank, start: u16, end: u16, number_of_questions: usize, students: &SBank) -> Option<Self>
    {
        let mut shuffler = Shuffler::new(qbank, start, end, students);
        shuffler.make_exams(number_of_questions);
        Some(
            Self
            {
                shuffler,
                current_question_number: 0,
                title_font_size: 14.0,
                body_font_size: 11.0,
                answer_sheet_font_size: 12.0,
                footer_font_size: 9.0,
                attributes: Self::TITLE_BOLD,
                margin_left_in_mm: 10.0,
                margin_right_in_mm: 10.0,
                margin_top_in_mm: 10.0,
                margin_buttom_in_mm: 10.0,
                line_spacing: 1.0,
                answer_sheet_title: "Answer Sheet        정답지        Ответы".to_string()
             }
        )
    }

    // pub fn new_empty() -> Self
    /// Creates a new, empty `Generator` instance with default values.
    ///
    /// This function initializes all fields of the `Generator` struct to their
    /// default empty or initial states, such as an empty `QBank` and
    /// `ShuffledQSets`, and predefined font sizes and margins.
    ///
    /// # Output
    /// `Self` - A new `Generator` instance, ready for configuration.
    ///
    /// # Examples
    /// ```
    /// use qrate::Generator;
    ///
    /// let generator = Generator::new_empty();
    /// // Verify that the generator's internal qbank is empty.
    /// assert!(generator.origin.get_questions().is_empty());
    /// ```
    pub fn new_empty() -> Self
    {
        Self
        {
            shuffler: Shuffler::new(&QBank::new_empty(), 1, 1, &SBank::new()),
            current_question_number: 0,
            title_font_size: 14.0,
            body_font_size: 11.0,
            answer_sheet_font_size: 12.0,
            footer_font_size: 9.0,
            attributes: Self::TITLE_BOLD,
            margin_left_in_mm: 10.0,
            margin_right_in_mm: 10.0,
            margin_top_in_mm: 10.0,
            margin_buttom_in_mm: 10.0,
            line_spacing: 1.0,
            answer_sheet_title: "Answer Sheet        정답지        Ответы".to_string()
        }
    }

    // pub fn new_one_set(qbank: &QBank, start: u16, end: u16, selected: usize) -> Option<Self>
    /// Creates a new `Generator` instance for a single shuffled set.
    ///
    /// This function generates a single shuffled question set based on the provided
    /// question bank, starting and ending question numbers.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `start` - The starting number of the questions to include (inclusive).
    /// * `end` - The ending number of the questions to include (inclusive).
    /// * `selected` - The number of questions to be randomly selected.
    ///
    /// # Output
    /// An `Option<Self>` which is `Some(Generator)` if successful, or `None` if
    /// the generation fails (e.g., invalid question range).
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let generator = Generator::new_one_set(&qbank, 1, 2, 2);
    /// assert!(generator.is_some());
    /// ```
    pub fn new_one_set(qbank: &QBank, start: u16, end: u16, selected: usize) -> Option<Self>
    {
        let student = Student::new_empty();
        let students = SBank::new_with_students(vec![student]);
        Self::new(qbank, start, end, selected, &students)
    }

    // pub fn new_with_seeds(qbank: &QBank, start: u16, end: u16, number_of_questions: usize, students: &Students, seeds: [u64; 16]) -> Option<Self>
    /// Creates a new `Generator` instance for multiple shuffled sets,
    /// one for each student.
    ///
    /// This function generates shuffled question sets for each student based
    /// on the provided question bank, considering a specified range and number
    /// of randomly selected questions.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `start` - The 1-based starting index of questions to consider (inclusive).
    /// * `end` - The 1-based ending index of questions to consider (inclusive).
    /// * `number_of_questions` - The number of questions to be randomly
    ///   selected for each student.
    /// * `students` - A slice of `Student` instances for whom shuffled sets
    ///    will be generated.
    /// * `seeds` - A seed array, each element of which is of u64.
    ///
    /// # Returns
    /// An `Option<Self>` which is `Some(Generator)` if successful, or
    /// `None` if the generation fails (e.g., invalid question range,
    /// insufficient questions, or selected count).
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let student2 = Student::new_from_name("Bob".to_string());
    /// let students = SBank::new_with_students(vec![student1, student2]);
    /// let seeds = [0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    /// 
    /// // Generate exams with 2 questions selected for each student
    /// let generator = Generator::new_with_seeds(&qbank, 1, 2, 2, &students, seed);
    /// assert!(generator.is_some());
    /// ```
    pub fn new_with_seeds(qbank: &QBank, start: u16, end: u16, number_of_questions: usize, students: &SBank, seeds: [u64; 16]) -> Option<Self>
    {
        let mut shuffler = Shuffler::new_with_seeds(qbank, start, end, students, seeds);
        if shuffler.make_exams(number_of_questions)
        {
            Some(
                Self
                {
                    shuffler,
                    current_question_number: 0,
                    title_font_size: 14.0,
                    body_font_size: 11.0,
                    answer_sheet_font_size: 12.0,
                    footer_font_size: 9.0,
                    attributes: Self::TITLE_BOLD,
                    margin_left_in_mm: 10.0,
                    margin_right_in_mm: 10.0,
                    margin_top_in_mm: 10.0,
                    margin_buttom_in_mm: 10.0,
                    line_spacing: 1.0,
                    answer_sheet_title: "Answer Sheet        정답지        Ответы".to_string()
                }
            )
        }
        else
        {
            None
        }
    }

    // pub fn new_empty_with_seeds(seeds: [u64; 16]) -> Self
    /// Creates a new, empty `Generator` instance with default values.
    ///
    /// This function initializes all fields of the `Generator` struct to their
    /// default empty or initial states, such as an empty `QBank` and
    /// `ShuffledQSets`, and predefined font sizes and margins.
    /// 
    /// # Arguments
    /// * `seeds` - A seed array, each element of which is of u64.
    ///
    /// # Returns
    /// `Self` - A new `Generator` instance, ready for configuration.
    ///
    /// # Examples
    /// ```
    /// use qrate::Generator;
    ///
    /// let seeds = [0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    /// let generator = Generator::new_empty_with_seeds(seeds);
    /// // Verify that the generator's internal qbank is empty.
    /// assert!(generator.origin.get_questions().is_empty());
    /// ```
    pub fn new_empty_with_seeds(seeds: [u64; 16]) -> Self
    {
        Self
        {
            shuffler: Shuffler::new_with_seeds(&QBank::new_empty(), 1, 1, &SBank::new(), seeds),
            current_question_number: 0,
            title_font_size: 14.0,
            body_font_size: 11.0,
            answer_sheet_font_size: 12.0,
            footer_font_size: 9.0,
            attributes: Self::TITLE_BOLD,
            margin_left_in_mm: 10.0,
            margin_right_in_mm: 10.0,
            margin_top_in_mm: 10.0,
            margin_buttom_in_mm: 10.0,
            line_spacing: 1.0,
            answer_sheet_title: "Answer Sheet        정답지        Ответы".to_string()
        }
    }

    // pub fn new_one_set(qbank: &QBank, start: u16, end: u16, selected: usize, seeds: [u64; 16]) -> Option<Self>
    /// Creates a new `Generator` instance for a single shuffled set.
    ///
    /// This function generates a single shuffled question set based on the provided
    /// question bank, starting and ending question numbers.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `start` - The starting number of the questions to include (inclusive).
    /// * `end` - The ending number of the questions to include (inclusive).
    /// * `selected` - The number of questions to be randomly selected.
    /// * `seeds` - A seed array, each element of which is of u64.
    ///
    /// # Output
    /// An `Option<Self>` which is `Some(Generator)` if successful, or `None` if
    /// the generation fails (e.g., invalid question range).
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let seeds = [0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    /// let generator = Generator::new_one_set_with_seeds(&qbank, 1, 2, 2, seeds);
    /// assert!(generator.is_some());
    /// ```
    pub fn new_one_set_with_seeds(qbank: &QBank, start: u16, end: u16, selected: usize, seeds: [u64; 16]) -> Option<Self>
    {
        let student = Student::new("Self Study".to_string(), "-".to_string());
        let students = SBank::new_with_students(vec![student]);
        Self::new_with_seeds(qbank, start, end, selected, &students, seeds)
    }

    // pub fn make_exams(&mut self, number_of_questions: usize)
    /// Generates a set of exams based on the specified number of questions.
    ///
    /// # Arguments
    /// * `number_of_questions` - The number of questions to be randomly
    ///   selected for each student.
    ///
    /// # Examples
    /// ```
    /// use qrate::Generator;
    ///
    /// let mut generator = Generator::new_empty();
    /// generator.make_exams(5);
    /// // Verify that the generator's internal qbank is empty.
    /// assert!(generator.origin.get_questions().is_empty());
    /// ```
    #[inline]
    pub fn make_exams(&mut self, number_of_questions: usize)
    {
        self.shuffler.make_exams(number_of_questions);
    }

    // pub fn get_title_font_size(&self) -> f32
    /// Retrieves the current title font size in points.
    ///
    /// # Output
    /// `f32` - The current font size used for titles.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let font_size = generator.get_title_font_size();
    /// assert_eq!(font_size, 14.0);
    /// ```
    #[inline]
    pub fn get_title_font_size(&self) -> f32
    {
        self.title_font_size
    }
    
    // pub fn set_title_font_size(&mut self, title_font_size: f32)
    /// Sets the title font siz in points.
    ///
    /// # Arguments
    /// * `title_font_size` - The new font size to be used for titles.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_title_font_size(16.0);
    /// assert_eq!(generator.get_title_font_size(), 16.0);
    /// ```
    #[inline]
    pub fn set_title_font_size(&mut self, title_font_size: f32)
    {
        self.title_font_size = title_font_size;
    }
    
    // pub fn get_body_font_size(&self) -> f32
    /// Retrieves the current default font size in points.
    ///
    /// # Output
    /// `f32` - The current default font size.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let font_size = generator.get_body_font_size();
    /// assert_eq!(font_size, 11.0);
    /// ```
    #[inline]
    pub fn get_body_font_size(&self) -> f32
    {
        self.body_font_size
    }
    
    // pub fn set_body_font_size(&mut self, body_font_size: f32)
    /// Sets the default font size in points.
    ///
    /// # Arguments
    /// * `body_font_size` - The new default font size.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_body_font_size(12.0);
    /// assert_eq!(generator.get_body_font_size(), 12.0);
    /// ```
    #[inline]
    pub fn set_body_font_size(&mut self, body_font_size: f32)
    {
        self.body_font_size = body_font_size
    }
    
    // pub fn get_footer_font_size(&self) -> f32
    /// Retrieves the current footer font size in points.
    ///
    /// # Output
    /// `f32` - The current font size used for footers.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let font_size = generator.get_footer_font_size();
    /// assert_eq!(font_size, 9.0);
    /// ```
    #[inline]
    pub fn get_footer_font_size(&self) -> f32
    {
        self.footer_font_size
    }
    
    // pub fn set_footer_font_size(&mut self, footer_font_size: f32)
    /// Sets the footer font size in points.
    ///
    /// # Arguments
    /// * `footer_font_size` - The new font size to be used for footers.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_footer_font_size(10.0);
    /// assert_eq!(generator.get_footer_font_size(), 10.0);
    /// ```
    #[inline]
    pub fn set_footer_font_size(&mut self, footer_font_size: f32)
    {
        self.footer_font_size = footer_font_size
    }
    
    // pub fn get_answer_sheet_font_size(&self) -> f32
    /// Retrieves the current answer sheet font size in points.
    ///
    /// # Output
    /// `f32` - The current font size used for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let font_size = generator.get_answer_sheet_font_size();
    /// assert_eq!(font_size, 12.0);
    /// ```
    #[inline]
    pub fn get_answer_sheet_font_size(&self) -> f32
    {
        self.answer_sheet_font_size
    }
    
    // pub fn set_answer_sheet_font_size(&mut self, answer_sheet_font_size: f32)
    /// Sets the answer sheet font size in points.
    ///
    /// # Arguments
    /// * `answer_sheet_font_size` - The new font size to be used for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_font_size(13.0);
    /// assert_eq!(generator.get_answer_sheet_font_size(), 13.0);
    /// ```
    #[inline]
    pub fn set_answer_sheet_font_size(&mut self, answer_sheet_font_size: f32)
    {
        self.answer_sheet_font_size = answer_sheet_font_size;
    }

    // pub fn is_body_bold(&self) -> bool
    /// Checks if the body text is set to bold.
    ///
    /// # Output
    /// `bool` - `true` if the body text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_body_bold());
    /// generator.set_body_bold(true);
    /// assert!(generator.is_body_bold());
    /// ```
    #[inline]
    pub fn is_body_bold(&self) -> bool
    {
        (self.attributes & Self::BODY_BOLD) == Self::BODY_BOLD
    }

    // pub fn set_body_bold(&mut self, on: bool)
    /// Sets or unsets the bold attribute for the body text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the body text to bold, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_body_bold(true);
    /// assert!(generator.is_body_bold());
    /// generator.set_body_bold(false);
    /// assert!(!generator.is_body_bold());
    /// ```
    #[inline]
    pub fn set_body_bold(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::BODY_BOLD; }
        else
            { self.attributes &= !Self::BODY_BOLD; }
    }

    // pub fn is_body_italic(&self) -> bool
    /// Checks if the body text is set to italic.
    ///
    /// # Output
    /// `bool` - `true` if the body text is italic, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_body_italic());
    /// generator.set_body_italic(true);
    /// assert!(generator.is_body_italic());
    /// ```
    #[inline]
    pub fn is_body_italic(&self) -> bool
    {
        (self.attributes & Self::BODY_ITALIC) == Self::BODY_ITALIC
    }

    // pub fn set_body_italic(&mut self, on: bool)
    /// Sets or unsets the italic attribute for the body text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the body text to italic, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_body_italic(true);
    /// assert!(generator.is_body_italic());
    /// generator.set_body_italic(false);
    /// assert!(!generator.is_body_italic());
    /// ```
    #[inline]
    pub fn set_body_italic(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::BODY_ITALIC; }
        else
            { self.attributes &= !Self::BODY_ITALIC; }
    }

    // pub fn is_body_underline(&self) -> bool
    /// Checks if the body text is set to underline.
    ///
    /// # Output
    /// `bool` - `true` if the body text is underlined, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_body_underline());
    /// generator.set_body_underline(true);
    /// assert!(generator.is_body_underline());
    /// ```
    #[inline]
    pub fn is_body_underline(&self) -> bool
    {
        (self.attributes & Self::BODY_UNDERLINE) == Self::BODY_UNDERLINE
    }

    // pub fn set_body_underline(&mut self, on: bool)
    /// Sets or unsets the underline attribute for the body text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the body text to underline, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_body_underline(true);
    /// assert!(generator.is_body_underline());
    /// generator.set_body_underline(false);
    /// assert!(!generator.is_body_underline());
    /// ```
    #[inline]
    pub fn set_body_underline(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::BODY_UNDERLINE; }
        else
            { self.attributes &= !Self::BODY_UNDERLINE; }
    }

    // pub fn is_body_strike(&self) -> bool
    /// Checks if the body text is set to strikethrough.
    ///
    /// # Output
    /// `bool` - `true` if the body text is strikethrough, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_body_strike());
    /// generator.set_body_strike(true);
    /// assert!(generator.is_body_strike());
    /// ```
    #[inline]
    pub fn is_body_strike(&self) -> bool
    {
        (self.attributes & Self::BODY_STRIKE) == Self::BODY_STRIKE
    }

    // pub fn set_body_strike(&mut self, on: bool)
    /// Sets or unsets the strikethrough attribute for the body text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the body text to strikethrough, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_body_strike(true);
    /// assert!(generator.is_body_strike());
    /// generator.set_body_strike(false);
    /// assert!(!generator.is_body_strike());
    /// ```
    #[inline]
    pub fn set_body_strike(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::BODY_STRIKE; }
        else
            { self.attributes &= !Self::BODY_STRIKE; }
    }


    // pub fn is_title_bold(&self) -> bool
    /// Checks if the title text is set to bold.
    ///
    /// # Output
    /// `bool` - `true` if the title text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(generator.is_title_bold()); // Default is true
    /// generator.set_title_bold(false);
    /// assert!(!generator.is_title_bold());
    /// ```
    #[inline]
    pub fn is_title_bold(&self) -> bool
    {
        (self.attributes & Self::TITLE_BOLD) == Self::TITLE_BOLD
    }

    // pub fn set_title_bold(&mut self, on: bool)
    /// Sets or unsets the bold attribute for the title text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the title text to bold, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_title_bold(false); // Default is true, so unset it
    /// assert!(!generator.is_title_bold());
    /// generator.set_title_bold(true);
    /// assert!(generator.is_title_bold());
    /// ```
    #[inline]
    pub fn set_title_bold(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::TITLE_BOLD; }
        else
            { self.attributes &= !Self::TITLE_BOLD; }
    }

    // pub fn is_title_italic(&self) -> bool
    /// Checks if the title text is set to italic.
    ///
    /// # Output
    /// `bool` - `true` if the title text is italic, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_title_italic());
    /// generator.set_title_italic(true);
    /// assert!(generator.is_title_italic());
    /// ```
    #[inline]
    pub fn is_title_italic(&self) -> bool
    {
        (self.attributes & Self::TITLE_ITALIC) == Self::TITLE_ITALIC
    }

    // pub fn set_title_italic(&mut self, on: bool)
    /// Sets or unsets the italic attribute for the title text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the title text to italic, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_title_italic(true);
    /// assert!(generator.is_title_italic());
    /// generator.set_title_italic(false);
    /// assert!(!generator.is_title_italic());
    /// ```
    #[inline]
    pub fn set_title_italic(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::TITLE_ITALIC; }
        else
            { self.attributes &= !Self::TITLE_ITALIC; }
    }

    // pub fn is_title_underline(&self) -> bool
    /// Checks if the title text is set to underline.
    ///
    /// # Output
    /// `bool` - `true` if the title text is underlined, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_title_underline());
    /// generator.set_title_underline(true);
    /// assert!(generator.is_title_underline());
    /// ```
    #[inline]
    pub fn is_title_underline(&self) -> bool
    {
        (self.attributes & Self::TITLE_UNDERLINE) == Self::TITLE_UNDERLINE
    }

    // pub fn set_title_underline(&mut self, on: bool)
    /// Sets or unsets the underline attribute for the title text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the title text to underline, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_title_underline(true);
    /// assert!(generator.is_title_underline());
    /// generator.set_title_underline(false);
    /// assert!(!generator.is_title_underline());
    /// ```
    #[inline]
    pub fn set_title_underline(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::TITLE_UNDERLINE; }
        else
            { self.attributes &= !Self::TITLE_UNDERLINE; }
    }

    // pub fn is_title_strike(&self) -> bool
    /// Checks if the title text is set to strikethrough.
    ///
    /// # Output
    /// `bool` - `true` if the title text is strikethrough, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_title_strike());
    /// generator.set_title_strike(true);
    /// assert!(generator.is_title_strike());
    /// ```
    #[inline]
    pub fn is_title_strike(&self) -> bool
    {
        (self.attributes & Self::TITLE_STRIKE) == Self::TITLE_STRIKE
    }

    // pub fn set_title_strike(&mut self, on: bool)
    /// Sets or unsets the strikethrough attribute for the title text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the title text to strikethrough, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_title_strike(true);
    /// assert!(generator.is_title_strike());
    /// generator.set_title_strike(false);
    /// assert!(!generator.is_title_strike());
    /// ```
    #[inline]
    pub fn set_title_strike(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::TITLE_STRIKE; }
        else
            { self.attributes &= !Self::TITLE_STRIKE; }
    }

    // pub fn is_footer_bold(&self) -> bool
    /// Checks if the footer text is set to bold.
    ///
    /// # Output
    /// `bool` - `true` if the footer text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_footer_bold());
    /// generator.set_footer_bold(true);
    /// assert!(generator.is_footer_bold());
    /// ```
    #[inline]
    pub fn is_footer_bold(&self) -> bool
    {
        (self.attributes & Self::FOOTER_BOLD) == Self::FOOTER_BOLD
    }

    // pub fn set_footer_bold(&mut self, on: bool)
    /// Sets or unsets the bold attribute for the footer text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the footer text to bold, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_footer_bold(true);
    /// assert!(generator.is_footer_bold());
    /// generator.set_footer_bold(false);
    /// assert!(!generator.is_footer_bold());
    /// ```
    #[inline]
    pub fn set_footer_bold(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::FOOTER_BOLD; }
        else
            { self.attributes &= !Self::FOOTER_BOLD; }
    }

    // pub fn is_footer_italic(&self) -> bool
    /// Checks if the footer text is set to italic.
    ///
    /// # Output
    /// `bool` - `true` if the footer text is italic, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_footer_italic());
    /// generator.set_footer_italic(true);
    /// assert!(generator.is_footer_italic());
    /// ```
    #[inline]
    pub fn is_footer_italic(&self) -> bool
    {
        (self.attributes & Self::FOOTER_ITALIC) == Self::FOOTER_ITALIC
    }

    // pub fn set_footer_italic(&mut self, on: bool)
    /// Sets or unsets the italic attribute for the footer text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the footer text to italic, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_footer_italic(true);
    /// assert!(generator.is_footer_italic());
    /// generator.set_footer_italic(false);
    /// assert!(!generator.is_footer_italic());
    /// ```
    #[inline]
    pub fn set_footer_italic(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::FOOTER_ITALIC; }
        else
            { self.attributes &= !Self::FOOTER_ITALIC; }
    }

    // pub fn is_footer_underline(&self) -> bool
    /// Checks if the footer text is set to underline.
    ///
    /// # Output
    /// `bool` - `true` if the footer text is underlined, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_footer_underline(true);
    /// assert!(generator.is_footer_underline());
    /// ```
    #[inline]
    pub fn is_footer_underline(&self) -> bool
    {
        (self.attributes & Self::FOOTER_UNDERLINE) == Self::FOOTER_UNDERLINE
    }

    // pub fn set_footer_underline(&mut self, on: bool)
    /// Sets or unsets the underline attribute for the footer text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the footer text to underline, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_footer_underline(true);
    /// assert!(generator.is_footer_underline());
    /// generator.set_footer_underline(false);
    /// assert!(!generator.is_footer_underline());
    /// ```
    #[inline]
    pub fn set_footer_underline(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::FOOTER_UNDERLINE; }
        else
            { self.attributes &= !Self::FOOTER_UNDERLINE; }
    }

    // pub fn is_footer_strike(&self) -> bool
    /// Checks if the footer text is set to strikethrough.
    ///
    /// # Output
    /// `bool` - `true` if the footer text is strikethrough, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_footer_strike());
    /// generator.set_footer_strike(true);
    /// assert!(generator.is_footer_strike());
    /// ```
    #[inline]
    pub fn is_footer_strike(&self) -> bool
    {
        (self.attributes & Self::FOOTER_STRIKE) == Self::FOOTER_STRIKE
    }

    // pub fn set_footer_strike(&mut self, on: bool)
    /// Sets or unsets the strikethrough attribute for the footer text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the footer text to strikethrough, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_footer_strike(true);
    /// assert!(generator.is_footer_strike());
    /// generator.set_footer_strike(false);
    /// assert!(!generator.is_footer_strike());
    /// ```
    #[inline]
    pub fn set_footer_strike(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::FOOTER_STRIKE; }
        else
            { self.attributes &= !Self::FOOTER_STRIKE; }
    }

    // pub fn is_answer_sheet_bold(&self) -> bool
    /// Checks if the answer sheet text is set to bold.
    ///
    /// # Output
    /// `bool` - `true` if the answer sheet text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_answer_sheet_bold());
    /// generator.set_answer_sheet_bold(true);
    /// assert!(generator.is_answer_sheet_bold());
    /// ```
    #[inline]
    pub fn is_answer_sheet_bold(&self) -> bool
    {
        (self.attributes & Self::ANSWER_SHEET_BOLD) == Self::ANSWER_SHEET_BOLD
    }

    // pub fn set_answer_sheet_bold(&mut self, on: bool)
    /// Sets or unsets the bold attribute for the answer sheet text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the answer sheet text to bold, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_bold(true);
    /// assert!(generator.is_answer_sheet_bold());
    /// generator.set_answer_sheet_bold(false);
    /// assert!(!generator.is_answer_sheet_bold());
    /// ```
    #[inline]
    pub fn set_answer_sheet_bold(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::ANSWER_SHEET_BOLD; }
        else
            { self.attributes &= !Self::ANSWER_SHEET_BOLD; }
    }

    // pub fn is_answer_sheet_italic(&self) -> bool
    /// Checks if the answer sheet text is set to italic.
    ///
    /// # Output
    /// `bool` - `true` if the answer sheet text is italic, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_answer_sheet_italic());
    /// generator.set_answer_sheet_italic(true);
    /// assert!(generator.is_answer_sheet_italic());
    /// ```
    #[inline]
    pub fn is_answer_sheet_italic(&self) -> bool
    {
        (self.attributes & Self::ANSWER_SHEET_ITALIC) == Self::ANSWER_SHEET_ITALIC
    }

    // pub fn set_answer_sheet_italic(&mut self, on: bool)
    /// Sets or unsets the italic attribute for the answer sheet text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the answer sheet text to italic, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_italic(true);
    /// assert!(generator.is_answer_sheet_italic());
    /// generator.set_answer_sheet_italic(false);
    /// assert!(!generator.is_answer_sheet_italic());
    /// ```
    #[inline]
    pub fn set_answer_sheet_italic(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::ANSWER_SHEET_ITALIC; }
        else
            { self.attributes &= !Self::ANSWER_SHEET_ITALIC; }
    }

    // pub fn is_answer_sheet_underline(&self) -> bool
    /// Checks if the answer sheet text is set to underline.
    ///
    /// # Output
    /// `bool` - `true` if the answer sheet text is underlined, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// assert!(!generator.is_answer_sheet_underline());
    /// generator.set_answer_sheet_underline(true);
    /// assert!(generator.is_answer_sheet_underline());
    /// ```
    #[inline]
    pub fn is_answer_sheet_underline(&self) -> bool
    {
        (self.attributes & Self::ANSWER_SHEET_UNDERLINE) == Self::ANSWER_SHEET_UNDERLINE
    }

    // pub fn set_answer_sheet_underline(&mut self, on: bool)
    /// Sets or unsets the underline attribute for the answer sheet text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the answer sheet text to underline, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_underline(true);
    /// assert!(generator.is_answer_sheet_underline());
    /// generator.set_answer_sheet_underline(false);
    /// assert!(!generator.is_answer_sheet_underline());
    /// ```
    #[inline]
    pub fn set_answer_sheet_underline(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::ANSWER_SHEET_UNDERLINE; }
        else
            { self.attributes &= !Self::ANSWER_SHEET_UNDERLINE; }
    }

    // pub fn is_answer_sheet_strike(&self) -> bool
    /// Checks if the answer sheet text is set to strikethrough.
    ///
    /// # Output
    /// `bool` - `true` if the answer sheet text is strikethrough, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_strike(true);
    /// assert!(generator.is_answer_sheet_strike());
    /// ```
    #[inline]
    pub fn is_answer_sheet_strike(&self) -> bool
    {
        (self.attributes & Self::ANSWER_SHEET_STRIKE) == Self::ANSWER_SHEET_STRIKE
    }

    // pub fn set_answer_sheet_strike(&mut self, on: bool)
    /// Sets or unsets the strikethrough attribute for the answer sheet text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the answer sheet text to strikethrough, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_strike(true);
    /// assert!(generator.is_answer_sheet_strike());
    /// generator.set_answer_sheet_strike(false);
    /// assert!(!generator.is_answer_sheet_strike());
    /// ```
    #[inline]
    pub fn set_answer_sheet_strike(&mut self, on: bool)
    {
        if on
            { self.attributes |= Self::ANSWER_SHEET_STRIKE; }
        else
            { self.attributes &= !Self::ANSWER_SHEET_STRIKE; }
    }

    // pub fn get_attributes(&self) -> u16
    /// Retrieves the current attribute bitmask.
    ///
    /// # Output
    /// `u16` - The current attribute bitmask.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let attributes = generator.get_attributes();
    /// // Example: Check if the default title is bold
    /// // const TITLE_BOLD: u16 = 0b_1_0000;
    /// // assert_eq!(attributes & TITLE_BOLD, TITLE_BOLD);
    /// ```
    #[inline]
    pub fn get_attributes(&self) -> u16
    {
        self.attributes
    }

    // pub fn set_attributes(&mut self, attr: u16)
    /// Sets the specified attributes by performing a bitwise OR operation.
    ///
    /// This method allows combining multiple attributes (e.g., bold, italic)
    /// by passing a bitmask.
    ///
    /// # Arguments
    /// * `attr` - A `u16` bitmask representing the attributes to set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// // For example purposes, let's assume direct access or similar constants
    /// const BODY_BOLD: u16 = 0b_1;
    /// const BODY_ITALIC: u16 = 0b_10;
    /// generator.set_attributes(BODY_BOLD | BODY_ITALIC);
    /// // assert_eq!(generator.get_attributes() & (BODY_BOLD | BODY_ITALIC), BODY_BOLD | BODY_ITALIC);
    /// ```
    #[inline]
    pub fn set_attributes(&mut self, attr: u16)
    {
        self.attributes |= attr;
    }

    // pub fn reset_attributes(&mut self, attr: u16)
    /// Resets the specified attributes by performing a bitwise AND NOT operation.
    ///
    /// This method allows unsetting multiple attributes (e.g., bold, italic)
    /// by passing a bitmask.
    ///
    /// # Arguments
    /// * `attr` - A `u16` bitmask representing the attributes to reset.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// // For example purposes, let's assume direct access or similar constants
    /// const BODY_BOLD: u16 = 0b_1;
    /// const BODY_ITALIC: u16 = 0b_10;
    /// generator.set_attributes(BODY_BOLD | BODY_ITALIC);
    /// // At this point, BODY_BOLD and BODY_ITALIC are set.
    /// generator.reset_attributes(BODY_ITALIC);
    /// // Now only BODY_BOLD should be set.
    /// // assert_eq!(generator.get_attributes() & BODY_BOLD, BODY_BOLD);
    /// // assert_eq!(generator.get_attributes() & BODY_ITALIC, 0);
    /// ```
    #[inline]
    pub fn reset_attributes(&mut self, attr: u16)
    {
        self.attributes &= !attr;
    }
    
    // pub fn get_margin_left_in_mm(&self) -> f32
    /// Retrieves the current left margin in millimeters.
    ///
    /// # Output
    /// `f32` - The current left margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let margin = generator.get_margin_left_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_left_in_mm(&self) -> f32
    {
        self.margin_left_in_mm
    }
    
    // pub fn set_margin_left_in_mm(&mut self, margin_left_in_mm: f32)
    /// Sets the left margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_left_in_mm` - The new left margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_margin_left_in_mm(15.0);
    /// assert_eq!(generator.get_margin_left_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_left_in_mm(&mut self, margin_left_in_mm: f32)
    {
        self.margin_left_in_mm = margin_left_in_mm;
    }
    
    // pub fn get_margin_right_in_mm(&self) -> f32
    /// Retrieves the current right margin in millimeters.
    ///
    /// # Output
    /// `f32` - The current right margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let margin = generator.get_margin_right_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_right_in_mm(&self) -> f32
    {
        self.margin_right_in_mm
    }
    
    // pub fn set_margin_right_in_mm(&mut self, margin_right_in_mm: f32)
    /// Sets the right margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_right_in_mm` - The new right margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_margin_right_in_mm(15.0);
    /// assert_eq!(generator.get_margin_right_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_right_in_mm(&mut self, margin_right_in_mm: f32)
    {
        self.margin_right_in_mm = margin_right_in_mm;
    }
    
    // pub fn get_margin_top_in_mm(&self) -> f32
    /// Retrieves the current top margin in millimeters.
    ///
    /// # Output
    /// `f32` - The current top margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let margin = generator.get_margin_top_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_top_in_mm(&self) -> f32
    {
        self.margin_top_in_mm
    }
    
    // pub fn set_margin_top_in_mm(&mut self, margin_top_in_mm: f32)
    /// Sets the top margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_top_in_mm` - The new top margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_margin_top_in_mm(15.0);
    /// assert_eq!(generator.get_margin_top_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_top_in_mm(&mut self, margin_top_in_mm: f32)
    {
        self.margin_top_in_mm = margin_top_in_mm;
    }
    
    // pub fn get_margin_buttom_in_mm(&self) -> f32
    /// Retrieves the current bottom margin in millimeters.
    ///
    /// # Output
    /// `f32` - The current bottom margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let margin = generator.get_margin_buttom_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_buttom_in_mm(&self) -> f32
    {
        self.margin_buttom_in_mm
    }
    
    // pub fn set_margin_buttom_in_mm(&mut self, margin_buttom_in_mm: f32)
    /// Sets the bottom margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_buttom_in_mm` - The new bottom margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_margin_buttom_in_mm(15.0);
    /// assert_eq!(generator.get_margin_buttom_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_buttom_in_mm(&mut self, margin_buttom_in_mm: f32)
    {
        self.margin_buttom_in_mm = margin_buttom_in_mm;
    }
    
    // pub fn get_line_spacing(&self) -> f32
    /// Retrieves the current line spacing in lines.
    ///
    /// # Output
    /// `f32` - The current line spacing value.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let spacing = generator.get_line_spacing();
    /// assert_eq!(spacing, 1.0);
    /// ```
    #[inline]
    pub fn get_line_spacing(&self) -> f32
    {
        self.line_spacing
    }
    
    // pub fn set_line_spacing(&mut self, line_spacing: f32)
    /// Sets the line spacing in lines.
    ///
    /// # Arguments
    /// * `line_spacing` - The new line spacing value.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_line_spacing(1.5);
    /// assert_eq!(generator.get_line_spacing(), 1.5);
    /// ```
    #[inline]
    pub fn set_line_spacing(&mut self, line_spacing: f32)
    {
        self.line_spacing = line_spacing;
    }
    
    // pub fn get_answer_sheet_title(&self) -> String
    /// Retrieves the current answer sheet title.
    ///
    /// # Output
    /// `String` - The current title for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let title = generator.get_answer_sheet_title();
    /// assert_eq!(title, "Answer Sheet        정답지        Ответы".to_string());
    /// ```
    #[inline]
    pub fn get_answer_sheet_title(&self) -> String
    {
        self.answer_sheet_title.clone()
    }
    
    // pub fn set_answer_sheet_title(&mut self, answer_sheet_title: String)
    /// Sets the answer sheet title.
    ///
    /// # Arguments
    /// * `answer_sheet_title` - The new title for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_title("New Answer Sheet Title".to_string());
    /// assert_eq!(generator.get_answer_sheet_title(), "New Answer Sheet Title".to_string());
    /// ```
    #[inline]
    pub fn set_answer_sheet_title(&mut self, answer_sheet_title: String)
    {
        self.answer_sheet_title = answer_sheet_title;
    }

    // // pub(crate) fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    // // Retrieves a specific shuffled question set by its index.
    // //
    // // This function returns a cloned `ShuffledQSet` for the given index,
    // // if the index is within the bounds of the generated shuffled sets.
    // //
    // // # Arguments
    // // * `idx` - The zero-based index of the shuffled question set to retrieve.
    // //
    // // # Output
    // // An `Option<ShuffledQSet>` which is `Some(ShuffledQSet)` if the index is valid,
    // // or `None` if the index is out of bounds.
    // //
    // // # Examples
    // // ```
    // // use qrate::{ QBank, Generator, Student, Students };
    // //
    // // let mut qbank = QBank::new_empty();
    // // qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    // // qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    // //
    // // let student1 = Student::new_from_name("Alice".to_string());
    // // let students = Students::new(vec![student1]);
    // //
    // // let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    // // let shuffled_qset = generator.get_shuffled_qset(0);
    // // assert!(shuffled_qset.is_some());
    // // let no_shuffled_qset = generator.get_shuffled_qset(1);
    // // assert!(no_shuffled_qset.is_none());
    // // ```
    // #[inline]
    // pub(crate) fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    // {
    //     if idx < self.shuffled_qsets.len() { Some(self.shuffled_qsets[idx].clone()) } else { None }
    // }

    // pub fn get_shuffled_qbank(&self, idx: usize) -> Option<(Student, QBank)>
    /// Retrieves a specific shuffled `QBank` and its associated `Student` by index.
    ///
    /// This function reconstructs a `QBank` with shuffled questions for a given student
    /// based on the original `QBank` and the shuffled question set at the specified index.
    ///
    /// # Arguments
    /// * `idx` - The zero-based index of the shuffled question set.
    ///
    /// # Output
    /// An `Option<(Student, QBank)>` which is `Some((Student, QBank))` if the index is valid,
    /// or `None` if the index is out of bounds or a question cannot be found.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, &students).unwrap();
    /// let shuffled_qbank_tuple = generator.get_shuffled_qbank(0);
    /// assert!(shuffled_qbank_tuple.is_some());
    /// let (student, shuffled_qbank) = shuffled_qbank_tuple.unwrap();
    /// assert_eq!(student.get_name(), "Alice");
    /// assert_eq!(shuffled_qbank.get_questions().len(), 2);
    ///
    /// let no_shuffled_qbank = generator.get_shuffled_qbank(1);
    /// assert!(no_shuffled_qbank.is_none());
    /// ```
    pub fn get_shuffled_qbank(&self, idx: usize) -> Option<(Student, QBank)>
    {
        if idx < self.shuffler.get_sbank_length()
        {
            let header = self.shuffler.get_header().clone();
            let mut qbank = QBank::new_with_header(header);
            let mut questions = Questions::new();
            for i in 0..self.shuffler.get_qbank_length()
            {
                let qn = self.shuffler.get_shuffled_question(idx, i);
                // Find question by actual ID, not by index
                let question = self.shuffler.get_qbank().get_question(qn as usize)?;
                questions.push(question.clone());
            }
            qbank.set_questions(questions);
            Some((self.shuffler.get_student(idx).unwrap(), qbank))
        }
        else
        {
            None
        }
    }

    // pub fn get_shuffled_qbanks(&self) -> Vec::<(Student, QBank)>
    /// Retrieves all generated shuffled `QBank` instances
    /// with their associated `Student`s.
    ///
    /// This function iterates through all generated shuffled question sets and
    /// reconstructs a `QBank` for each, paired with its corresponding `Student`.
    ///
    /// # Output
    /// A `Vec<(Student, QBank)>` containing all shuffled question banks and
    /// their students.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let student2 = Student::new_from_name("Bob".to_string());
    /// let students = Students::new(vec![student1, student2]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, &students).unwrap();
    /// let shuffled_qbanks = generator.get_shuffled_qbanks();
    /// assert_eq!(shuffled_qbanks.len(), 2);
    /// assert_eq!(shuffled_qbanks[0].0.get_name(), "Alice");
    /// assert_eq!(shuffled_qbanks[1].0.get_name(), "Bob");
    /// ```
    pub fn get_shuffled_qbanks(&self) -> Vec::<(Student, QBank)>
    {
        let mut shuffled_qbanks = Vec::new();
        for i in 0..self.shuffler.get_qbank_length()
        {
            if let Some(shuffled_qbank) = self.get_shuffled_qbank(i)
                { shuffled_qbanks.push(shuffled_qbank); }
        }
        shuffled_qbanks
    }

    // pub fn get_notice(&self) -> String
    /// Retrieves the notice string from the original question bank's header.
    ///
    /// This function accesses the header of the `QBank` used to create the
    /// `Generator` instance and returns its notice string.
    ///
    /// # Output
    /// A `String` containing the notice from the question bank's header.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Header };
    ///
    /// let mut qbank = QBank::new_empty();
    /// let mut header = Header::new_empty();
    /// header.set_notice("Important Notice!".to_string());
    /// qbank.set_header(header);
    ///
    /// let generator = Generator::new_one_set(&qbank, 1, 1).unwrap();
    /// let notice = generator.get_notice();
    /// assert_eq!(notice, "Important Notice!");
    /// ```
    #[inline]
    pub fn get_notice(&self) -> String
    {
        self.shuffler.get_header().get_notice().clone()
    }

    // pub fn next(&mut self) -> Option<(u16, String, String, Choices)>
    /// Advances to the next question in the shuffled set
    /// and returns its details.
    ///
    /// This function acts as an iterator for the generated question set. Each
    /// call increments the internal question counter and provides the details
    /// of the next question, including the category, the question text,
    /// and the choices in their shuffled order.
    ///
    /// It is primarily used for self-testing scenarios,
    /// suchs as in the `exam()` function found in `src/examples/prep.rs`.
    ///
    /// # Output
    /// `Option<(u16, String, String, Choices)>` - An `Option` containing
    /// a tuple with:
    ///   - `u16`: The current question number within the shuffled set.
    ///   - `String`: The category of the current question.
    ///   - `String`: The text of the current question.
    ///   - `Choices`: A vector of tuples `(String, bool)` representing the
    ///                shuffled choices and whether each is a correct answer.
    ///
    /// Returns `None` if there are no more questions in the set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    ///
    /// let mut generator = Generator::new_one_set(&qbank, 1, 2).unwrap();
    ///
    /// if let Some((num, cat, q_text, choices)) = generator.next()
    ///     { assert_eq!(num, 1); }  // The actual question text depends on the shuffled order.
    ///
    /// if let Some((num, cat, q_text, choices)) = generator.next()
    ///     { assert_eq!(num, 2); }
    /// assert!(generator.next().is_none());
    /// ```
    pub fn next(&mut self) -> Option<(u16, u8, String, String, Choices)>
    {
        self.current_question_number += 1;
        self.get_question_by_number(self.current_question_number)
    }

    // pub fn prev(&mut self) -> Option<(u16, String, String, Choices)>
    /// Withdraws to the previous question in the shuffled set
    /// and returns its details.
    ///
    /// This function acts as an iterator for the generated question set. Each
    /// call decrements the internal question counter and provides the details
    /// of the previous question, including the category, the question text,
    /// and the choices in their shuffled order.
    ///
    /// It is primarily used for self-testing scenarios,
    /// suchs as in the `exam()` function found in `src/examples/prep.rs`.
    ///
    /// # Output
    /// `Option<(u16, String, String, Choices)>` - An `Option` containing
    /// a tuple with:
    ///   - `u16`: The current question number within the shuffled set.
    ///   - `String`: The category of the current question.
    ///   - `String`: The text of the current question.
    ///   - `Choices`: A vector of tuples `(String, bool)` representing the
    ///                shuffled choices and whether each is a correct answer.
    ///
    /// Returns `None` if there are no more questions in the set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    ///
    /// let mut generator = Generator::new_one_set(&qbank, 1, 2).unwrap();
    ///
    /// if let Some((num, cat, q_text, choices)) = generator.next()
    ///     { assert_eq!(num, 1); }  // The actual question text depends on the shuffled order.
    ///
    /// if let Some((num, cat, q_text, choices)) = generator.next()
    ///     { assert_eq!(num, 2); }  // The actual question text depends on the shuffled order.
    ///
    /// if let Some((num, cat, q_text, choices)) = generator.prev()
    ///     { assert_eq!(num, 1); }
    /// assert!(generator.prev().is_none());
    /// ```
    pub fn prev(&mut self) -> Option<(u16, u8, String, String, Choices)>
    {
        self.current_question_number -= 1;
        self.get_question_by_number(self.current_question_number)
    }

    // pub fn get_question_by_number(&self, num: u16) -> Option<(u16, u8, String, String, Choices)>
    /// Retrieves a specific question by its number within the shuffled set.
    /// 
    /// This function returns a tuple containing the question number, category,
    /// question text, and the choices in their shuffled order.
    /// 
    /// # Arguments
    /// * `num` - The number of the question to retrieve.
    /// 
    /// # Returns
    /// `Option<(u16, String, String, Choices)>` - An `Option` containing
    /// a tuple with:
    ///   - `u16`: The current question number within the shuffled set.
    ///   - `u8`: The category ID of the current question.
    ///   - `String`: The category string of the current question.
    ///   - `String`: The text of the current question.
    ///   - `Choices`: A vector of tuples `(String, bool)` representing the
    ///                shuffled choices and whether each is a correct answer.
    /// 
    /// Returns `None` if the question with the given number is not found.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    /// 
    /// let mut generator = Generator::new_one_set(&qbank, 1, 2).unwrap();
    /// 
    /// if let Some((num, cat_id, cat_str, q_text, choices)) = generator.get_question_by_number(1)
    ///     { assert_eq!(num, 1); }  // The actual question text depends on the shuffled order.
    /// 
    /// if let Some((num, cat_id, cat_str, q_text, choices)) = generator.get_question_by_number(2)
    ///     { assert_eq!(num, 2); }
    /// assert!(generator.get_question_by_number(3).is_none());
    /// ```
    pub fn get_question_by_number(&mut self, num: u16) -> Option<(u16, u8, String, String, Choices)>
    {
        self.current_question_number = num;
        let shuffled_qset = self.shuffler.get_shuffled_qsets().get(0)?;
        if num as usize > shuffled_qset.get_shuffled_questions().len() || num == 0
            { return None; }

        let shuffled_question = shuffled_qset.get_shuffled_question(num)?;
        let real_question_number = shuffled_question.get_question();
        let shuffled_indices = shuffled_question.get_choices();

        let origin_question = self.shuffler.get_qbank().get_question(real_question_number as usize)?;
        let category_id = origin_question.get_category();
        let category_str = self.shuffler.get_header().get_category(category_id)?.clone();
        let question_text = origin_question.get_question().clone();
        let origin_choices = origin_question.get_choices();

        let mut choices = Choices::new();
        for &shuffled_index in shuffled_indices
        {
            if let Some(choice) = origin_choices.get((shuffled_index - 1) as usize)
                { choices.push(choice.clone()); }
            else
                { return None; }
        }

        Some((num, category_id, category_str, question_text, choices))
    }

    // pub fn get_number_of_questions(&self) -> usize
    /// Returns the total number of shuffled questions for the first set (self-study).
    /// 
    /// # Returns
    /// `usize` - The total number of shuffled questions for the first set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    /// 
    /// let mut generator = Generator::new_one_set(&qbank, 1, 2).unwrap();
    /// assert_eq!(generator.get_number_of_questions(), 2);
    /// ```
    #[inline]
    pub fn get_number_of_questions(&self) -> usize
    {
        self.shuffler.get_qbank_length()
    }

    // pub fn get_current_question_number(&self) -> u16
    /// Retrieves the current question number within the shuffled set.
    /// 
    /// # Returns
    /// `u16` - The current question number within the shuffled set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    /// 
    /// let mut generator = Generator::new_one_set(&qbank, 1, 2).unwrap();
    /// assert_eq!(generator.get_current_question_number(), 0);
    /// 
    /// if let Some((num, cat, q_text, choices)) = generator.next()
    ///     { assert_eq!(num, 1); }
    /// assert_eq!(generator.get_current_question_number(), 1);
    /// 
    /// if let Some((num, cat, q_text, choices)) = generator.next()
    ///     { assert_eq!(num, 2); }
    /// assert_eq!(generator.get_current_question_number(), 2);
    /// assert!(generator.next().is_none());
    /// ```
    pub fn get_current_question_number(&self) -> u16
    {
        self.current_question_number
    }

    // pub fn set_current_question_number(&mut self, num: u16)
    /// Sets the current question number within the shuffled set.
    /// 
    /// # Arguments
    /// * `num` - The new question number to set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    /// 
    /// let mut generator = Generator::new_one_set(&qbank, 1, 2).unwrap();
    /// assert_eq!(generator.get_current_question_number(), 0);
    /// 
    /// generator.set_current_question_number(1);
    /// assert_eq!(generator.get_current_question_number(), 1);
    /// ```
    pub fn set_current_question_number(&mut self, num: u16)
    {
        self.current_question_number = num;
    }

    // pub fn save_shuffled_exams(&self, path: String, extention: &str) -> Result<(), String>
    /// Saves the shuffled exam sets for all students to a single file.
    ///
    /// The output format is determined by the file extension of the provided path.
    /// Supported formats are: .txt, .docx, and .pdf.
    /// This function delegates the actual saving process to format-specific private functions.
    ///
    /// # Arguments
    /// * `path` - The file path where the exams will be saved.
    /// * `extention` - The desired file extension (e.g., "txt", "docx", "pdf").
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs; // For std::fs::remove_file
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// // Generate exams with 1 question selected for each student
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    ///
    /// let result = generator.save_shuffled_exams("exam.txt".to_string(), "txt");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam.txt").unwrap();
    /// ```
    pub fn save_shuffled_exams(&self, path: String, extention: &str) -> Result<(), String>
    {
        let checked = check_path(path, extention);
        let file_path = Path::new(&checked);
        match file_path.extension().and_then(|s| s.to_str())
        {
            Some("txt") => self.save_shuffled_exams_in_txt(file_path),
            Some("docx") => self.save_shuffled_exams_in_docx(file_path),
            #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            Some("hwpx") => self.save_shuffled_exams_in_hwpx(file_path),
            #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            Some("hwp") => self.save_shuffled_exams_in_hwp(file_path),
            #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            Some("pdf") => self.save_shuffled_exams_in_pdf(file_path),
            _ => Err("Unsupported file format. Please use .txt, .docx, or .pdf.".to_string()),
        }
    }

    // fn format_exam_for_student(&self, student: &Student, qbank: &QBank) -> String
    /// Formats the exam content for a single student into a human-readable string.
    ///
    /// This private helper function generates the textual representation of an exam
    /// for a given student and their shuffled question bank. It includes the student's
    /// name, the exam title, and all questions with their shuffled choices.
    ///
    /// # Arguments
    /// * `student` - A reference to the `Student` for whom the exam is being formatted.
    /// * `qbank` - A reference to the `QBank` containing the shuffled questions for this student.
    ///
    /// # Output
    /// A `String` containing the fully formatted exam content for the student.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Header };
    ///
    /// let mut qbank = QBank::new_empty();
    /// let mut header = Header::new_empty();
    /// header.set_title("Test Exam".to_string());
    /// qbank.set_header(header);
    /// qbank.add_question_with_choices(
    ///     "What is 1+1?".to_string(),
    ///     vec![("1".to_string(), false), ("2".to_string(), true)]
    /// );
    ///
    /// let student = Student::new_from_name("John Doe".to_string());
    /// let generator = Generator::new_one_set(&qbank, 1, 1).unwrap();
    ///
    /// // Since format_exam_for_student is private, we can't directly call it in an example.
    /// // This example demonstrates how the data would be prepared.
    /// let (retrieved_student, retrieved_qbank) = generator.get_shuffled_qbank(0).unwrap();
    /// let formatted_content = format!(
    ///     "Student: {}\nExam: {}\n\n1. What is 1+1?\n    (A) 1\n    (B) 2\n",
    ///     retrieved_student.get_name(),
    ///     retrieved_qbank.get_header().get_title()
    /// );
    /// // In a real test, you'd assert against the output of the function,
    /// // but for a private helper, we rely on its callers to be tested.
    /// ```
    fn format_exam_for_student(&self, student: &Student, qbank: &QBank) -> String
    {
        let mut content = String::new();
        let header = qbank.get_header();

        // Exam Title
        content.push_str(&format!("{}\n", header.get_title()));

        // Student Information
        content.push_str(&format!("{}: {}        {}: {}\n\n", header.get_name(), student.get_name(), header.get_id(), student.get_id()));

        // Instructions
        content.push_str(&format!("{}\n\n", header.get_notice()));

        // Questions
        for (question_index, question) in qbank.get_questions().iter().enumerate()
        {
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s| s.as_str()).unwrap_or("");
            content.push_str(&format!("{}. [{}]   {}\n", question_index + 1, category_text, question.get_question()));
            
            if category_id == 3 {
                // Short answer: ( space * 3 * max_choice_len )
                let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                let spaces = " ".repeat(max_len * 3);
                content.push_str(&format!("    ({})\n", spaces));
            } else if category_id == 4 {
                // Essay: 15 blank lines
                for _ in 0..15 {
                    content.push_str("\n");
                }
            } else {
                // Category 1, 2: Standard choices
                for (choice_index, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
                {
                    let choice_char = (choice_index + 1).to_string();
                    content.push_str(&format!("    ({}) {}\n", choice_char, choice_text));
                }
            }
            content.push_str("\n"); // Blank line after each question
        }
        content
    }

    // pub fn save_shuffled_exams_in_txt(&self, path: &Path) -> Result<(), String>
    /// Saves the shuffled exam sets to a text file.
    ///
    /// This function generates a text file containing the shuffled exam sets
    /// for all students, with each student's exam separated by a clear delimiter.
    ///
    /// # Arguments
    /// * `path` - The file path where the text document will be saved.
    ///
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices(
    ///     "What is 1+1?".to_string(),
    ///     vec![("1".to_string(), false), ("2".to_string(), true)]
    /// );
    /// qbank.add_question_with_choices(
    ///     "What is 2+2?".to_string(),
    ///     vec![("3".to_string(), false), ("4".to_string(), true)]
    /// );
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    ///
    /// let result = generator.save_shuffled_exams_in_txt(Path::new("exam_shuffled.txt"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam_shuffled.txt").unwrap();
    /// ```
    pub fn save_shuffled_exams_in_txt(&self, path: &Path) -> Result<(), String>
    {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (student, qbank) in &shuffled_qbanks
        {
            let content = self.format_exam_for_student(&student, &qbank);
            writeln!(file, "{}", content).map_err(|e| e.to_string())?;
            // Add a separator for multiple students, if applicable
            if self.shuffler.get_sbank_length() > 1
                { writeln!(file, "-------X------- CUT -------X------- 자르기 -------X------- резать -------X-------\n\n").map_err(|e| e.to_string())?; }
        }
        // Add a separator for the answer sheet
        //write!(file, "\n\u{000C}\n").map_err(|e| e.to_string())?; // Form feed for page break

        let header = self.shuffler.get_header(); // Need the original header for titles
        writeln!(file, "{}{}", self.answer_sheet_title, "\n").map_err(|e| e.to_string())?;
        for (student, qbank) in &shuffled_qbanks
        {
            // Student Info
            writeln!(file, "{}: {}        {}: {}",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()
            ).map_err(|e| e.to_string())?;

            // Answers
            let mut answer_line = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let category_id = question.get_category();
                let answer_string = if category_id == 3 {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                } else if category_id == 4 {
                    "(---)".to_string()
                } else {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                };

                let entry = format!("{}. {}    ", i + 1, answer_string);

                // Simple line wrapping logic
                if answer_line.len() + entry.len() > 80 && !answer_line.is_empty() {
                    writeln!(file, "{}", answer_line).map_err(|e| e.to_string())?;
                    answer_line.clear();
                }
                answer_line.push_str(&entry);
            }
            if !answer_line.is_empty() {
                writeln!(file, "{}", answer_line).map_err(|e| e.to_string())?;
            }
            writeln!(file, "").map_err(|e| e.to_string())?; // Blank line after each student
        }
        Ok(())
    }

    // pub fn export_shuffled_exams_in_txt(&self) -> Vec<u8>
    /// Exports the shuffled exam sets to a Vec<u8> object.
    ///
    /// This function generates a Vec<u8> object containing
    /// the shuffled exam sets for all students,
    /// with each student's exam separated by a clear delimiter
    /// in text format.
    ///
    /// # Returns
    /// `Vec<u8>` that contains the contents of String object.
    pub fn export_shuffled_exams_in_txt(&self) -> Vec<u8>
    {
        let mut content = String::new();
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (student, qbank) in &shuffled_qbanks
        {
            let exam_content = self.format_exam_for_student(&student, &qbank);
            content.push_str(exam_content.as_str());
            // Add a separator for multiple students, if applicable
            if self.shuffler.get_sbank_length() > 1
                { content.push_str("-------X------- CUT -------X------- 자르기 -------X------- резать -------X-------\n\n"); }
        }
        // Add a separator for the answer sheet
        //content.push_str("\n\u{000C}\n"); // Form feed for page break

        let header = self.shuffler.get_header(); // Need the original header for titles
        content.push_str(format!("{}{}\n", self.answer_sheet_title, "\n").as_str());
        for (student, qbank) in &shuffled_qbanks
        {
            // Student Info
            content.push_str(format!("{}: {}        {}: {}\n",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()).as_str());

            // Answers
            let mut answer_line = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let category_id = question.get_category();
                let answer_string = if category_id == 3 {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                } else if category_id == 4 {
                    "(---)".to_string()
                } else {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                };

                let entry = format!("{}. {}    ", i + 1, answer_string);

                // Simple line wrapping logic
                if answer_line.len() + entry.len() > 80 && !answer_line.is_empty() {
                    content.push_str(format!("{}\n", answer_line).as_str());
                    answer_line.clear();
                }
                answer_line.push_str(entry.as_str());
            }
            if !answer_line.is_empty() {
                content.push_str(format!("{}\n", answer_line).as_str());
            }
            content.push_str("\n"); // Blank line after each student
        }
        content.into_bytes()
    }

    // pub fn save_shuffled_exams_in_docx(&self, path: &Path) -> Result<(), String>
    /// Saves the shuffled exam sets to a DOCX file.
    ///
    /// This function generates a DOCX document containing the shuffled exam
    /// sets for all students, applying specified page margins and a footer
    /// with page numbers.
    ///
    /// # Arguments
    /// * `path` - The file path where the DOCX document will be saved.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    ///
    /// let result = generator.save_shuffled_exams_in_docx(Path::new("exam.docx"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam.docx").unwrap();
    /// ```
    pub fn save_shuffled_exams_in_docx(&self, path: &Path) -> Result<(), String>
    {
        let file = File::create(path).map_err(|e| e.to_string())?;
        self.build_shuffled_exams_in_docx()?
            .build()
            .pack(file)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // pub fn export_shuffled_exams_in_docx(&self) -> Result<Vec<u8>, String>
    /// Exports the shuffled exam sets to a Vec<u8> object.
    ///
    /// This function generates a DOCX document containing the shuffled exam
    /// sets for all students, applying specified page margins and a footer
    /// with page numbers. The resulting DOCX content is returned as a
    /// `Vec<u8>`, which can be used for in-memory operations or further
    /// processing without writing to disk.
    ///
    /// # Output
    /// `Vec<u8>` - Returns `Vec<u8>` which is binary dataon success,
    /// or .
    pub fn export_shuffled_exams_in_docx(&self) -> Result<Vec<u8>, String>
    {
        let mut buffer = Cursor::new(Vec::new());
        self.build_shuffled_exams_in_docx()?
            .build()
            .pack(&mut buffer)
            .map_err(|e| e.to_string())?;
        Ok(buffer.into_inner())
    }

    fn build_shuffled_exams_in_docx(&self) -> Result<Docx, String>
    {
        let pt_to_usize = |pt: f32| -> usize { (pt as usize) << 1 };
        let linespacing_to_twips = |linespacing: f32| -> i32 { (linespacing * 240.0) as i32 };
        let footer_font_size = pt_to_usize(self.footer_font_size);
        let mut footer_run = Run::new();
        if self.is_footer_bold()
            { footer_run = footer_run.bold(); }
        if self.is_footer_italic()
            { footer_run = footer_run.italic(); }
        if self.is_footer_underline()
            { footer_run = footer_run.underline("single"); }
        if self.is_footer_strike()
            { footer_run = footer_run.strike(); }
        let footer = Footer::new()
            .add_paragraph(
                Paragraph::new()
                    .add_run(footer_run.clone()
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::PAGE(InstrPAGE::default()))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // Placeholder text
                            .add_field_char(FieldCharType::End, false)
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .add_run(footer_run.clone()
                            .add_text(" / ")
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .add_run(footer_run
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::NUMPAGES(InstrNUMPAGES::default()))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // Placeholder text
                            .add_field_char(FieldCharType::End, false)
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .align(AlignmentType::Center)
            );
        let mm_to_twips = |mm: f32| -> i32  { (mm * 56.6929).round() as i32 };
        let left = mm_to_twips(self.margin_left_in_mm);
        let right = mm_to_twips(self.margin_right_in_mm);
        let top = mm_to_twips(self.margin_top_in_mm);
        let buttom = mm_to_twips(self.margin_buttom_in_mm);
        let mut docx = Docx::new()
                        .page_margin(
                            PageMargin::new()
                                .left(left)
                                .right(right)
                                .top(top)
                                .bottom(buttom)
            ) // 1 cm for default left, right, top, bottom
            .footer(footer);
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (idx, (student, qbank)) in shuffled_qbanks.iter().enumerate()
        {
            if idx > 0
                { docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page))); } // Page break for subsequent students
            docx = self.write_exam_content_to_docx(docx, &student, &qbank)?;
        }

        // Add answer sheet
        let title_font_size = pt_to_usize(self.title_font_size);
        let mut title_run = Run::new();
        if self.is_title_bold()
            { title_run = title_run.bold(); }
        if self.is_title_italic()
            { title_run = title_run.italic(); }
        if self.is_title_underline()
            { title_run = title_run.underline("single"); }
        if self.is_title_strike()
            { title_run = title_run.strike(); }
        docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
        docx = docx.add_paragraph(Paragraph::new()
                                    .add_run(title_run.clone()
                                            .add_text(self.answer_sheet_title.as_str())
                                            .size(title_font_size)
                                    )
                                    .align(AlignmentType::Center)); // 14 pt for default font size
        docx = docx.add_paragraph(Paragraph::new()); // Blank line

        let answer_sheet_font_size = pt_to_usize(self.answer_sheet_font_size);
        let mut answer_sheet_run = Run::new();
        if self.is_answer_sheet_bold()
            { answer_sheet_run = answer_sheet_run.bold(); }
        if self.is_answer_sheet_italic()
            { answer_sheet_run = answer_sheet_run.italic(); }
        if self.is_answer_sheet_underline()
            { answer_sheet_run = answer_sheet_run.underline("single"); }
        if self.is_answer_sheet_strike()
            { answer_sheet_run = answer_sheet_run.strike(); }
        
        let header = self.shuffler.get_header();
        let line_spacing = linespacing_to_twips(self.line_spacing);
        for (student, qbank) in &shuffled_qbanks
        {
            // Student Info
            let student_info_text = format!("{}: {}        {}: {}",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()
            );
            let student_info_paragraph = Paragraph::new()
                .add_run(
                    answer_sheet_run.clone()
                        .add_text(student_info_text)
                        .size(answer_sheet_font_size)) // 12 pt for default
                .line_spacing(docx_rs::LineSpacing::new().line(line_spacing));   // Single line spacing
            docx = docx.add_paragraph(student_info_paragraph);

            // Answers
            let mut answers_text = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let category_id = question.get_category();
                let answer_string = if category_id == 3 {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                } else if category_id == 4 {
                    "(---)".to_string()
                } else {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                };
                answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
            }

            let answers_paragraph = Paragraph::new()
                                        .add_run(answer_sheet_run.clone()
                                                    .add_text(answers_text)
                                                    .size(answer_sheet_font_size)
                                                ) // 12 pt for default answer sheet font size
                                        .line_spacing(docx_rs::LineSpacing::new().line(line_spacing));   // Single line spacing
            docx = docx.add_paragraph(answers_paragraph);
            docx = docx.add_paragraph(Paragraph::new()); // Blank line
        }
        Ok(docx)
    }


    // fn write_exam_content_to_docx(&self, docx: Docx, student: &Student, qbank: &QBank) -> Result<Docx, String>
    /// Writes the formatted exam content for a single student to a DOCX document.
    ///
    /// This private helper function takes a DOCX `Docx` object and appends
    /// the exam content for the given student and their shuffled question bank,
    /// returning the modified `Docx` object.
    ///
    /// # Arguments
    /// * `docx` - The `docx_rs::Docx` object.
    /// * `student` - A reference to the `Student` for whom the exam content is being written.
    /// * `qbank` - A reference to the `QBank` containing the shuffled questions for this student.
    ///
    /// # Output
    /// `Result<Docx, String>` - Returns `Ok(Docx)` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    fn write_exam_content_to_docx(&self, mut docx: Docx, student: &Student, qbank: &QBank) -> Result<Docx, String>
    {
        let pt_to_usize = |pt: f32| -> usize { (pt as usize) << 1 };
        
        // Exam Title
        let title_font_size = pt_to_usize(self.title_font_size);
        let mut title_run = Run::new();
        if self.is_title_bold()
            { title_run = title_run.bold(); }
        if self.is_title_italic()
            { title_run = title_run.italic(); }
        if self.is_title_underline()
            { title_run = title_run.underline("single"); }
        if self.is_title_strike()
            { title_run = title_run.strike(); }
        
        let header = qbank.get_header();
        let ex = Paragraph::new()
                .add_run(
                    title_run
                    .add_text(format!("{}", header.get_title()))
                    .size(title_font_size)
                )
                .align(AlignmentType::Center);
        
        let body_font_size = pt_to_usize(self.body_font_size);
        let mut body_run = Run::new();
        if self.is_body_bold()
            { body_run = body_run.bold(); }
        if self.is_body_italic()
            { body_run = body_run.italic(); }
        if self.is_body_underline()
            { body_run = body_run.underline("single"); }
        if self.is_body_strike()
            { body_run = body_run.strike(); }
        
        let paragraph = |run: Run, txt, size| -> Paragraph
        {
            let elem = run.add_text(txt).size(size);  // `size` pt
            Paragraph::new().add_run(elem)
        };

        // Student Information
        let st = paragraph(body_run.clone(), format!("{}: {}        {}: {}", header.get_name(), student.get_name(), header.get_id(), student.get_id()), body_font_size);

        // Blank line
        let blank_line = Paragraph::new();

        docx = docx.add_paragraph(ex);
        docx = docx.add_paragraph(st);
        docx = docx.add_paragraph(blank_line.clone());

        // Instructions (Handle multi-line notice)
        for line in header.get_notice().lines()
        {
            let notice_para = paragraph(body_run.clone(), format!("{}", line), body_font_size);
            docx = docx.add_paragraph(notice_para);
        }

        docx = docx.add_paragraph(blank_line.clone());

        for (question_index, question) in qbank.get_questions().iter().enumerate()
        {
            let category = header.get_category(question.get_category()).map(|s| s.as_str()).unwrap_or("");
            let mut q_para = Paragraph::new();
            let mut lines = question.get_question().lines().peekable();
            let mut is_first_line = true;
            while let Some(line) = lines.next()
            {
                let mut run = body_run.clone().add_text(if is_first_line { format!("{}. [{}]   {}", question_index + 1, category, line) } else { line.to_string() }).size(body_font_size);
                if lines.peek().is_some()
                    { run = run.add_break(docx_rs::BreakType::TextWrapping); }
                q_para = q_para.add_run(run);
                is_first_line = false;
            }
            docx = docx.add_paragraph(q_para);

            let category_id = question.get_category();
            if category_id == 3 {
                // Short answer: ( space * 3 * max_choice_len )
                let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                let spaces = " ".repeat(max_len * 3);
                let c_para = paragraph(body_run.clone(), format!("    ({})", spaces), body_font_size);
                docx = docx.add_paragraph(c_para);
            } else if category_id == 4 {
                // Essay: 15 blank lines
                for _ in 0..15 {
                    docx = docx.add_paragraph(blank_line.clone());
                }
            } else {
                // Category 1, 2: Standard choices
                for (choice_index, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
                {
                    let mut c_para = Paragraph::new();
                    let mut lines = choice_text.lines().peekable();
                    let mut is_first_line = true;
                    while let Some(line) = lines.next()
                    {
                        let mut run = body_run.clone().add_text(if is_first_line { format!("    ({}) {}", choice_index + 1, line) } else { line.to_string() }).size(body_font_size);
                        if lines.peek().is_some()
                            { run = run.add_break(docx_rs::BreakType::TextWrapping); }
                        c_para = c_para.add_run(run);
                        is_first_line = false;
                    }
                    docx = docx.add_paragraph(c_para);
                }
            }
            // Blank line after each question
            docx = docx.add_paragraph(blank_line.clone());
        }
        Ok(docx)
    }
    
    // pub fn save_shuffled_exams_in_hwpx(&self, path: &Path) -> Result<(), String>
    /// Saves the shuffled exam sets to a HWPX file.
    ///
    /// This function generates a HWPX document containing the shuffled exam
    /// sets for all students, with specified page margins and a footer
    /// with page numbers.
    /// 
    /// # Arguments
    /// * `path` - The file path where the HWPX document will be saved.
    /// 
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    /// let result = generator.save_shuffled_exams_in_hwpx(Path::new("exam.hwpx"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam.hwpx").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_shuffled_exams_in_hwpx(&self, path: &Path) -> Result<(), String>
    {
        self.build_shuffled_exams_in_hwpx()?
            .save_to_file(path)
            .map_err(|e| e.to_string())
    }

    // pub fn export_shuffled_exams_in_hwpx(&self) -> Result<Vec<u8>, String>
    /// Exports the shuffled exam sets to a Vec<u8> object.
    ///
    /// This function generates a HWPX document containing the shuffled exam
    /// sets for all students, with specified page margins and a footer
    /// with page numbers, and returns the document as a Vec<u8> object.
    /// 
    /// # Output
    /// `Result<Vec<u8>, String>` - Returns `Ok(Vec<u8>)` containing the HWPX
    ///                             document on success, or an `Err` with a
    ///                             `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    /// let result = generator.export_shuffled_exams_in_hwpx();
    /// assert!(result.is_ok());
    /// std::fs::write("exam_exported.hwpx", result.unwrap()).unwrap();
    /// std::fs::remove_file("exam_exported.hwpx").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn export_shuffled_exams_in_hwpx(&self) -> Result<Vec<u8>, String>
    {
        self.build_shuffled_exams_in_hwpx()?
            .to_bytes()
            .map_err(|e| e.to_string())
    }

    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn build_shuffled_exams_in_hwpx(&self) -> Result<HwpxWriter, String>
    {
        let mut hwpx = HwpxWriter::new();
        hwpx.add_footer_with_page_number(" ");
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (student, qbank) in &shuffled_qbanks
        {
            self.write_exam_content_to_hwpx(&mut hwpx, student, qbank)?;
        }

        // Add answer sheet
        let title_style = HwpxTextStyle::new()
            .size(self.title_font_size.round() as u32);
        let title_style = if self.is_title_bold() { title_style.bold() } else { title_style };
        let title_style = if self.is_title_italic() { title_style.italic() } else { title_style };
        let title_style = if self.is_title_underline() { title_style.underline() } else { title_style };
        let title_style = if self.is_title_strike() { title_style.strikethrough() } else { title_style };

        hwpx.add_mixed_styled_paragraph(vec![StyledText {
            text: self.answer_sheet_title.clone(),
            style: title_style,
        }]).map_err(|e| e.to_string())?;

        hwpx.add_paragraph("").map_err(|e| e.to_string())?;

        let answer_sheet_style = HwpxTextStyle::new()
            .size(self.answer_sheet_font_size.round() as u32);
        let answer_sheet_style = if self.is_answer_sheet_bold() { answer_sheet_style.bold() } else { answer_sheet_style };
        let answer_sheet_style = if self.is_answer_sheet_italic() { answer_sheet_style.italic() } else { answer_sheet_style };
        let answer_sheet_style = if self.is_answer_sheet_underline() { answer_sheet_style.underline() } else { answer_sheet_style };
        let answer_sheet_style = if self.is_answer_sheet_strike() { answer_sheet_style.strikethrough() } else { answer_sheet_style };

        let header = self.shuffler.get_header();
        for (student, qbank) in &shuffled_qbanks
        {
            let student_info_text = format!("{}: {}        {}: {}",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()
            );
            hwpx.add_mixed_styled_paragraph(vec![StyledText {
                text: student_info_text,
                style: answer_sheet_style.clone(),
            }]).map_err(|e| e.to_string())?;

            let mut answers_text = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let category_id = question.get_category();
                let answer_string = if category_id == 3 {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                } else if category_id == 4 {
                    "(---)".to_string()
                } else {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                };
                answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
            }
            hwpx.add_mixed_styled_paragraph(vec![StyledText {
                text: answers_text,
                style: answer_sheet_style.clone(),
            }]).map_err(|e| e.to_string())?;
            hwpx.add_paragraph("").map_err(|e| e.to_string())?;
        }
        Ok(hwpx)
    }

    // fn write_exam_content_to_hwpx(&self, hwpx: &mut HwpxWriter, student: &Student, qbank: &QBank) -> Result<(), String>
    /// Writes the formatted exam content for a single student to a HWPX document.
    ///
    /// This private helper function takes a mutable HWPX `HwpxWriter` object and
    /// appends the exam content for the given student and their shuffled
    /// question bank, applying HWPX-specific formatting such as font sizes.
    ///
    /// # Arguments
    /// * `hwpx` - A mutable reference to the `hwpers::HwpxWriter` object.
    /// * `student` - A reference to the `Student` for whom the exam content is
    ///               being written.
    /// * `qbank` - A reference to the `QBank` containing the shuffled questions
    ///             for this student.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn write_exam_content_to_hwpx(&self, hwpx: &mut HwpxWriter, student: &Student, qbank: &QBank) -> Result<(), String>
    {
        let header = qbank.get_header();
        let title_style = HwpxTextStyle::new()
            .size(self.title_font_size.round() as u32);
        let title_style = if self.is_title_bold() { title_style.bold() } else { title_style };
        let title_style = if self.is_title_italic() { title_style.italic() } else { title_style };
        let title_style = if self.is_title_underline() { title_style.underline() } else { title_style };
        let title_style = if self.is_title_strike() { title_style.strikethrough() } else { title_style };

        hwpx.add_mixed_styled_paragraph(vec![StyledText {
            text: header.get_title().to_string(),
            style: title_style,
        }]).map_err(|e| e.to_string())?;

        let body_style = HwpxTextStyle::new()
            .size(self.body_font_size.round() as u32);
        let body_style = if self.is_body_bold() { body_style.bold() } else { body_style };
        let body_style = if self.is_body_italic() { body_style.italic() } else { body_style };
        let body_style = if self.is_body_underline() { body_style.underline() } else { body_style };
        let body_style = if self.is_body_strike() { body_style.strikethrough() } else { body_style };

        let student_info = format!("{}: {}        {}: {}", 
            header.get_name(), student.get_name(), header.get_id(), student.get_id());
        hwpx.add_mixed_styled_paragraph(vec![StyledText {
            text: student_info,
            style: body_style.clone(),
        }]).map_err(|e| e.to_string())?;

        hwpx.add_paragraph("").map_err(|e| e.to_string())?;

        // Instructions (Handle multi-line notice)
        for line in header.get_notice().lines()
        {
            hwpx.add_mixed_styled_paragraph(vec![StyledText {
                text: line.to_string(),
                style: body_style.clone(),
            }]).map_err(|e| e.to_string())?;
        }
        hwpx.add_paragraph("").map_err(|e| e.to_string())?;

        for (question_index, question) in qbank.get_questions().iter().enumerate()
        {
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s| s.as_str()).unwrap_or("");
            let q_text = format!("{}. [{}]   {}", question_index + 1, category_text, question.get_question());
            hwpx.add_mixed_styled_paragraph(vec![StyledText {
                text: q_text,
                style: body_style.clone(),
            }]).map_err(|e| e.to_string())?;

            if category_id == 3 {
                // Short answer: ( space * 3 * max_choice_len )
                let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                let spaces = " ".repeat(max_len * 3);
                hwpx.add_mixed_styled_paragraph(vec![StyledText {
                    text: format!("    ({})", spaces),
                    style: body_style.clone(),
                }]).map_err(|e| e.to_string())?;
            } else if category_id == 4 {
                // Essay: 15 blank lines
                for _ in 0..15 {
                    hwpx.add_paragraph("").map_err(|e| e.to_string())?;
                }
            } else {
                // Category 1, 2: Standard choices
                for (choice_index, (choice_text, _)) in question.get_choices().iter().enumerate()
                {
                    let c_text = format!("    ({}) {}", choice_index + 1, choice_text);
                    hwpx.add_mixed_styled_paragraph(vec![StyledText {
                        text: c_text,
                        style: body_style.clone(),
                    }]).map_err(|e| e.to_string())?;
                }
            }
            hwpx.add_paragraph("").map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    // pub fn save_shuffled_exams_in_hwp(&self, path: &Path) -> Result<(), String>
    /// Saves the shuffled exam sets to a HWP file.
    ///
    /// This function generates a HWP document containing the shuffled exam
    /// sets for all students, with specified page margins and a footer
    /// with page numbers.
    /// 
    /// # Arguments
    /// * `path` - The file path where the HWP document will be saved.
    /// 
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    /// let result = generator.save_shuffled_exams_in_hwp(Path::new("exam.hwp"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam.hwp").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_shuffled_exams_in_hwp(&self, path: &Path) -> Result<(), String>
    {
        self.build_shuffled_exams_in_hwp()?
            .save_to_file(path)
            .map_err(|e| e.to_string())
    }

    // pub fn export_shuffled_exams_in_hwp(&self) -> Result<Vec<u8>, String>
    /// Exports the shuffled exam sets to a Vec<u8> object.
    ///
    /// This function generates a HWP document containing the shuffled exam
    /// sets for all students, with specified page margins and a footer
    /// with page numbers, and returns the document as a Vec<u8> object.
    /// 
    /// # Output
    /// `Result<Vec<u8>, String>` - Returns `Ok(Vec<u8>)` containing the HWP
    ///                             document on success, or an `Err` with a
    ///                             `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    /// let result = generator.export_shuffled_exams_in_hwp();
    /// assert!(result.is_ok());
    /// std::fs::write("exam_exported.hwp", result.unwrap()).unwrap();
    /// std::fs::remove_file("exam_exported.hwp").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn export_shuffled_exams_in_hwp(&self) -> Result<Vec<u8>, String>
    {
        self.build_shuffled_exams_in_hwp()?
            .to_bytes()
            .map_err(|e| e.to_string())
    }

    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn build_shuffled_exams_in_hwp(&self) -> Result<HwpWriter, String>
    {
        let mut hwp = HwpWriter::new();
        hwp.set_page_margins_mm(
            self.margin_left_in_mm,
            self.margin_right_in_mm,
            self.margin_top_in_mm,
            self.margin_buttom_in_mm,
        );
        hwp.add_footer_with_page_number(" ", hwpers::model::PageNumberFormat::Numeric);
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (student, qbank) in &shuffled_qbanks
        {
            self.write_exam_content_to_hwp(&mut hwp, student, qbank)?;
        }

        // Add answer sheet
        let title_style = TextStyle::new()
            .size(self.title_font_size.round() as u32);
        let title_style = if self.is_title_bold() { title_style.bold() } else { title_style };
        let title_style = if self.is_title_italic() { title_style.italic() } else { title_style };
        let title_style = if self.is_title_underline() { title_style.underline() } else { title_style };
        let title_style = if self.is_title_strike() { title_style.strikethrough() } else { title_style };

        let mut title_styled = hwpers::writer::style::StyledText::new(self.answer_sheet_title.clone());
        title_styled = title_styled.add_range(0, self.answer_sheet_title.len(), title_style);
        hwp.add_styled_paragraph(&title_styled).map_err(|e| e.to_string())?;

        hwp.add_paragraph("").map_err(|e| e.to_string())?;

        let answer_sheet_style = TextStyle::new()
            .size(self.answer_sheet_font_size.round() as u32);
        let answer_sheet_style = if self.is_answer_sheet_bold() { answer_sheet_style.bold() } else { answer_sheet_style };
        let answer_sheet_style = if self.is_answer_sheet_italic() { answer_sheet_style.italic() } else { answer_sheet_style };
        let answer_sheet_style = if self.is_answer_sheet_underline() { answer_sheet_style.underline() } else { answer_sheet_style };
        let answer_sheet_style = if self.is_answer_sheet_strike() { answer_sheet_style.strikethrough() } else { answer_sheet_style };

        let header = self.shuffler.get_header();
        for (student, qbank) in &shuffled_qbanks
        {
            let student_info_text = format!("{}: {}        {}: {}",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()
            );
            let mut student_info_styled = hwpers::writer::style::StyledText::new(student_info_text.clone());
            student_info_styled = student_info_styled.add_range(0, student_info_text.len(), answer_sheet_style.clone());
            hwp.add_styled_paragraph(&student_info_styled).map_err(|e| e.to_string())?;

            let mut answers_text = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let category_id = question.get_category();
                let answer_string = if category_id == 3 {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                } else if category_id == 4 {
                    "(---)".to_string()
                } else {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                };
                answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
            }
            let mut answers_styled = hwpers::writer::style::StyledText::new(answers_text.clone());
            answers_styled = answers_styled.add_range(0, answers_text.len(), answer_sheet_style.clone());
            hwp.add_styled_paragraph(&answers_styled).map_err(|e| e.to_string())?;
            hwp.add_paragraph("").map_err(|e| e.to_string())?;
        }
        Ok(hwp)
    }

    // fn write_exam_content_to_hwp(&self, hwp: &mut HwpWriter, student: &Student, qbank: &QBank) -> Result<(), String>
    /// Writes the formatted exam content for a single student to a HWP document.
    ///
    /// This private helper function takes a mutable HWP `HwpWriter` object and
    /// appends the exam content for the given student and their shuffled
    /// question bank, applying HWP-specific formatting such as font sizes.
    ///
    /// # Arguments
    /// * `hwp` - A mutable reference to the `hwpers::HwpWriter` object.
    /// * `student` - A reference to the `Student` for whom the exam content is
    ///               being written.
    /// * `qbank` - A reference to the `QBank` containing the shuffled questions
    ///             for this student.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn write_exam_content_to_hwp(&self, hwp: &mut HwpWriter, student: &Student, qbank: &QBank) -> Result<(), String>
    {
        let header = qbank.get_header();
        let title_style = TextStyle::new()
            .size(self.title_font_size.round() as u32);
        let title_style = if self.is_title_bold() { title_style.bold() } else { title_style };
        let title_style = if self.is_title_italic() { title_style.italic() } else { title_style };
        let title_style = if self.is_title_underline() { title_style.underline() } else { title_style };
        let title_style = if self.is_title_strike() { title_style.strikethrough() } else { title_style };

        let title_text = header.get_title();
        let mut title_styled = hwpers::writer::style::StyledText::new(title_text.to_string());
        title_styled = title_styled.add_range(0, title_text.len(), title_style);
        hwp.add_styled_paragraph(&title_styled).map_err(|e| e.to_string())?;

        let body_style = TextStyle::new()
            .size(self.body_font_size.round() as u32);
        let body_style = if self.is_body_bold() { body_style.bold() } else { body_style };
        let body_style = if self.is_body_italic() { body_style.italic() } else { body_style };
        let body_style = if self.is_body_underline() { body_style.underline() } else { body_style };
        let body_style = if self.is_body_strike() { body_style.strikethrough() } else { body_style };

        let student_info = format!("{}: {}        {}: {}", 
            header.get_name(), student.get_name(), header.get_id(), student.get_id());
        let mut student_info_styled = hwpers::writer::style::StyledText::new(student_info.clone());
        student_info_styled = student_info_styled.add_range(0, student_info.len(), body_style.clone());
        hwp.add_styled_paragraph(&student_info_styled).map_err(|e| e.to_string())?;

        hwp.add_paragraph("").map_err(|e| e.to_string())?;

        // Instructions (Handle multi-line notice)
        for line in header.get_notice().lines()
        {
            let mut line_styled = hwpers::writer::style::StyledText::new(line.to_string());
            line_styled = line_styled.add_range(0, line.len(), body_style.clone());
            hwp.add_styled_paragraph(&line_styled).map_err(|e| e.to_string())?;
        }
        hwp.add_paragraph("").map_err(|e| e.to_string())?;

        for (question_index, question) in qbank.get_questions().iter().enumerate()
        {
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s| s.as_str()).unwrap_or("");
            let q_text = format!("{}. [{}]   {}", question_index + 1, category_text, question.get_question());
            let mut q_styled = hwpers::writer::style::StyledText::new(q_text.clone());
            q_styled = q_styled.add_range(0, q_text.len(), body_style.clone());
            hwp.add_styled_paragraph(&q_styled).map_err(|e| e.to_string())?;

            if category_id == 3 {
                // Short answer: ( space * 3 * max_choice_len )
                let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                let spaces = " ".repeat(max_len * 3);
                let c_text = format!("    ({})", spaces);
                let mut c_styled = hwpers::writer::style::StyledText::new(c_text.clone());
                c_styled = c_styled.add_range(0, c_text.len(), body_style.clone());
                hwp.add_styled_paragraph(&c_styled).map_err(|e| e.to_string())?;
            } else if category_id == 4 {
                // Essay: 15 blank lines
                for _ in 0..15 {
                    hwp.add_paragraph("").map_err(|e| e.to_string())?;
                }
            } else {
                // Category 1, 2: Standard choices
                for (choice_index, (choice_text, _)) in question.get_choices().iter().enumerate()
                {
                    let c_text = format!("    ({}) {}", choice_index + 1, choice_text);
                    let mut c_styled = hwpers::writer::style::StyledText::new(c_text.clone());
                    c_styled = c_styled.add_range(0, c_text.len(), body_style.clone());
                    hwp.add_styled_paragraph(&c_styled).map_err(|e| e.to_string())?;
                }
            }
            hwp.add_paragraph("").map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    // pub fn save_shuffled_exams_in_pdf(&self, path: &Path) -> Result<(), String>
    /// Saves the shuffled exam sets to a PDF file.
    ///
    /// This function generates a PDF document containing the shuffled exam sets
    /// for all students, with a footer showing page numbers.
    ///
    /// # Arguments
    /// * `path` - The file path where the PDF document will be saved.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    /// `String` describing the error on failure.
    ///
    /// # Caution
    /// - This method searches for four specific font files within a `./fonts` 
    ///   subdirectory relative to the current working directory.
    /// - The attributes of underline and strike are not working.
    ///
    /// The following files must be present for the function to operate correctly:
    /// * `font-Regular.ttf`
    /// * `font-Italic.ttf`
    /// * `font-Bold.ttf`
    /// * `font-BoldItalic.ttf`
    ///
    /// If the directory or any of these files are missing, the function will fail. 
    /// Ensure that the `fonts` directory is created and all four files are 
    /// correctly named before calling this method.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    ///
    /// let result = generator.save_shuffled_exams_in_pdf(Path::new("exam.pdf"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam.pdf").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_shuffled_exams_in_pdf(&self, path: &Path) -> Result<(), String>
    {
        let font_family = fonts::from_files("./fonts", "font", None).map_err(|e| format!("Failed to load font: {}", e))?;
        let mut doc = Document::new(font_family);
        // Set 1cm margins (10mm) and page numbers for all sides
        let mut decorator = SimplePageDecorator::new();
        let margin = (self.margin_left_in_mm + self.margin_right_in_mm + self.margin_top_in_mm + self.margin_buttom_in_mm) / 4.0;
        decorator.set_margins(margin); // 10mm = 1cm
        doc.set_page_decorator(decorator);
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (idx, (student, qbank)) in shuffled_qbanks.iter().enumerate()
        {
            if idx > 0
                { doc.push(elements::PageBreak::new()); } // Page break for subsequent students
            self.write_exam_content_to_pdf(&mut doc, &student, &qbank)?;
        }

        // Add answer sheet
        doc.push(elements::PageBreak::new());
        let mut answer_style = style::Style::new();
        answer_style.set_font_size(self.answer_sheet_font_size as u8);
        if self.is_answer_sheet_bold()
            { answer_style.set_bold(); }
        if self.is_answer_sheet_italic()
            { answer_style.set_italic(); }
        let mut answer_title_style = style::Style::new();
        answer_title_style.set_font_size(self.title_font_size as u8);
        if self.is_title_bold()
            { answer_title_style.set_bold(); }
        if self.is_title_italic()
            { answer_title_style.set_italic(); }
        
        let mut title_paragraph = elements::Paragraph::new(self.answer_sheet_title.clone());
        title_paragraph.set_alignment(Alignment::Center);
        doc.push(title_paragraph.styled(answer_title_style));
        doc.push(elements::Paragraph::new("")); // Blank line

        let header = self.shuffler.get_header();

        for (student, qbank) in &shuffled_qbanks {
            // Student Info
            let student_info_text = format!("{}: {}        {}: {}",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()
            );
            doc.push(elements::Paragraph::new(student_info_text).styled(answer_style));

            // Answers
            let mut answers_text = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let category_id = question.get_category();
                let answer_string = if category_id == 3 {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                } else if category_id == 4 {
                    "(---)".to_string()
                } else {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                };
                answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
            }
            doc.push(elements::Paragraph::new(answers_text).styled(answer_style));
            doc.push(elements::Paragraph::new("")); // Blank line
        }

        doc.render_to_file(path).map_err(|e: Error| e.to_string())?;
        Ok(())
    }

    // fn write_exam_content_to_pdf(&self, doc: &mut genpdfi::Document, student: &Student, qbank: &QBank) -> Result<(), String>
    /// Writes the formatted exam content for a single student to a PDF document.
    ///
    /// This private helper function takes a mutable PDF `genpdfi::Document` object
    /// and appends the exam content for the given student and their shuffled
    /// question bank, applying PDF-specific formatting such as font sizes.
    ///
    /// # Arguments
    /// * `doc` - A mutable reference to the `genpdfi::Document` object.
    /// * `student` - A reference to the `Student` for whom the exam content is being written.
    /// * `qbank` - A reference to the `QBank` containing the shuffled questions for this student.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    /// 
    /// # Caution
    /// - The attributes of underline and strike are not working.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn write_exam_content_to_pdf(&self, doc: &mut genpdfi::Document, student: &Student, qbank: &QBank) -> Result<(), String>
    {
        // Define font sizes
        let title_font_size = self.title_font_size as u8;       // 14 pt for default
        let normal_font_size = self.body_font_size as u8;    // 11 pt for default
        let header = qbank.get_header();

        let mut title_style = style::Style::new();
        title_style.set_font_size(title_font_size);
        if self.is_title_bold()
            { title_style.set_bold(); }
        if self.is_title_italic()
            { title_style.set_italic(); }

        // Exam Title
        let mut title_paragraph = elements::Paragraph::new(format!("{}", header.get_title()));
        title_paragraph.set_alignment(Alignment::Center);
        doc.push(title_paragraph.styled(title_style));

        let mut body_style = style::Style::new();
        body_style.set_font_size(normal_font_size);
        if self.is_body_bold()
            { body_style.set_bold(); }
        if self.is_body_italic()
            { body_style.set_italic(); }
        
        // Student Information
        doc.push(elements::Paragraph::new(format!("{}: {}        {}: {}", header.get_name(), student.get_name(), header.get_id(), student.get_id())).styled(body_style));
        doc.push(elements::Paragraph::new("")); // Blank line

        // Instructions (Handle multi-line notice)
        for line in header.get_notice().lines()
        {
            doc.push(elements::Paragraph::new(format!("{}", line)).styled(body_style));
        }
        doc.push(elements::Paragraph::new("")); // Blank line after notice

        for (question_index, question) in qbank.get_questions().iter().enumerate()
        {
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s| s.as_str()).unwrap_or("");
            doc.push(elements::Paragraph::new(format!("{}. [{}]   {}", question_index + 1, category_text, question.get_question())).styled(body_style));
            
            if category_id == 3 {
                // Short answer: ( space * 3 * max_choice_len )
                let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                let spaces = " ".repeat(max_len * 3);
                doc.push(elements::Paragraph::new(format!("    ({})", spaces)).styled(body_style));
            } else if category_id == 4 {
                // Essay: 15 blank lines
                for _ in 0..15 {
                    doc.push(elements::Paragraph::new(""));
                }
            } else {
                // Category 1, 2: Standard choices
                for (choice_index, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
                {
                    let choice_char = (choice_index + 1).to_string();
                    doc.push(elements::Paragraph::new(format!("    ({}) {}", choice_char, choice_text)).styled(body_style));
                }
            }
            doc.push(elements::Paragraph::new("")); // Blank line after each question
        }
        Ok(())
    }

    // pub fn export_shuffled_exams_in_pdf(&self) -> Result<Vec<u8>, String>
    /// Exports the shuffled exam sets as a PDF-compatible JSON byte vector.
    ///
    /// This function generates a JSON structure that can be used with the `pdfmake`
    /// library ot javascript to create a PDF document.
    /// It includes the shuffled exam sets for all students and an answer sheet.
    ///
    /// # Returns
    /// `Result<Vec<u8>, String>` - Returns a byte vector containing the JSON string
    ///                            on success, or an `Err` with a `String`
    ///                            describing the error on failure.
    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    pub fn export_shuffled_exams_in_pdf(&self) -> Result<Vec<u8>, String>
    {
        use serde_json::json;
        let shuffled_qsets: Vec<(Student, QBank)> = self.get_shuffled_qbanks();
        let mut content = Vec::new();

        let title_font_size = self.title_font_size;
        let body_font_size = self.body_font_size;
        let answer_sheet_font_size = self.answer_sheet_font_size;

        let title_style = json!({
            "fontSize": title_font_size,
            "bold": self.is_title_bold(),
            "italics": self.is_title_italic(),
            "decoration": if self.is_title_underline() { Some("underline") } else if self.is_title_strike() { Some("lineThrough") } else { None },
            "alignment": "center",
            "margin": [0, 0, 0, 10]
        });

        let body_style = json!({
            "fontSize": body_font_size,
            "bold": self.is_body_bold(),
            "italics": self.is_body_italic(),
            "decoration": if self.is_body_underline() { Some("underline") } else if self.is_body_strike() { Some("lineThrough") } else { None },
            "margin": [0, 2, 0, 2]
        });

        let answer_sheet_style = json!({
            "fontSize": answer_sheet_font_size,
            "bold": self.is_answer_sheet_bold(),
            "italics": self.is_answer_sheet_italic(),
            "decoration": if self.is_answer_sheet_underline() { Some("underline") } else if self.is_answer_sheet_strike() { Some("lineThrough") } else { None },
            "margin": [0, 2, 0, 2]
        });

        for (idx, (student, qbank)) in shuffled_qsets.iter().enumerate()
        {
            let student: &Student = student;
            let qbank: &QBank = qbank;
            if idx > 0
                { content.push(json!({"text": "", "pageBreak": "before"})); }

            let header = qbank.get_header();
            
            // Title
            content.push(json!({"text": header.get_title(), "style": "title"}));

            // Student Info
            content.push(json!({
                "text": format!("{}: {}        {}: {}", header.get_name(), student.get_name(), header.get_id(), student.get_id()),
                "style": "body"
            }));
            content.push(json!({"text": "\n"}));

            // Notice
            for line in header.get_notice().lines()
            {
                content.push(json!({"text": line, "style": "body"}));
            }
            content.push(json!({"text": "\n"}));

            // Questions
            for (question_index, question) in qbank.get_questions().iter().enumerate()
            {
                let category_id = question.get_category();
                let category_text = header.get_category(category_id).map(|s: &String| s.as_str()).unwrap_or("");
                content.push(json!({
                    "text": format!("{}. [{}]   {}", question_index + 1, category_text, question.get_question()),
                    "style": "body"
                }));

                if category_id == 3 {
                    // Short answer: ( space * 3 * max_choice_len )
                    let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                    let spaces = " ".repeat(max_len * 3);
                    content.push(json!({
                        "text": format!("    ({})", spaces),
                        "style": "body"
                    }));
                } else if category_id == 4 {
                    // Essay: 15 blank lines
                    for _ in 0..15 {
                        content.push(json!({"text": "\n"}));
                    }
                } else {
                    // Category 1, 2: Standard choices
                    for (choice_index, (choice_text, _)) in question.get_choices().iter().enumerate()
                    {
                        content.push(json!({
                            "text": format!("    ({}) {}", choice_index + 1, choice_text),
                            "style": "body"
                        }));
                    }
                }
                content.push(json!({"text": "\n"}));
            }
        }

        // Answer Sheet
        content.push(json!({"text": "", "pageBreak": "before"}));
        content.push(json!({"text": self.answer_sheet_title.as_str(), "style": "title"}));

        let header = self.shuffler.get_header();
        for (student, qbank) in &shuffled_qsets
        {
            let student: &Student = student;
            let qbank: &QBank = qbank;
            content.push(json!({
                "text": format!("{}: {}        {}: {}", header.get_name(), student.get_name(), header.get_id(), student.get_id()),
                "style": "answer_sheet"
            }));

            let mut answers_text = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate()
            {
                let category_id = question.get_category();
                let answer_string = if category_id == 3 {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                } else if category_id == 4 {
                    "(---)".to_string()
                } else {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct)): &(usize, &(String, bool))| *is_correct)
                        .map(|(j, _): (usize, &(String, bool))| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                };
                answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
            }
            content.push(json!({"text": answers_text, "style": "answer_sheet"}));
            content.push(json!({"text": "\n"}));
        }

        let doc_definition = json!({
            "content": content,
            "styles": {
                "title": title_style,
                "body": body_style,
                "answer_sheet": answer_sheet_style
            }
        });

        serde_json::to_vec(&doc_definition).map_err(|e| e.to_string())
    }
}