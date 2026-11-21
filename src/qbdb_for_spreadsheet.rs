// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use calamine::{ DataType, Reader, open_workbook_auto };

use crate::{ QBDB, Header, QBank, Spreadsheet, ErrorMessage };

impl QBDB for Spreadsheet
{
    // fn open(path: String) -> Result<Self, ErrorMessage> where Self: Sized
    /// Implements `open` for `Spreadsheet`.
    /// Appends `.qb.ods` to the path if no extension is present.
    /// 
    /// # Arguments
    /// * `path` - The file path for the Spreadsheet workbook.
    ///
    /// # Returns
    /// `Result<Spreadsheet, ErrorMessage>` - A `Result` containing:
    /// * `Ok(Spreadsheet)` if the workbook is successfully opened, or
    /// * `Err(ErrorMessage::FailedToOpenQBank)` if it fails.
    fn open(path: String) -> Result<Self, ErrorMessage>
    where Self: Sized
    {
        if !path.is_empty()
            { Ok(Spreadsheet::open_with_ext(path, "qb.ods")) }
        else
            { Err(ErrorMessage::FailedToOpenQBank) }
    }

    // fn make_tables(&mut self, categories: u8, choices: u8) -> Result<(), ErrorMessage>
    /// Creates tables for `Spreadsheet`.
    ///
    /// # Arguments
    /// * `categories` - The number of category columns to create in Header sheet.
    /// * `choices` - The number of choice columns to create in Questions sheet.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success, or
    /// * `Err(ErrorMessage::FailedToCreateHeaderForQBank)` if it fails to
    ///   create the header sheet, or
    /// * `Err(ErrorMessage::FailedToMakeTableForQBank)` if `categories` is `0`
    ///   or it fails to create the question sheet.
    fn make_tables(&mut self, _categories: u8, choices: u8) -> Result<(), ErrorMessage>
    {
        // 1. Create "Header" sheet
        if let Ok(header_sheet) = self.workbook_to_write.add_worksheet().set_name("Header")
        {
            if header_sheet.write_string_with_format(0, 0, "Version", &self.bold_border_format).is_err()
            || header_sheet.write_string_with_format(1, 0, "Title", &self.bold_border_format).is_err()
            || header_sheet.write_string_with_format(2, 0, "Name", &self.bold_border_format).is_err()
            || header_sheet.write_string_with_format(3, 0, "ID", &self.bold_border_format).is_err()
            || header_sheet.write_string_with_format(4, 0, "Notice", &self.bold_border_format).is_err()
            || header_sheet.write_string_with_format(5, 0, "Categories", &self.bold_border_format).is_err()
                { return Err(ErrorMessage::FailedToCreateHeaderForQBank); }
        }
        else
        {
            return Err(ErrorMessage::FailedToCreateHeaderForQBank);
        }

        // 2. Create "Questions" sheet
        if let Ok(questions_sheet) = self.workbook_to_write.add_worksheet().set_name("Questions")
        {
            if questions_sheet.write_string_with_format(0, 0, "ID", &self.bold_border_format).is_err()
            || questions_sheet.write_string_with_format(0, 1, "Group", &self.bold_border_format).is_err()
            || questions_sheet.write_string_with_format(0, 2, "Category", &self.bold_border_format).is_err()
            || questions_sheet.write_string_with_format(0, 3, "Question", &self.bold_border_format).is_err()
                { return Err(ErrorMessage::FailedToMakeTableForQBank); }

            let mut current_col = 4;
            for i in 1..=choices
            {
                if questions_sheet.write_string_with_format(0, current_col, &format!("Choice{}", i), &self.bold_border_format).is_err()
                    { return Err(ErrorMessage::FailedToMakeTableForQBank); }
                current_col += 1;
                if questions_sheet.write_string_with_format(0, current_col, &format!("IsAnswer{}", i), &self.bold_border_format).is_err()
                    { return Err(ErrorMessage::FailedToMakeTableForQBank); }
                current_col += 1;
            }
            Ok(())
        }
        else
        {
            Err(ErrorMessage::FailedToMakeTableForQBank)
        }
    }

