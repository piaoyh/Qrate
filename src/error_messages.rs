// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::fmt::Display;


/// Defines the `ErrorMessage` enum, which represents various error conditions
/// that can occur in the Qrate application. Each variant corresponds to a
/// specific error scenario, such as failing to open a database or generate an
/// exam. This enum can be used throughout the application to provide consistent
/// error handling and messaging.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorMessage
{
    /// Represents an error where the version of the data is invalid or incompatible.
    InvalidVersion,

    /// Represents an error where the question bank is empty.
    EmptyQBank,

    /// Represents an error where the student bank is empty.
    EmptySBank,

    /// Represents an error where the database cannot be opened in memory.
    FailedToOpenEmptyDatabaseInMemory,

    /// Represents an error where the database cannot be opened.
    FailedToOpenDatabase,

    /// Represents an error where the database cannot be written.
    FailedToWriteDatabase,

    /// Represents an error where the table for the database cannot be written.
    FailedToMakeTableForDatabase,

    /// Represents an error where the database cannot be opened in memory.
    FailedToOpenDatabaseInMemory,

    /// Represents an error where the database cannot be received from memory.
    FailedToReceiveDatabaseFromMemory,

    /// Represents an error where the database cannot be written to memory.
    FailedToWriteDatabaseToMemory,

    /// Represents an error where the database cannot be closed.
    FailedToCloseDatabase,

    /// Represents an error where the database cannot be vacuumed.
    FailedToVacuumDatabase,

    /// Represents an error where the database cannot be opened.
    FailedToOpenEmptyQBankInMemory,

    /// Represents an error where the data format `QBank` is invalid or cannot be parsed.
    FailedToOpenQBank,

    /// Represents an error where the header for `QBank` cannot be read.
    FailedToReadHeaderForQBank,

    /// Represents an error where the `QBank` cannot be written to the database.
    FailedToWriteQBank,

    /// Represents an error where the header for `QBank` cannot be written.
    FailedToWriteHeaderForQBank,

    /// Represents an error where the table for `QBank` cannot be written to the database.
    FailedToMakeTableForQBank,

    /// Represents an error where the header for `QBank` cannot be created.
    FailedToCreateHeaderForQBank,

    /// Represents an error where the data format `SBank` is invalid or cannot be parsed.
    FailedToOpenSBank,

    /// Represents an error where the header for `SBank` cannot be read.
    FailedToReadHeaderForSBank,

    /// Represents an error where the `SBank` cannot be written to the database.
    FailedToWriteSBank,

    /// Represents an error where the table for `SBank` cannot be written to the database.
    FailedToMakeTableForSBank,

    /// Represents an error where the header for `SBank` cannot be created.
    FailedToCreateHeaderForSBank,

    /// Represents an error where the Excel file for `QBank` cannot be opened or read.
    FailedToOpenQExcel,

    /// Represents an error where the `QBank` cannot be written to the Excel file.
    FailedToWriteQExcel,

    /// Represents an error where the Excel file for `SBank` cannot be opened or read.
    FailedToOpenSExcel,

    /// Represents an error where the `SBank` cannot be written to the Excel file.
    FailedToWriteSExcel,

    /// Represents an error where the `QBank` cannot be received from memory.
    FailedToReceiveQBankFromMemory,

    /// Represents an error where the `QBank` cannot be written to memory.
    FailedToWriteQBankToMemory,

    /// Represents an error where the `SBank` cannot be received from memory.
    FailedToReceiveSBankFromMemory,

    /// Represents an error where the `SBank` cannot be written to memory.
    FailedToWriteSBankToMemory,
    
    /// Represents an error where the exam cannot be generated.
    FailedToGenerateExam,
}



impl Display for ErrorMessage
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            ErrorMessage::InvalidVersion => write!(f, "Invalid version of the data."),
            ErrorMessage::EmptyQBank => write!(f, "The question bank is empty."),
            ErrorMessage::EmptySBank => write!(f, "The student bank is empty."),
            ErrorMessage::FailedToOpenEmptyDatabaseInMemory => write!(f, "Failed to open an empty database in memory."),
            ErrorMessage::FailedToOpenDatabase => write!(f, "Failed to open the database."),
            ErrorMessage::FailedToWriteDatabase => write!(f, "Failed to write to the database."),
            ErrorMessage::FailedToMakeTableForDatabase => write!(f, "Failed to create a table for the database."),
            ErrorMessage::FailedToOpenDatabaseInMemory => write!(f, "Failed to open the database in memory."),
            ErrorMessage::FailedToReceiveDatabaseFromMemory => write!(f, "Failed to receive the database from memory."),
            ErrorMessage::FailedToWriteDatabaseToMemory => write!(f, "Failed to write the database to memory."),
            ErrorMessage::FailedToCloseDatabase => write!(f, "Failed to close the database."),
            ErrorMessage::FailedToVacuumDatabase => write!(f, "Failed to vacuum the database."),
            ErrorMessage::FailedToOpenEmptyQBankInMemory => write!(f, "Failed to open an empty QBank in memory."),
            ErrorMessage::FailedToOpenQBank => write!(f, "Failed to open the QBank."),
            ErrorMessage::FailedToReadHeaderForQBank => write!(f, "Failed to read the header for the QBank."),
            ErrorMessage::FailedToWriteQBank => write!(f, "Failed to write the QBank."),
            ErrorMessage::FailedToWriteHeaderForQBank => write!(f, "Failed to write the header for the QBank."),
            ErrorMessage::FailedToMakeTableForQBank => write!(f, "Failed to create a table for the QBank."),
            ErrorMessage::FailedToCreateHeaderForQBank => write!(f, "Failed to create the header for the QBank."),
            ErrorMessage::FailedToOpenSBank => write!(f, "Failed to open the SBank."),
            ErrorMessage::FailedToReadHeaderForSBank => write!(f, "Failed to read the header for the SBank."),
            ErrorMessage::FailedToWriteSBank => write!(f, "Failed to write the SBank."),
            ErrorMessage::FailedToMakeTableForSBank => write!(f, "Failed to create a table for the SBank."),
            ErrorMessage::FailedToCreateHeaderForSBank => write!(f, "Failed to create the header for the SBank."),
            ErrorMessage::FailedToOpenQExcel => write!(f, "Failed to open the Excel file for the QBank."),
            ErrorMessage::FailedToWriteQExcel => write!(f, "Failed to write the QBank to the Excel file."),
            ErrorMessage::FailedToOpenSExcel => write!(f, "Failed to open the Excel file for the SBank."),
            ErrorMessage::FailedToWriteSExcel => write!(f, "Failed to write the SBank to the Excel file."),
            ErrorMessage::FailedToReceiveQBankFromMemory => write!(f, "Failed to receive the QBank from memory."),
            ErrorMessage::FailedToWriteQBankToMemory => write!(f, "Failed to write the QBank to memory."),
            ErrorMessage::FailedToReceiveSBankFromMemory => write!(f, "Failed to receive the SBank from memory."),
            ErrorMessage::FailedToWriteSBankToMemory => write!(f, "Failed to write the SBank to memory."),
            ErrorMessage::FailedToGenerateExam => write!(f, "Failed to generate the exam."),
        }
    }
}