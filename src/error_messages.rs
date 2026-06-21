// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


/// Defines the `ErrorMessage` enum, which represents various error conditions
/// that can occur in the Qrate application. Each variant corresponds to a
/// specific error scenario, such as failing to open a database or generate an
/// exam. This enum can be used throughout the application to provide consistent
/// error handling and messaging.
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
