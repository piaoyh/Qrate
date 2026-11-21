// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use calamine::{ DataType, Reader };
use spreadsheet_ods::{ Sheet, write_ods };

use crate::{ SBDB, SBank, Student, Spreadsheet, ErrorMessage };

impl SBDB for Spreadsheet
{
    // fn open(path: String) -> Result<Self, ErrorMessage>
    /// Implements `open` for `Spreadsheet`.
    /// Appends `.sb.ods` to the path if no extension is present
    /// and opens a spreadsheet file.
    ///
    /// # Arguments
    /// * `path` - The file path for the spreadsheet file.
    /// * `extention` - The file extension to append.
    ///
    /// # Returns
    /// `Result<Spreadsheet, ErrorMessage>` - A `Result` containing:
    /// * `Ok(Spreadsheet)` if the workbook is successfully opened, or
    /// * `Err(ErrorMessage::FailedToOpenSBank)` if it fails.
    #[inline]
    fn open(path: String) -> Result<Self, ErrorMessage>
    where Self: Sized
    {
        if !path.is_empty()
            { Ok(Spreadsheet::open_with_ext(path, "sb.ods")) }
        else
            { Err(ErrorMessage::FailedToOpenSBank) }
    }

    // fn make_table(&mut self) -> Result<(), ErrorMessage>
    /// Creates the necessary table(s) for storing student data.
    ///
    /// For a database that already has the table,
    /// this should not produce an error.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success, or
    /// * `Err(ErrorMessage::FailedToCreateHeaderForSBank)` if it fails to
    ///   create the header table, or
    /// * `Err(ErrorMessage::FailedToMakeTableForSBank)` if it fails to
    ///   create the student list table.
    fn make_table(&mut self) -> Result<(), ErrorMessage>
    {
        let bold_ref = self.bold_border_style.clone();
        let thin_ref = self.thin_border_style.clone();

        // 1. Create "Header" sheet
        let mut header_sheet = Sheet::new("Header");
        header_sheet.set_styled_value(0, 0, "Version", &bold_ref);
        header_sheet.set_styled_value(1, 0, "1",       &thin_ref);
        self.workbook_to_write.push_sheet(header_sheet);

        // 2. Create "Students" sheet
        let mut students_sheet = Sheet::new("Students");
        students_sheet.set_styled_value(0, 0, "Name", &bold_ref);
        students_sheet.set_styled_value(0, 1, "ID",   &bold_ref);
        self.workbook_to_write.push_sheet(students_sheet);

        if !self.path.is_empty()
        {
            write_ods(&mut self.workbook_to_write, &self.path)
                .map_err(|_| ErrorMessage::FailedToMakeTableForSBank)
        }
        else
        {
            Ok(())
        }
    }

    // fn read_sbank(&mut self) -> Result<SBank, ErrorMessage>
    /// Reads all student data from the database into an `SBank`.
    ///
    /// # Returns
    /// `Result<SBank, ErrorMessage>`
    /// * `Ok(SBank)` if the SBank is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForSBank)` if reading
    ///    the header fails, or
    /// * `Err(ErrorMessage::FailedToOpenSBank)` if it fails.
    fn read_sbank(&mut self) -> Result<SBank, ErrorMessage>
    {
        let mut sbank = SBank::new();

        let parse_workbook = |header_range: &calamine::Range<calamine::Data>, students_range: &calamine::Range<calamine::Data>| -> Result<SBank, ErrorMessage> {
            for row in header_range.rows().skip(1)
            {
                if let Some(v) = row.get(0).and_then(|d| d.as_i64())
                    { sbank.set_version(v as u32); }
                else
                    { return Err(ErrorMessage::FailedToReadHeaderForSBank); }
            }

            for row in students_range.rows().skip(1) // Skip header row
            {
                if let Some(name) = row.get(0).and_then(|d| d.as_string())
                {
                    if let Some(id) = row.get(1).and_then(|d| d.as_string())
                        { sbank.push_student(Student::new(name, id)); }
                    else
                        { return Err(ErrorMessage::FailedToOpenSBank); }
                }
                else
                {
                    return Err(ErrorMessage::FailedToOpenSBank);
                }
            }
            Ok(sbank)
        };

        if let Some(workbook) = self.workbook_to_read.as_mut()
        {
            if let Ok(header_range) = workbook.worksheet_range("Header")
            {
                if let Ok(students_range) = workbook.worksheet_range("Students")
                    { return parse_workbook(&header_range, &students_range); }
                else
                    { return Err(ErrorMessage::FailedToOpenSBank); }
            }
            else
            {
                return Err(ErrorMessage::FailedToReadHeaderForSBank);
            }
        }
        else if let Ok(mut spreadsheet) = calamine::open_workbook_auto(&self.path)
        {
            if let Ok(header_range) = spreadsheet.worksheet_range("Header")
            {
                if let Ok(students_range) = spreadsheet.worksheet_range("Students")
                    { return parse_workbook(&header_range, &students_range); }
                else
                    { return Err(ErrorMessage::FailedToOpenSBank); }
            }
            else
            {
                return Err(ErrorMessage::FailedToReadHeaderForSBank);
            }
        }
        else
        {
            Err(ErrorMessage::FailedToOpenSBank)
        }
    }
    
    // fn write_sbank(&mut self, sbank: &SBank) -> Result<(), ErrorMessage>
    /// Writes the contents of an `SBank` to the database.
    ///
    /// This will insert all students from the `SBank` into the database.
    /// If the table already contains data, this may result in duplicates
    /// depending on the implementation.
    ///
    /// # Arguments
    /// * `sbank` - A reference to the `SBank`
    /// containing the students to be written.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success,
    /// * `Err(ErrorMessage::EmptySBank)` if the student bank is empty,
    /// * `Err(ErrorMessage::FailedToMakeTableForSBank)`
    ///   if it fails to create the necessary tables,
    /// * `Err(ErrorMessage::FailedToWriteHeaderForSBank)`
    ///   if writing the header fails, or
    /// * `Err(ErrorMessage::FailedToWriteSBank)`
    ///   if writing a question fails.
    fn write_sbank(&mut self, sbank: &SBank) -> Result<(), ErrorMessage>
    {
        let bold_ref = self.bold_border_style.clone();
        let thin_ref = self.thin_border_style.clone();

        // 1. Create "Header" sheet
        let mut header_sheet = Sheet::new("Header");
        header_sheet.set_styled_value(0, 0, "Version", &bold_ref);
        header_sheet.set_styled_value(1, 0, "1",       &thin_ref);
        self.workbook_to_write.push_sheet(header_sheet);

        // 2. Create "Students" sheet and write student data
        let mut students_sheet = Sheet::new("Students");
        students_sheet.set_styled_value(0, 0, "Name", &bold_ref);
        students_sheet.set_styled_value(0, 1, "ID",   &bold_ref);

        for (row_idx, student) in sbank.get_students().iter().enumerate()
        {
            let row = (row_idx + 1) as u32;
            students_sheet.set_styled_value(row, 0, student.get_name(), &thin_ref);
            students_sheet.set_styled_value(row, 1, student.get_id(),   &thin_ref);
        }

        self.workbook_to_write.push_sheet(students_sheet);

        if !self.path.is_empty()
        {
            write_ods(&mut self.workbook_to_write, &self.path)
                .map_err(|_| ErrorMessage::FailedToWriteSBank)
        }
        else
        {
            Ok(())
        }
    }
}