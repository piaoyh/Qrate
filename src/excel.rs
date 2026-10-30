// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::io::Cursor;
use calamine::{ DataType, Xlsx, Reader };
use rust_xlsxwriter::{ Format, FormatBorder, Workbook };

use crate::{ Question, Choices, ErrorMessage, check_path };


/// Represents an Excel file for question bank operations.
///
/// This struct provides methods to read from and write to `.xlsx` files,
/// structuring the data into "header" and "bank" sheets.
pub struct Excel
{
    /// The path to the Excel file.
    pub(crate) path: String,

    /// The workbook object for reading Excel files.
    pub(crate) workbook_to_read: Option<Xlsx<Cursor<Vec<u8>>>>,

    /// The workbook object for writing Excel files.
    pub(crate) workbook_to_write: Workbook,

    /// The format for bold borders.
    pub(crate) bold_border_format: Format,

    /// The format for thin borders.
    pub(crate) thin_border_format: Format,
}

impl Excel
{
    // pub fn open_with_ext(path: String, extention: &str) -> Self
    /// Creates a new `Excel` instance with a given path.
    ///
    /// # Arguments
    /// * `path` - The file path for the Excel workbook.
    /// * `extention` - The file extension to append
    ///   if the path does not have one.
    ///
    /// # Returns
    /// `Self` - An instance of `Excel` with the specified path and
    /// initialized workbooks.
    /// 
    /// # Features
    /// If the path does not have an extension, `extention` is appended.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let excel_handler = Excel::open_with_ext("my_quiz".to_string(), ".qb.xlsx");
    /// assert!(excel_handler.is_ok());
    /// assert_eq!(excel_handler.unwrap().get_path(), "my_quiz.qb.xlsx");
    /// ```
    #[inline]
    pub fn open_with_ext(path: String, extention: &str) -> Self
    {
        Self
        {
            path: check_path(path, extention),
            workbook_to_read: None,
            workbook_to_write: Workbook::new(),
            bold_border_format: Format::new().set_bold().set_border(FormatBorder::Thick),
            thin_border_format: Format::new().set_border(FormatBorder::Thin),
        }
    }

    // pub fn open_in_memory(data: &[u8]) -> Result<Self, ErrorMessage>
    /// Opens a new `Excel` instance to an Excel database in memory.
    ///
    /// # Arguments
    /// * `data` - contains the contents of the Excel database opened outside
    /// of the application and read.
    ///
    /// # Output
    /// An `Result<Self, ErrorMessage>` which is:
    /// * `Ok(Excel)` on successful connection, or
    /// * `Err(ErrorMessage::FailedToReceiveDatabaseFromMemory)` on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    /// 
    /// // Simulate reading an Excel file into a byte vector
    /// let excel_data: Vec<u8> = vec![/* Excel file bytes */];
    /// let db = Excel::open_in_memory(&excel_data);
    /// assert!(db.is_ok());
    /// ```
    pub fn open_in_memory(data: &[u8]) -> Result<Self, ErrorMessage>
    {
        let cursor = Cursor::new(data.to_vec());
        if let Ok(workbook) = Xlsx::new(cursor)
        {
            Ok(
                Self
                {
                    path: String::new(),
                    workbook_to_read: Some(workbook),
                    workbook_to_write: Workbook::new(),
                    bold_border_format: Format::new().set_bold().set_border(FormatBorder::Thick),
                    thin_border_format: Format::new().set_border(FormatBorder::Thin),
                }
            )
        }
        else
        {
            Err(ErrorMessage::FailedToReceiveDatabaseFromMemory)
        }
    }

    // pub fn open_empty_in_memory() -> Result<Self, ErrorMessage>
    /// Opens a new `Excel` instance to an empty Excel database in memory.
    /// 
    /// # Returns
    /// An `Result<Self, ErrorMessage>` which is:
    /// * `Ok(Excel)` on successful connection, or
    /// * `Err(ErrorMessage::FailedToOpenEmptyDatabaseInMemory)` on failure.
    /// 
    /// # Examples
    /// ```
    /// use qrate::Excel;
    /// let db = Excel::open_empty_in_memory();
    /// ```
    pub fn open_empty_in_memory() -> Self
    {
        Self
        {
            path: String::new(),
            workbook_to_read: None,
            workbook_to_write: Workbook::new(),
            bold_border_format: Format::new().set_bold().set_border(FormatBorder::Thick),
            thin_border_format: Format::new().set_border(FormatBorder::Thin),
        }
    }

