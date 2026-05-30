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

    /// Represents an error where the data format `QBank` is invalid or cannot be parsed.
    FailedToOpenQBank,

    /// Represents an error where the data format `SBank` is invalid or cannot be parsed.
    FailedToOpenSBank,

    /// Represents an error where the Excel file for `QBank` cannot be opened or read.
    FailedToOpenQExcel,

    /// Represents an error where the Excel file for `SBank` cannot be opened or read.
    FailedToOpenSExcel,

    /// Represents an error where the `QBank` cannot be received from memory.
    FailedToReceiveQBankFromMemory,

    /// Represents an error where the `SBank` cannot be received from memory.
    FailedToReceiveSBankFromMemory,

    /// Represents an error where the `QBank` cannot be written to memory.
    FailedToWriteQBankToMemory,

    /// Represents an error where the `SBank` cannot be written to memory.
    FailedToWriteSBankToMemory,
    
    /// Represents an error where the exam cannot be generated.
    FailedToGenerateExam,
}