    // fn read_header(&mut self) -> Result<Header, ErrorMessage>
    /// Reads the `Header` data from the database.
    ///
    /// # Returns
    /// `Result<Header, ErrorMessage>`
    /// * `Ok(Header)` if the header is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForQBank)` if it fails.
    fn read_header(&mut self) -> Result<Header, ErrorMessage>
    {
        let parse_range = |range: &calamine::Range<calamine::Data>| -> Result<Header, ErrorMessage> {
            let version = range.get((0, 1)).and_then(|c| c.as_string()).unwrap_or_default();
            let title = range.get((1, 1)).and_then(|c| c.as_string()).unwrap_or_default();
            let name = range.get((2, 1)).and_then(|c| c.as_string()).unwrap_or_default();
            let id = range.get((3, 1)).and_then(|c| c.as_string()).unwrap_or_default();
            let notice = range.get((4, 1)).and_then(|c| c.as_string()).unwrap_or_default();
            let mut categories = Vec::new();
            let mut col = 1;
            while let Some(cat) = range.get((5, col)).and_then(|c| c.as_string())
            {
                if cat.is_empty()
                    { break; }
                categories.push(cat);
                col += 1;
            }
            let mut header = Header::new(title, name, id, categories, notice);
            header.set_version(version.parse().unwrap_or(0));
            Ok(header)
        };

        if let Some(workbook) = self.workbook_to_read.as_mut()
        {
            if let Ok(range) = workbook.worksheet_range("Header")
                { return parse_range(&range); }
        }
        else if let Ok(mut spreadsheet) = open_workbook_auto(&self.path)
        {
            if let Ok(range) = spreadsheet.worksheet_range("Header")
                { return parse_range(&range); }
        }
        Err(ErrorMessage::FailedToReadHeaderForQBank)
    }

    // fn write_header_with_default(&mut self) -> Result<(), ErrorMessage>
    /// Writes a default `Header` to the database.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success, or
    /// * `Err(ErrorMessage::FailedToWriteHeaderForQBank)` on failure.
    #[inline]
    fn write_header_with_default(&mut self) -> Result<(), ErrorMessage>
    {
        self.write_header(&Header::new_with_default())
    }

    // fn write_header(&mut self, header: &Header) -> Result<(), ErrorMessage>
    /// Implements `write_header` for `Spreadsheet`.
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
    fn write_header(&mut self, header: &Header) -> Result<(), ErrorMessage>
    {
        if let Ok(header_sheet) = self.workbook_to_write.worksheet_from_name("Header")
        {
            if header_sheet.write_string_with_format(0, 1, &header.get_version().to_string(), &self.thin_border_format).is_err()
            || header_sheet.write_string_with_format(1, 1, header.get_title(), &self.thin_border_format).is_err()
            || header_sheet.write_string_with_format(2, 1, header.get_name(), &self.thin_border_format).is_err()
            || header_sheet.write_string_with_format(3, 1, header.get_id(), &self.thin_border_format).is_err()
            || header_sheet.write_string_with_format(4, 1, header.get_notice(), &self.thin_border_format).is_err()
                { return Err(ErrorMessage::FailedToWriteHeaderForQBank); }
            for (i, cat) in header.get_categories().iter().enumerate()
            {
                if header_sheet.write_string_with_format(5, i as u16 + 1, cat, &self.thin_border_format).is_err()
                    { return Err(ErrorMessage::FailedToWriteHeaderForQBank); }
            }
            Ok(())
        }
        else
        {
            Err(ErrorMessage::FailedToWriteHeaderForQBank)
        }
    }

