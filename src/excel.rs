// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////



use calamine::DataType;

use crate::{ Question, Choices, check_path };


/// Represents an Excel file for question bank operations.
///
/// This struct provides methods to read from and write to `.xlsx` files,
/// structuring the data into "header" and "bank" sheets.
#[derive(Debug, Clone)]
pub struct Excel
{
    /// The path to the Excel file.
    pub(crate) path: String,
}

impl Excel
{
    // pub fn open(path: String, extention: &str) -> Option<Self>
    /// Creates a new `Excel` instance with a given path.
    ///
    /// # Arguments
    /// * `path` - The file path for the Excel workbook.
    /// * `extention` - The file extension to append if the path does not have one.
    ///
    /// # Output
    /// An `Option<Self>` containing the `Excel` instance.
    /// 
    /// # Features
    /// If the path does not have an extension, `extention` is appended.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let excel_handler = Excel::open("my_quiz".to_string(), ".qb.xlsx");
    /// assert!(excel_handler.is_some());
    /// assert_eq!(excel_handler.unwrap().get_path(), "my_quiz.qb.xlsx");
    /// ```
    #[inline]
    pub fn open_with_ext(path: String, extention: &str) -> Option<Self>
    {
        Some(Self { path: check_path(path, extention) })
    }

    // pub fn get_path(&self) -> &String
    /// Gets the path of the Excel file.
    ///
    /// # Output
    /// `&String` - A reference to the path of the Excel file.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let db = Excel::open("my".to_string(), ".qb.xlsx").unwrap();
    /// assert_eq!(db.get_path(), "my.qb.xlsx");
    /// ```
    pub fn get_path(&self) -> &String
    {
        &self.path
    }
    
    // pub(crate) fn parse_question_row(row: &[calamine::Data]) -> Option<Question>
    /// Parses a single row from an Excel sheet into a `Question` struct.
    ///
    /// This function takes a slice of `calamine::Data` representing a single row
    /// from the "Questions" sheet and attempts to parse it into a `Question`.
    ///
    /// # Arguments
    /// * `row` - A slice of `calamine::Data` representing the cells of a single row.
    ///   It expects the cells to be in the order: ID, Category, Question Text,
    ///   followed by pairs of Choice Text and IsAnswer.
    ///
    /// # Output
    /// * `Some(Question)` if the row is successfully parsed.
    /// * `None` if essential data (ID, Category, Question Text) is missing or has
    ///   an incorrect data type.
    ///
    /// # Examples
    /// ```ignore
    /// // This function is `pub(crate)` and its example cannot be directly tested in `cargo test --doc`.
    /// // It is typically used internally by `QBDB for Excel` implementations.
    /// use qrate::{Excel, Question, Choices};
    /// use calamine::Data;
    ///
    /// // Simulate a row from an Excel sheet: ID, Category, Question, Choice1, IsAnswer1, Choice2, IsAnswer2
    /// let row_data = vec![
    ///     Data::Float(1.0), // ID
    ///     Data::Float(1.0), // Group
    ///     Data::Float(10.0), // Category
    ///     Data::String("What is the capital of France?".to_string()), // Question Text
    ///     Data::String("Berlin".to_string()), Data::Bool(false), // Choice 1
    ///     Data::String("Paris".to_string()), Data::Bool(true),  // Choice 2
    ///     Data::String("Rome".to_string()), Data::Bool(false),  // Choice 3
    /// ];
    ///
    /// let question = Excel::parse_question_row(&row_data).unwrap();
    ///
    /// assert_eq!(question.get_id(), 1);
    /// assert_eq!(question.get_group(), 1); // Added group assertion
    /// assert_eq!(question.get_category(), 10);
    /// assert_eq!(question.get_question(), "What is the capital of France?");
    ///
    /// let expected_choices = Choices::from(vec![
    ///     ("Berlin".to_string(), false),
    ///     ("Paris".to_string(), true),
    ///     ("Rome".to_string(), false),
    /// ]);
    /// assert_eq!(question.get_choices(), &expected_choices);
    /// ```
    pub(crate) fn parse_question_row(row: &[calamine::Data]) -> Option<Question>
    {
        let id = row.get(0).and_then(|d| d.as_f64()).map(|f| f as u16)?;
        let group = row.get(1).and_then(|d| d.as_f64()).map(|f| f as u16)?;
        let category = row.get(2).and_then(|d| d.as_f64()).map(|f| f as u8)?;
        let question_text = row.get(3).and_then(|d| d.as_string())?;
        
        let mut choices = Choices::new();
        for choice_pair in row.get(4..).unwrap_or(&[]).chunks(2)
        {
            let choice_text = choice_pair.get(0).and_then(|d| d.get_string()).map(|s| s.to_string()).unwrap_or_default();
            let is_answer = choice_pair.get(1).and_then(|d| d.get_bool()).unwrap_or_else(|| {
                choice_pair.get(1).and_then(|d| d.get_string())
                    .map_or(false, |s| s.eq_ignore_ascii_case("TRUE"))
            });

            if !choice_text.is_empty() || is_answer
            {
                choices.push((choice_text, is_answer));
            }
            else
            {
                break;
            }
        }
        Some(Question::new(id, group, category, question_text, choices))
    }
}