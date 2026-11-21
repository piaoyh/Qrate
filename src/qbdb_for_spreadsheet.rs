// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use calamine::{ DataType, Reader, open_workbook_auto };
use spreadsheet_ods::{ Sheet, write_ods };

use crate::{ QBDB, Header, QBank, Spreadsheet, ErrorMessage };

impl QBDB for Spreadsheet
{
    // fn open(path: String) -> Result<Self, ErrorMessage>
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
        let bold_ref = self.bold_border_style.clone();
        let thin_ref = self.thin_border_style.clone();

        // 1. Create "Header" sheet
        let mut header_sheet = Sheet::new("Header");
        header_sheet.set_styled_value(0, 0, "Version",    &bold_ref);
        header_sheet.set_styled_value(1, 0, "Title",      &bold_ref);
        header_sheet.set_styled_value(2, 0, "Name",       &bold_ref);
        header_sheet.set_styled_value(3, 0, "ID",         &bold_ref);
        header_sheet.set_styled_value(4, 0, "Notice",     &bold_ref);
        header_sheet.set_styled_value(5, 0, "Categories", &bold_ref);

        // placeholder data cells (col 1) with thin border
        header_sheet.set_styled_value(0, 1, "", &thin_ref);
        header_sheet.set_styled_value(1, 1, "", &thin_ref);
        header_sheet.set_styled_value(2, 1, "", &thin_ref);
        header_sheet.set_styled_value(3, 1, "", &thin_ref);
        header_sheet.set_styled_value(4, 1, "", &thin_ref);

        self.workbook_to_write.push_sheet(header_sheet);

        // 2. Create "Questions" sheet
        let mut questions_sheet = Sheet::new("Questions");
        questions_sheet.set_styled_value(0, 0, "ID",       &bold_ref);
        questions_sheet.set_styled_value(0, 1, "Group",    &bold_ref);
        questions_sheet.set_styled_value(0, 2, "Category", &bold_ref);
        questions_sheet.set_styled_value(0, 3, "Question", &bold_ref);

        let mut current_col: u32 = 4;
        for i in 1..=choices
        {
            questions_sheet.set_styled_value(0, current_col, format!("Choice{}", i),   &bold_ref);
            current_col += 1;
            questions_sheet.set_styled_value(0, current_col, format!("IsAnswer{}", i), &bold_ref);
            current_col += 1;
        }

        self.workbook_to_write.push_sheet(questions_sheet);
        Ok(())
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
        let thin_ref = self.thin_border_style.clone();

        // Find the "Header" sheet index
        let sheet_idx = self.workbook_to_write.sheet_idx("Header")
            .ok_or(ErrorMessage::FailedToWriteHeaderForQBank)?;

        let mut detached = self.workbook_to_write.detach_sheet(sheet_idx);
        detached.set_styled_value(0, 1, header.get_version().to_string(), &thin_ref);
        detached.set_styled_value(1, 1, header.get_title(),               &thin_ref);
        detached.set_styled_value(2, 1, header.get_name(),                &thin_ref);
        detached.set_styled_value(3, 1, header.get_id(),                  &thin_ref);
        detached.set_styled_value(4, 1, header.get_notice(),              &thin_ref);
        for (i, cat) in header.get_categories().iter().enumerate()
        {
            detached.set_styled_value(5, (i as u32) + 1, cat.as_str(), &thin_ref);
        }
        self.workbook_to_write.attach_sheet(detached);
        Ok(())
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

        // 1. Determine the maximum number of choices in the entire bank.
        let max_choices = qbank.get_max_choices_of_all_questions();

        // 2. Create necessary tables in the workbook
        self.make_tables(categories, max_choices as u8)?;

        // 3. Write "Header" sheet
        self.write_header(qbank.get_header())?;

        // 4. Write "Questions" sheet
        let thin_ref = self.thin_border_style.clone();

        let sheet_idx = self.workbook_to_write.sheet_idx("Questions")
            .ok_or(ErrorMessage::FailedToWriteHeaderForQBank)?;

        let mut detached = self.workbook_to_write.detach_sheet(sheet_idx);

        for (row_idx, question) in qbank.get_questions().iter().enumerate()
        {
            let current_row = (row_idx + 1) as u32;
            detached.set_styled_value(current_row, 0, question.get_id() as f64,       &thin_ref);
            detached.set_styled_value(current_row, 1, question.get_group() as f64,    &thin_ref);
            detached.set_styled_value(current_row, 2, question.get_category() as f64, &thin_ref);
            detached.set_styled_value(current_row, 3, question.get_question(),        &thin_ref);

            for (i, (choice_text, is_answer)) in question.get_choices().iter().enumerate()
            {
                let choice_col = (i * 2 + 4) as u32;
                detached.set_styled_value(current_row, choice_col,     choice_text.as_str(),                    &thin_ref);
                detached.set_styled_value(current_row, choice_col + 1, is_answer.to_string().to_uppercase(),    &thin_ref);
            }
        }

        self.workbook_to_write.attach_sheet(detached);

        if !self.path.is_empty()
        {
            write_ods(&mut self.workbook_to_write, &self.path)
                .map_err(|_| ErrorMessage::FailedToWriteQBank)
        }
        else
        {
            Ok(())
        }
    }
}