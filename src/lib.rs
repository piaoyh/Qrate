// Copyright 2026 PARK Youngho.
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

/// The `header` module contains the `Header` structure for parsing file headers.
pub mod header;

/// The `qbank` module contains the `QBank` structure and its associated methods.
pub mod qbank;

/// The `qbdb` module handles the question bank database operations.
pub mod qbdb;

/// The `question` module defines the `Question` structure.
pub mod question;

/// The `sbank` module contains the `SBank` structure and related functionalities.
pub mod sbank;

/// The `sbdb` module handles the student bank database operations.
pub mod sbdb;

/// The `student` module defines the `Student` structure.
pub mod student;

/// The `shuffler` module provides functionalities for shuffling questions and creating shuffled sets.
pub mod shuffler;

/// The `generator` module provides functionalities for generating various exam formats.
pub mod generator;

/// The `utils` module contains utility functions used across the library.
pub mod utils;

pub use database::SQLiteDB;
pub use excel::Excel;
pub use header::Header;
pub use qbank::QBank;
pub use qbdb::QBDB;
pub use question::{ ChoiceAnswer, Choices, Question, Questions };
pub use sbank::{ SBank, SBankHelper };
pub use sbdb::SBDB;
pub use student::{ Student, Students };
pub use shuffler::{ ShuffledQuestion, ShuffledQuestions, ShuffledQSet, ShuffledQSets};
pub use generator::Generator;

pub(crate) use utils::check_path;