    // pub fn save_to_file(&self) -> Result<(), ErrorMessage>
    /// Saves the Excel database to a file.
    ///
    /// # Returns
    /// `Ok(())` if the Excel database is saved successfully,
    /// `Err(ErrorMessage::FailedToWriteDatabase)` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let db = Excel::open_with_ext("my".to_string(), "qb.xlsx").unwrap();
    /// let result = db.save_to_file();
    /// assert!(result.is_ok());
    /// ```
    pub fn save_to_file(&mut self) -> Result<(), ErrorMessage>
    {
        if self.workbook_to_write.save(&self.path).is_ok()
            { Ok(()) }
        else
            { Err(ErrorMessage::FailedToWriteDatabase) }
    }

    // pub fn save_in_memory(&mut self) -> Result<Vec<u8>, ErrorMessage>
    /// Saves the database to a byte vector in memory.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` containing the serialized database if successful,
    /// * `Err(ErrorMessage::FailedToWriteDatabaseToMemory)` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let db = Excel::open_with_ext("my_db".to_string(), "qb.xlsx").unwrap();
    /// let result = db.save_in_memory();
    /// assert!(result.is_ok());
    /// ```
    pub fn save_in_memory(&mut self) -> Result<Vec<u8>, ErrorMessage>
    {
        if let Ok(buffer) = self.workbook_to_write.save_to_buffer()
            { Ok(buffer) }
        else
            { Err(ErrorMessage::FailedToWriteDatabaseToMemory) }
    }

    // pub fn close(self) -> Result<(), ErrorMessage>
    /// Closes the database connection.
    ///
    /// # Returns
    /// `Ok(())` always, as closing an Excel database does not have failure
    /// conditions in this implementation.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let db = Excel::open_with_ext("my_db".to_string(), "qb.xlsx").unwrap();
    /// let result = db.close();
    /// assert!(result.is_ok());
    /// ```
    #[inline]
    pub fn close(self) -> Result<(), ErrorMessage>
    {
        Ok(())
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
    /// # Returns
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
        let is_empty = |d: &calamine::Data| -> bool {
            match d {
                calamine::Data::Empty => true,
                calamine::Data::String(s) => s.trim().is_empty(),
                _ => false,
            }
        };

        if row.is_empty() || row.iter().all(is_empty) {
            return None;
        }

        let id_cell = row.get(0)?;
        if is_empty(id_cell) {
            return None;
        }
        let id = id_cell.as_f64()
            .or_else(|| id_cell.as_string().and_then(|s| s.trim().parse::<f64>().ok()))
            .map(|f| f as u16)?;

        let group = row.get(1)
            .and_then(|d| {
                if is_empty(d) {
                    None
                } else {
                    d.as_f64().or_else(|| d.as_string().and_then(|s| s.trim().parse::<f64>().ok()))
                }
            })
            .map(|f| f as u16)
            .unwrap_or(id);

        let question_text = row.get(3)
            .and_then(|d| d.as_string())
            .unwrap_or_default();

        let is_answer_1_empty = row.get(5).map_or(true, is_empty);
        if is_answer_1_empty {
            return Some(Question::new(id, group, 4, question_text, Choices::new()));
        }

        let parse_bool = |d: &calamine::Data| -> bool {
            d.get_bool().unwrap_or_else(|| {
                d.as_string().map_or(false, |s| {
                    let trimmed = s.trim();
                    trimmed.eq_ignore_ascii_case("TRUE") || trimmed == "1" || trimmed.eq_ignore_ascii_case("T")
                })
            })
        };

        let mut choices = Choices::new();
        let mut k = 0;
        loop {
            let choice_idx = 4 + k * 2;
            let is_answer_idx = 5 + k * 2;

            let is_ans_cell = match row.get(is_answer_idx) {
                Some(c) => c,
                None => break,
            };

            if is_empty(is_ans_cell) {
                break;
            }

            let choice_text = row.get(choice_idx)
                .and_then(|d| d.get_string().map(|s| s.to_string()).or_else(|| d.as_string()))
                .unwrap_or_default();

            let is_answer = parse_bool(is_ans_cell);
            choices.push((choice_text, is_answer));
            k += 1;
        }

        let cat_cell = row.get(2);
        let category = if cat_cell.map_or(true, is_empty) {
            let true_count = choices.iter().filter(|c| c.1).count();
            let total = choices.len();
            if true_count == 0 {
                4
            } else if true_count == total {
                3
            } else if true_count == 1 {
                1
            } else {
                2
            }
        } else {
            cat_cell.and_then(|d| d.as_f64().or_else(|| d.as_string().and_then(|s| s.trim().parse::<f64>().ok())))
                .map(|f| f as u8)
                .unwrap_or_else(|| {
                    let true_count = choices.iter().filter(|c| c.1).count();
                    let total = choices.len();
                    if true_count == 0 {
                        4
                    } else if true_count == total {
                        3
                    } else if true_count == 1 {
                        1
                    } else {
                        2
                    }
                })
        };

        Some(Question::new(id, group, category, question_text, choices))
    }