    // fn read_qbank(&mut self) -> Result<QBank, ErrorMessage>
    /// Reads the entire `QBank` (header and all questions) from the database.
    ///
    /// # Returns
    /// `Result<QBank, ErrorMessage>`
    /// * `Ok(QBank)` if the QBank is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForQBank)` if reading
    ///    the header fails, or
    /// * `Err(ErrorMessage::FailedToOpenQBank)` if it fails.
    fn read_qbank(&mut self) -> Result<QBank, ErrorMessage>
    {
        if let Ok(header) = self.read_header()
        {
            let mut qbank = QBank::new_with_header(header);
            let mut parse_questions = |range: &calamine::Range<calamine::Data>| -> Result<(), ErrorMessage> {
                let is_empty = |d: &calamine::Data| -> bool {
                    match d
                    {
                        calamine::Data::Empty => true,
                        calamine::Data::String(s) => s.trim().is_empty(),
                        _ => false,
                    }
                };
                for row in range.rows().skip(1) // Skip header row
                {
                    if row.is_empty() || row.iter().all(is_empty) || row.get(0).map_or(true, is_empty)
                        { continue; }

                    if let Some(question) = Spreadsheet::parse_question_row(row)
                        { qbank.push_question(question); }
                    else
                        { return Err(ErrorMessage::FailedToOpenQBank); }
                }
                Ok(())
            };

            if let Some(workbook) = self.workbook_to_read.as_mut()
            {
                if let Ok(range) = workbook.worksheet_range("Questions")
                {
                    parse_questions(&range)?;
                    return Ok(qbank);
                }
            }
            else if let Ok(mut spreadsheet) = open_workbook_auto(&self.path)
            {
                if let Ok(range) = spreadsheet.worksheet_range("Questions")
                {
                    parse_questions(&range)?;
                    return Ok(qbank);
                }
            }
            Err(ErrorMessage::FailedToOpenQBank)
        }
        else
        {
            Err(ErrorMessage::FailedToReadHeaderForQBank)
        }
    }

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
    fn write_qbank(&mut self, qbank: &QBank) -> Result<(), ErrorMessage>
    {
        let categories = if qbank.get_header().get_categories().is_empty()
            { 4 }
        else
            { qbank.get_header().get_categories().len() as u8 };

        // 1. Determine the maximum number of choices in the entire bank to create a uniform SQL statement.
        let max_choices = qbank.get_max_choices_of_all_questions();

        // 2. Create necessary tables in the workbook
        self.make_tables(categories, max_choices as u8)?;

        // 3. Write "Header" sheet
        self.write_header(qbank.get_header())?;

        // 4. Write "Questions" sheet
        if let Ok(questions_sheet) = self.workbook_to_write.worksheet_from_name("Questions")
        {
            for (row_idx, question) in qbank.get_questions().iter().enumerate()
            {
                let current_row = (row_idx + 1) as u32;
                if questions_sheet.write_number_with_format(current_row, 0, question.get_id() as f64, &self.thin_border_format).is_err()
                || questions_sheet.write_number_with_format(current_row, 1, question.get_group() as f64, &self.thin_border_format).is_err()
                || questions_sheet.write_number_with_format(current_row, 2, question.get_category() as f64, &self.thin_border_format).is_err()
                || questions_sheet.write_string_with_format(current_row, 3, question.get_question(), &self.thin_border_format).is_err()
                    { return Err(ErrorMessage::FailedToWriteQBank); }

                for (i, (choice_text, is_answer)) in question.get_choices().iter().enumerate()
                {
                    let choice_col = (i * 2 + 4) as u16;
                    if questions_sheet.write_string_with_format(current_row, choice_col, choice_text, &self.thin_border_format).is_err()
                    || questions_sheet.write_string_with_format(current_row, choice_col + 1, &is_answer.to_string().to_uppercase(), &self.thin_border_format).is_err()
                        { return Err(ErrorMessage::FailedToWriteQBank); }
                }
            }
            
            if !self.path.is_empty()
            {
                if self.workbook_to_write.save(&self.path).is_ok()
                    { Ok(()) }
                else
                    { Err(ErrorMessage::FailedToWriteQBank) }
            }
            else
            {
                Ok(())
            }
        }
        else
        {
            Err(ErrorMessage::FailedToWriteHeaderForQBank)
        }
    }
}