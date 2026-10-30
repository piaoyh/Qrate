// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::{ Header, QBank, ErrorMessage };


/// A trait defining the database operations for a Question Bank (`QBank`).
///
/// This abstracts the storage mechanism for question banks, allowing for
/// different backend implementations (e.g., SQLite, flat files).
/// Implementors of this trait must provide methods for opening a connection,
/// creating tables, and saving data to the database.
/// The trait includes methods for reading and writing both the header
/// information and the questions themselves. The design allows for flexibility
/// in how the data is stored while providing a consistent interface for
/// interacting with the question bank.
pub trait QBDB
{
    // fn open(path: String, extention: &str) -> Result<Self, ErrorMessage> where Self: Sized
    /// Opens a connection to the question bank database.
    /// If the path has no extension, proper extention is appended.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    /// * `extention` - The file extension to append.
    ///
    /// # Returns
    /// `Result<Self, ErrorMessage>`
    /// * `Ok(Self)` if the connection is successful, or
    /// * `Err(ErrorMessage::FailedToOpenQBank)` if it fails.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{ SQLiteDB, QBDB };
    ///
    /// let db = SQLiteDB::open(":memory:".to_string());
    /// assert!(db.is_ok());
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{ Excel, QBDB };
    ///
    /// let excel = Excel::open("test_quiz.qb.xlsx".to_string());
    /// assert!(excel.is_ok());
    /// ```
    fn open(path: String) -> Result<Self, ErrorMessage> where Self: Sized;

    // fn make_tables(&mut self, categories: u8, choices: u8) -> Result<(), ErrorMessage>
    /// Creates the necessary tables in the database.
    ///
    /// # Arguments
    /// * `categories` - The number of category columns to create in Header table.
    /// * `choices` - The number of choice columns to create in Questions table.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success, or
    /// * `Err(ErrorMessage::FailedToCreateHeaderForQBank)` if it fails to
    ///   create the header table, or
    /// * `Err(ErrorMessage::FailedToMakeTableForQBank)` if `categories` is `0`
    ///   or it fails to create the question table.
    /// 
    /// # Features
    /// If `choices` is zero, this method will not make Questions table.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{ SQLiteDB, QBDB };
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// let result = db.make_tables(2, 4); // 2 categories, 4 choices
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{ Excel, QBDB };
    /// use std::path::Path;
    ///
    /// let file_path = "test_make_tables.qb.xlsx";
    /// let excel = Excel::open(file_path.to_string()).unwrap();
    /// let result = excel.make_tables(2, 5); // 2 categories, 5 choices
    /// assert!(result.is_ok());
    /// assert!(Path::new(file_path).exists());
    /// std::fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn make_tables(&mut self, categories: u8, choices: u8) -> Result<(), ErrorMessage>;

    // fn read_header(&mut self) -> Result<Header, ErrorMessage>
    /// Reads the `Header` data from the database.
    ///
    /// # Returns
    /// `Result<Header, ErrorMessage>`
    /// * `Ok(Header)` if the header is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForQBank)` if it fails.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{ SQLiteDB, QBDB, Header };
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// db.write_header_with_default().unwrap();
    ///
    /// let header = db.read_header();
    /// assert!(header.is_ok());
    /// assert_eq!(header.unwrap().get_title(), "Examination");
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{ Excel, QBDB, Header };
    /// use std::fs;
    ///
    /// let file_path = "test_read_header.qb.xlsx";
    /// let mut excel = Excel::open(file_path.to_string()).unwrap();
    /// excel.write_header_with_default().unwrap();
    ///
    /// let header = excel.read_header();
    /// assert!(header.is_ok());
    /// assert_eq!(header.unwrap().get_title(), "Examination");
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn read_header(&mut self) -> Result<Header, ErrorMessage>;

    // fn write_header_with_default(&mut self) -> Result<(), ErrorMessage>
    /// Writes a default `Header` to the database.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success, or
    /// * `Err(ErrorMessage::FailedToWriteHeaderForQBank)` on failure.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{ SQLiteDB, QBDB };
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// let result = db.write_header_with_default();
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{ Excel, QBDB };
    /// use std::path::Path;
    /// use std::fs;
    ///
    /// let file_path = "test_write_header_default.qb.xlsx";
    /// let mut excel = Excel::open(file_path.to_string()).unwrap();
    /// let result = excel.write_header_with_default();
    /// assert!(result.is_ok());
    /// assert!(Path::new(file_path).exists());
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn write_header_with_default(&mut self) -> Result<(), ErrorMessage>;

    // fn write_header(&mut self, header: &Header) -> Result<(), ErrorMessage>
    /// Writes a given `Header` to the database.
    ///
    /// # Arguments
    /// * `header` - A reference to the `Header` to be written to the database.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success,
    /// * `Err(ErrorMessage::FailedToMakeTableForQBank)` if `categories` is `0`
    ///   or it fails to create the question table, or
    /// * `Err(ErrorMessage::FailedToWriteHeaderForQBank)` on failure.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{ SQLiteDB, QBDB, Header };
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(1, 4).unwrap(); // 1 category for this test
    /// let custom_header = Header::new(
    ///     "Custom Title".to_string(),
    ///     "Author".to_string(),
    ///     "ID123".to_string(),
    ///     vec!["Type C".to_string()],
    ///     "Custom Notice".to_string(),
    /// );
    /// let result = db.write_header(&custom_header);
    /// assert!(result.is_ok());
    /// let read_header = db.read_header().unwrap();
    /// assert_eq!(read_header.get_title(), "Custom Title");
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{ Excel, QBDB, Header };
    /// use std::fs;
    ///
    /// let file_path = "test_write_header.qb.xlsx";
    /// let mut excel = Excel::open(file_path.to_string()).unwrap();
    /// let custom_header = Header::new(
    ///     "Math Exam".to_string(),
    ///     "Dr. Turing".to_string(),
    ///     "CS101".to_string(),
    ///     vec!["Algebra".to_string()],
    ///     "No calculators.".to_string(),
    /// );
    /// let result = excel.write_header(&custom_header);
    /// assert!(result.is_ok());
    ///
    /// let read_header = excel.read_header().unwrap();
    /// assert_eq!(read_header.get_title(), "Math Exam");
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn write_header(&mut self, header: &Header) -> Result<(), ErrorMessage>;