    // pub fn get_path(&self) -> &String
    /// Gets the path of the Excel file.
    ///
    /// # Returns
    /// `&String` - A reference to the path of the Excel file.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let db = Excel::open("my".to_string(), "qb.xlsx").unwrap();
    /// assert_eq!(db.get_path(), "my.qb.xlsx");
    /// ```
    #[inline]
    pub fn get_path(&self) -> &String
    {
        &self.path
    }

    // pub fn set_path(&mut self, path: String)
    /// Sets the path of the database file.
    ///
    /// # Arguments
    /// * `path` - The new path for the database file.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let mut db = Excel::open("my_file".to_string(), "qb.xlsx").unwrap();
    /// db.set_path("new_path.qb.xlsx".to_string());
    /// assert_eq!(db.get_path(), "new_path.qb.xlsx");
    /// ```
    #[inline]
    pub fn set_path(&mut self, path: String)
    {
        self.path = path;
    }

    // pub fn get_workbook_to_read(&self) -> &Xlsx<Cursor<Vec<u8>>>
    /// Gets a reference to the workbook object wrapped by `Option`
    /// for reading Excel files.
    ///
    /// # Returns
    /// `&Xlsx<Cursor<Vec<u8>>>` - A reference to the workbook object used for
    /// reading Excel files.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let db = Excel::open("my".to_string(), "qb.xlsx").unwrap();
    /// let workbook_ref = db.get_workbook_to_read();
    /// // You can now use `workbook_ref` to read data from the Excel file.
    /// ```
    #[inline]
    pub fn get_workbook_to_read(&self) -> Option<&Xlsx<Cursor<Vec<u8>>>>
    {
        self.workbook_to_read.as_ref()
    }

    // pub fn set_workbook_to_read(&mut self, workbook: Xlsx<Cursor<Vec<u8>>>)
    /// Sets the workbook object for reading Excel files.
    ///
    /// # Arguments
    /// * `workbook` - The new workbook object to be used for reading Excel files.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    /// use calamine::Xlsx;
    ///
    /// let mut db = Excel::open("my".to_string(), "qb.xlsx").unwrap();
    /// let new_workbook = Xlsx::new(Cursor::new(Vec::new())).unwrap();
    /// db.set_workbook_to_read(new_workbook);
    /// // The workbook for reading Excel files has been updated.
    /// ```
    #[inline]
    pub fn set_workbook_to_read(&mut self, workbook: Xlsx<Cursor<Vec<u8>>>)
    {
        self.workbook_to_read = Some(workbook);
    }

    // pub fn get_workbook_to_write(&self) -> &Workbook
    /// Gets a reference to the workbook object for writing Excel files.
    ///
    /// # Returns
    /// `&Workbook` - A reference to the workbook object used for writing Excel files.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let db = Excel::open("my".to_string(), "qb.xlsx").unwrap();
    /// let workbook_ref = db.get_workbook_to_write();
    /// // You can now use `workbook_ref` to write data to the Excel file.
    /// ```
    #[inline]
    pub fn get_workbook_to_write(&self) -> &Workbook
    {
        &self.workbook_to_write
    }

    // pub fn set_workbook_to_write(&mut self, workbook: Workbook)
    /// Sets the workbook object for writing Excel files.
    ///
    /// # Arguments
    /// * `workbook` - The new workbook object to be used for writing Excel files.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    /// use rust_xlsxwriter::Workbook;
    ///
    /// let mut db = Excel::open("my".to_string(), "qb.xlsx").unwrap();
    /// let new_workbook = Workbook::new();
    /// db.set_workbook_to_write(new_workbook);
    /// // The workbook for writing Excel files has been updated.
    /// ```
    #[inline]
    pub fn set_workbook_to_write(&mut self, workbook: Workbook)
    {
        self.workbook_to_write = workbook;
    }
}