// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


//! # Qrate
//!
//! `Qrate` is a library for managing question banks and student data.
//! It provides functionalities to load, manipulate, and store questions and student information,
//! primarily from and to SQLite database files and Excel files for storage.

/// The `database` module provides the `SQLiteDB` for database interactions.
pub mod database;

/// The `excel` module provides functionalities for Excel file operations.
pub mod excel;

/// The `spreadsheet` module provides functionalities for Spreadsheet file operations.
pub mod spreadsheet;

/// The `header` module contains the `Header` structure for parsing file headers.
pub mod header;

/// The `qbank` module contains the `QBank` structure and its associated methods.
pub mod qbank;

/// The `qbdb` module handles the question bank database operations.
pub mod qbdb;

/// The `qbdb_for_sqlite` module handles the question bank database operations for SQLite files.
pub mod qbdb_for_sqlite;

/// The `qbdb_for_excel` module handles the question bank database operations for Excel files.
pub mod qbdb_for_excel;

/// The `qbdb_for_spreadsheet` module handles the question bank database operations for Spreadsheet files.
pub mod qbdb_for_spreadsheet;

/// The `question` module defines the `Question` structure.
pub mod question;

/// The `sbank` module contains the `SBank` structure and related functionalities.
pub mod sbank;

/// The `sbdb` module handles the student bank database operations.
pub mod sbdb;

/// The `sbdb_for_sqlite` module handles the student bank database operations for SQLite files.
pub mod sbdb_for_sqlite;

/// The `sbdb_for_excel` module handles the student bank database operations for Excel files.
pub mod sbdb_for_excel;

/// The `sbdb_for_spreadsheet` module handles the student bank database operations for Spreadsheet files.
pub mod sbdb_for_spreadsheet;

/// The `student` module defines the `Student` structure.
pub mod student;

/// The `shuffled_qset` module provides the `ShuffledQSet` structure and related functionalities.
pub mod shuffled_qset;

/// The `shuffled_question` module provides the `ShuffledQuestion` structure and related functionalities.
pub mod shuffled_question;

/// The `shuffler` module contains the `Shuffler` structure for shuffling questions and students.
pub mod shuffler;

/// The `generator` module provides functionalities for generating various exam formats.
pub mod generator;

/// The `exporter` module provides functionalities for exporting question bank.
pub mod qbank_exporter;

/// The `exporter` module provides functionalities for exporting student bank.
pub mod sbank_exporter;

/// The `utils` module contains utility functions used across the library.
pub mod utils;

/// The `error_messages` module defines the `ErrorMessage` enum for standardized error handling.
pub mod error_messages;

/// The `self_study` module provides functionalities for self-study sessions and scoring.
pub mod self_study;


/// Re-exporting key structures and functionalities for external use.
pub use database::SQLiteDB;

/// Re-exporting the `Excel` structure for external use.
pub use excel::Excel;

/// Re-exporting the `Spreadsheet` structure for external use.
pub use spreadsheet::Spreadsheet;

/// Re-exporting the `Header` structure for external use.
pub use header::Header;

/// Re-exporting the `QBank` structure for external use.
pub use qbank::QBank;

/// Re-exporting the `QBDB` structure for external use.
pub use qbdb::QBDB;

/// Re-exporting the `QBDBForSQLite` structure for external use.
pub use question::{ ChoiceAnswer, Choices, Question, Questions };

/// Re-exporting the `SBank` structure for external use.
pub use sbank::SBank;

/// Re-exporting the `SBDB` structure for external use.
pub use sbdb::SBDB;

/// Re-exporting the `Student` structure for external use.
pub use student::{ Student, Students };

/// Re-exporting the `ShuffledQuestion` and `ShuffledQuestions` structures for external use.
pub use shuffled_question::{ ShuffledQuestion, ShuffledQuestions };

/// Re-exporting the `ShuffledQSet` and `ShuffledQSets` structures for external use.
pub use shuffled_qset::{ ShuffledQSet, ShuffledQSets};

/// Re-exporting the `Shuffler` structure for external use.
pub use shuffler::Shuffler;

/// Re-exporting the `Generator` structure for external use.
pub use generator::Generator;

/// Re-exporting the `QBankExporter` structure for external use.
pub use qbank_exporter::QBankExporter;

/// Re-exporting the `SBankExporter` structure for external use.
pub use sbank_exporter::SBankExporter;

/// Re-exporting the `SelfStudy`, `ScoringRule`, and `UserAnswer` structures for external use.
pub use self_study::{ SelfStudy, ScoringRule, UserAnswer };

/// Re-exporting the `ErrorMessage` enum for external use.
pub use error_messages::ErrorMessage;

/// Re-exporting the `check_path` utility function for internal use.
pub(crate) use utils::check_path;