    // fn read_qbank(&mut self) -> Result<QBank, ErrorMessage>
    /// Reads the entire `QBank` (header and all questions) from the database.
    ///
    /// # Returns
    /// `Result<QBank, ErrorMessage>`
    /// * `Ok(QBank)` if the QBank is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForQBank)` if reading
    ///    the header fails, or
    /// * `Err(ErrorMessage::FailedToOpenQBank)` if it fails.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{ SQLiteDB, QBDB, QBank, Question, Choices };
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// db.write_header_with_default().unwrap();
    ///
    /// let mut qbank_to_write = QBank::new_with_default();
    /// let choices = vec![("Ch1".to_string(), true), ("Ch2".to_string(), false)];
    /// let question = Question::new(1, 1, 1, "Test Question".to_string(), choices);
    /// qbank_to_write.push_question(question);
    /// db.write_qbank(&qbank_to_write).unwrap();
    ///
    /// let qbank_read = db.read_qbank();
    /// assert!(qbank_read.is_ok());
    /// let read_bank = qbank_read.unwrap();
    /// assert_eq!(read_bank.get_questions().len(), 1);
    /// assert_eq!(read_bank.get_header().get_title(), "Examination");
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{ Excel, QBDB, QBank, Question, Choices };
    /// use std::fs;
    ///
    /// let file_path = "test_read_qbank.qb.xlsx";
    /// let mut excel = Excel::open(file_path.to_string()).unwrap();
    ///
    /// let mut qbank_to_write = QBank::new_with_default();
    /// let choices = vec![("Paris".to_string(), true), ("Berlin".to_string(), false)];
    /// qbank_to_write.push_question(Question::new(1, 1, 1, "Capital of France?".to_string(), choices));
    /// excel.write_qbank(&qbank_to_write).unwrap();
    ///
    /// let qbank_read = excel.read_qbank();
    /// assert!(qbank_read.is_ok());
    /// let read_bank = qbank_read.unwrap();
    /// assert_eq!(read_bank.get_questions().len(), 1);
    /// assert_eq!(read_bank.get_header().get_title(), "Examination");
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn read_qbank(&mut self) -> Result<QBank, ErrorMessage>;

    // fn write_qbank(&mut self, qbank: &QBank) -> Result<(), ErrorMessage>
    /// Writes an entire `QBank` (header and all questions) to the database.
    ///
    /// Note: This typically writes the questions. The header should be written
    /// separately if it's not already present.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` to be written to the database.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success,
    /// * `Err(ErrorMessage::EmptyQBank)` if the question bank is empty,
    /// * `Err(ErrorMessage::FailedToMakeTableForQBank)`
    ///   if it fails to create the necessary tables,
    /// * `Err(ErrorMessage::FailedToWriteHeaderForQBank)`
    ///   if writing the header fails, or
    /// * `Err(ErrorMessage::FailedToWriteQBank)`
    ///   if writing a question fails.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{ SQLiteDB, QBDB, QBank, Question, Choices };
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// db.write_header_with_default().unwrap();
    ///
    /// let mut qbank = QBank::new_with_default();
    /// let choices = vec![("Ans1".to_string(), true), ("Ans2".to_string(), false)];
    /// let question = Question::new(1, 1, 1, "Test Q".to_string(), choices);
    /// qbank.push_question(question);
    ///
    /// let result = db.write_qbank(&qbank);
    /// assert!(result.is_ok());
    ///
    /// // Verify by reading back
    /// let read_qbank = db.read_qbank().unwrap();
    /// assert_eq!(read_qbank.get_questions().len(), 1);
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{ Excel, QBDB, QBank, Question, Choices };
    /// use std::path::Path;
    /// use std::fs;
    ///
    /// let file_path = "test_write_qbank.qb.xlsx";
    /// let mut excel = Excel::open(file_path.to_string()).unwrap();
    ///
    /// let mut qbank = QBank::new_with_default();
    /// let choices = vec![("Opt1".to_string(), false), ("Opt2".to_string(), true)];
    /// qbank.push_question(Question::new(1, 1, 1, "A Question".to_string(), choices));
    ///
    /// let result = excel.write_qbank(&qbank);
    /// assert!(result.is_ok());
    /// assert!(Path::new(file_path).exists());
    ///
    /// // Optional: verify by reading back
    /// let read_qbank = excel.read_qbank().unwrap();
    /// assert_eq!(read_qbank.get_questions().len(), 1);
    ///
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn write_qbank(&mut self, qbank: &QBank) -> Result<(), ErrorMessage>;
}