// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use calamine::{ DataType, Reader, open_workbook_auto };

use crate::{ QBDB, Header, QBank, Excel, ErrorMessage };

impl QBDB for Excel
{
    // fn open(path: String) -> Result<Self, ErrorMessage> where Self: Sized
    /// Implements `open` for `Excel`.
    /// Appends `.qb.xlsx` to the path if no extension is present.
    /// 
    /// # Arguments
    /// * `path` - The file path for the Excel workbook.
    ///
    /// # Returns
    /// `Result<Excel, ErrorMessage>` - A `Result` containing:
    /// * `Ok(Excel)` if the workbook is successfully opened, or
    /// * `Err(ErrorMessage::FailedToOpenQBank)` if it fails.
    fn open(path: String) -> Result<Self, ErrorMessage>
    where Self: Sized
    {
        if !path.is_empty()
            { Ok(Excel::open_with_ext(path, "qb.xlsx")) }
        else
            { Err(ErrorMessage::FailedToOpenQBank) }
    }

    // fn make_tables(&mut self, categories: u8, choices: u8) -> Result<(), ErrorMessage>
    /// Creates tables for `Excel`.
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
        else if let Ok(mut excel) = open_workbook_auto(&self.path)
        {
            if let Ok(range) = excel.worksheet_range("Header")
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
    /// Implements `write_header` for `Excel`.
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

                    if let Some(question) = Excel::parse_question_row(row)
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
            else if let Ok(mut excel) = open_workbook_auto(&self.path)
            {
                if let Ok(range) = excel.worksheet_range("Questions")
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


// AI wrote tests for SQLite and Excel read/write functionality
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_excel_write_qbank_in_memory()
    {
        let mut excel = Excel::open_empty_in_memory();
        let mut qbank = QBank::new_with_default();
        let choices = Choices::from(vec![
            ("Choice 1".to_string(), true),
            ("Choice 2".to_string(), false),
        ]);
        qbank.push_question(Question::new(1, 1, 1, "Sample question".to_string(), choices));

        let write_res = excel.write_qbank(&qbank);
        assert!(write_res.is_ok(), "write_qbank in memory should succeed without calling save to file");

        let buffer_res = excel.save_in_memory();
        assert!(buffer_res.is_ok(), "save_in_memory should successfully produce bytes");
        let bytes = buffer_res.unwrap();
        assert!(!bytes.is_empty(), "Produced buffer should not be empty");

        // Verify the generated Excel can be read back
        let mut read_excel = Excel::open_in_memory(&bytes).expect("Failed to open excel from memory");
        let read_qbank = read_excel.read_qbank().expect("Failed to read qbank from generated excel");
        assert_eq!(read_qbank.get_questions().len(), 1);
        assert_eq!(read_qbank.get_questions()[0].get_question(), "Sample question");
        assert_eq!(read_qbank.get_header().get_title(), qbank.get_header().get_title());
    }

    #[test]
    fn test_excel_write_qbank_category4_choices()
    {
        let mut excel = Excel::open_empty_in_memory();
        let mut qbank = QBank::new_with_default();
        let choices = Choices::from(vec![
            ("Essay Choice 1".to_string(), false),
            ("Essay Choice 2".to_string(), false),
        ]);
        qbank.push_question(Question::new(1, 1, 4, "Essay Question".to_string(), choices));

        let write_res = excel.write_qbank(&qbank);
        assert!(write_res.is_ok());

        let bytes = excel.save_in_memory().unwrap();
        let mut read_excel = Excel::open_in_memory(&bytes).unwrap();
        let read_qbank = read_excel.read_qbank().unwrap();
        assert_eq!(read_qbank.get_questions().len(), 1);
        let q = &read_qbank.get_questions()[0];
        assert_eq!(q.get_category(), 4);
        assert_eq!(q.get_choices().len(), 2);
        assert_eq!(q.get_choices()[0].0, "Essay Choice 1");
    }

    #[test]
    fn test_parse_question_row_rules()
    {
        use calamine::Data;

        // 1. Empty row -> None
        let empty_row = vec![Data::Empty, Data::Empty];
        assert!(Excel::parse_question_row(&empty_row).is_none());

        // 2. ID empty -> None
        let id_empty_row = vec![Data::Empty, Data::Float(1.0), Data::Float(1.0), Data::String("Q".to_string())];
        assert!(Excel::parse_question_row(&id_empty_row).is_none());

        // 3. Group empty -> ID value used
        let group_empty_row = vec![
            Data::Float(5.0),
            Data::Empty,
            Data::Float(1.0),
            Data::String("Question 5".to_string()),
            Data::String("Choice 1".to_string()),
            Data::Bool(true),
        ];
        let q = Excel::parse_question_row(&group_empty_row).unwrap();
        assert_eq!(q.get_id(), 5);
        assert_eq!(q.get_group(), 5);

        // 4. Question text empty -> ""
        let qtext_empty_row = vec![
            Data::Float(6.0),
            Data::Float(6.0),
            Data::Float(1.0),
            Data::Empty,
            Data::String("Choice 1".to_string()),
            Data::Bool(true),
        ];
        let q = Excel::parse_question_row(&qtext_empty_row).unwrap();
        assert_eq!(q.get_question(), "");

        // 5. IsAnswer1 empty -> Category = 4, choices empty
        let is_ans1_empty_row = vec![
            Data::Float(7.0),
            Data::Float(7.0),
            Data::Float(1.0), // Even if Category is 1 in cell, IsAnswer1 empty makes it 4
            Data::String("Essay Q".to_string()),
            Data::String("Some Choice".to_string()),
            Data::Empty, // IsAnswer1 is empty
            Data::String("Choice 2".to_string()),
            Data::Bool(true),
        ];
        let q = Excel::parse_question_row(&is_ans1_empty_row).unwrap();
        assert_eq!(q.get_category(), 4);
        assert_eq!(q.get_choices().len(), 0);

        // 6. IsAnswer1 not empty, Choice1 empty -> Choice1 is ""
        let choice1_empty_row = vec![
            Data::Float(8.0),
            Data::Float(8.0),
            Data::Float(1.0),
            Data::String("Q".to_string()),
            Data::Empty, // Choice1 empty
            Data::Bool(true),
        ];
        let q = Excel::parse_question_row(&choice1_empty_row).unwrap();
        assert_eq!(q.get_choices().len(), 1);
        assert_eq!(q.get_choices()[0].0, "");
        assert_eq!(q.get_choices()[0].1, true);

        // 7. IsAnswer2 empty -> Choice2 and subsequent ignored
        let is_ans2_empty_row = vec![
            Data::Float(9.0),
            Data::Float(9.0),
            Data::Float(1.0),
            Data::String("Q".to_string()),
            Data::String("Choice 1".to_string()),
            Data::Bool(true),
            Data::String("Choice 2".to_string()),
            Data::Empty, // IsAnswer2 empty
            Data::String("Choice 3".to_string()),
            Data::Bool(true),
        ];
        let q = Excel::parse_question_row(&is_ans2_empty_row).unwrap();
        assert_eq!(q.get_choices().len(), 1);
        assert_eq!(q.get_choices()[0].0, "Choice 1");

        // 8. Category empty inference:
        // 8a. All FALSE -> Category 4
        let cat_empty_all_false = vec![
            Data::Float(10.0),
            Data::Float(10.0),
            Data::Empty, // Category empty
            Data::String("Q".to_string()),
            Data::String("Choice 1".to_string()),
            Data::Bool(false),
            Data::String("Choice 2".to_string()),
            Data::Bool(false),
        ];
        let q = Excel::parse_question_row(&cat_empty_all_false).unwrap();
        assert_eq!(q.get_category(), 4);

        // 8b. All TRUE -> Category 3
        let cat_empty_all_true = vec![
            Data::Float(11.0),
            Data::Float(11.0),
            Data::Empty,
            Data::String("Q".to_string()),
            Data::String("Choice 1".to_string()),
            Data::Bool(true),
            Data::String("Choice 2".to_string()),
            Data::Bool(true),
        ];
        let q = Excel::parse_question_row(&cat_empty_all_true).unwrap();
        assert_eq!(q.get_category(), 3);

        // 8c. Exactly one TRUE -> Category 1
        let cat_empty_one_true = vec![
            Data::Float(12.0),
            Data::Float(12.0),
            Data::Empty,
            Data::String("Q".to_string()),
            Data::String("Choice 1".to_string()),
            Data::Bool(false),
            Data::String("Choice 2".to_string()),
            Data::Bool(true),
            Data::String("Choice 3".to_string()),
            Data::Bool(false),
        ];
        let q = Excel::parse_question_row(&cat_empty_one_true).unwrap();
        assert_eq!(q.get_category(), 1);

        // 8d. Otherwise -> Category 2
        let cat_empty_multi_true = vec![
            Data::Float(13.0),
            Data::Float(13.0),
            Data::Empty,
            Data::String("Q".to_string()),
            Data::String("Choice 1".to_string()),
            Data::Bool(true),
            Data::String("Choice 2".to_string()),
            Data::Bool(true),
            Data::String("Choice 3".to_string()),
            Data::Bool(false),
        ];
        let q = Excel::parse_question_row(&cat_empty_multi_true).unwrap();
        assert_eq!(q.get_category(), 2);
    }

    #[test]
    fn test_sqlite_write_and_read_qbank_null_padding()
    {
        let mut db = SQLiteDB::open(":memory:".to_string()).unwrap();
        let mut qbank = QBank::new_with_default();

        // Question 1: 4 choices (category 1)
        let choices1 = Choices::from(vec![
            ("Q1C1".to_string(), true),
            ("Q1C2".to_string(), false),
            ("Q1C3".to_string(), false),
            ("Q1C4".to_string(), false),
        ]);
        qbank.push_question(Question::new(1, 1, 1, "Question 1".to_string(), choices1));

        // Question 2: 2 choices (category 1) -> padded with null for choice 3 and 4
        let choices2 = Choices::from(vec![
            ("Q2C1".to_string(), false),
            ("Q2C2".to_string(), true),
        ]);
        qbank.push_question(Question::new(2, 2, 1, "Question 2".to_string(), choices2));

        // Question 3: 0 choices (category 4) -> padded with null for choice 1..4
        let choices3 = Choices::new();
        qbank.push_question(Question::new(3, 3, 4, "Essay Question".to_string(), choices3));

        // Question 4: 1 choice (category 3) -> padded with null for choice 2..4
        let choices4 = Choices::from(vec![
            ("Q4C1".to_string(), true),
        ]);
        qbank.push_question(Question::new(4, 4, 3, "Short Answer Question".to_string(), choices4));

        assert_eq!(qbank.get_max_choices_of_all_questions(), 4);

        let write_res = db.write_qbank(&qbank);
        assert!(write_res.is_ok(), "write_qbank should succeed");

        // Verify direct database contents have NULLs
        {
            let conn = db.get_connection();
            let mut stmt = conn.prepare("SELECT choice1_is_answer, choice2_is_answer, choice3_is_answer, choice4_is_answer FROM tblQuestions ORDER BY id").unwrap();
            let rows: Vec<(Option<bool>, Option<bool>, Option<bool>, Option<bool>)> = stmt.query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            }).unwrap().map(|r| r.unwrap()).collect();

            assert_eq!(rows.len(), 4);
            // Q1: all 4 have answers
            assert_eq!(rows[0], (Some(true), Some(false), Some(false), Some(false)));
            // Q2: choice 3 & 4 are NULL
            assert_eq!(rows[1], (Some(false), Some(true), None, None));
            // Q3: all 4 are NULL
            assert_eq!(rows[2], (None, None, None, None));
            // Q4: choice 2..4 are NULL
            assert_eq!(rows[3], (Some(true), None, None, None));
        }

        // Read back with read_qbank and verify
        let read_qbank = db.read_qbank().expect("read_qbank should succeed");
        assert_eq!(read_qbank.get_questions().len(), 4);

        let q1 = &read_qbank.get_questions()[0];
        assert_eq!(q1.get_id(), 1);
        assert_eq!(q1.get_category(), 1);
        assert_eq!(q1.get_choices().len(), 4);

        let q2 = &read_qbank.get_questions()[1];
        assert_eq!(q2.get_id(), 2);
        assert_eq!(q2.get_category(), 1);
        assert_eq!(q2.get_choices().len(), 2);
        assert_eq!(q2.get_choices()[0], ("Q2C1".to_string(), false));
        assert_eq!(q2.get_choices()[1], ("Q2C2".to_string(), true));

        let q3 = &read_qbank.get_questions()[2];
        assert_eq!(q3.get_id(), 3);
        assert_eq!(q3.get_category(), 4);
        assert_eq!(q3.get_choices().len(), 0);

        let q4 = &read_qbank.get_questions()[3];
        assert_eq!(q4.get_id(), 4);
        assert_eq!(q4.get_category(), 3);
        assert_eq!(q4.get_choices().len(), 1);
        assert_eq!(q4.get_choices()[0], ("Q4C1".to_string(), true));
    }

    #[test]
    fn test_sqlite_read_qbank_null_rules()
    {
        let mut db = SQLiteDB::open(":memory:".to_string()).unwrap();
        db.make_tables(4, 4).unwrap();
        db.write_header_with_default().unwrap();

        let conn = db.get_connection();
        // Insert row 1: choice1_is_answer is NULL, but category in cell is 1 -> category must be 4, choices empty
        conn.execute(
            "INSERT INTO tblQuestions (id, modum, category, question, choice1_text, choice1_is_answer, choice2_text, choice2_is_answer, choice3_text, choice3_is_answer, choice4_text, choice4_is_answer)
             VALUES (1, 1, 1, 'Q1', 'Some choice', NULL, 'Choice 2', 1, NULL, NULL, NULL, NULL)",
            [],
        ).unwrap();

        // Insert row 2: choice1_is_answer not NULL, choice2_is_answer is NULL -> only choice 1 recognized
        conn.execute(
            "INSERT INTO tblQuestions (id, modum, category, question, choice1_text, choice1_is_answer, choice2_text, choice2_is_answer, choice3_text, choice3_is_answer, choice4_text, choice4_is_answer)
             VALUES (2, 2, 3, 'Q2', 'C1', 1, 'C2', NULL, 'C3', 1, NULL, NULL)",
            [],
        ).unwrap();

        // Insert row 3: choice1, 2 not NULL, choice3 is NULL -> only choice 1 & 2 recognized
        conn.execute(
            "INSERT INTO tblQuestions (id, modum, category, question, choice1_text, choice1_is_answer, choice2_text, choice2_is_answer, choice3_text, choice3_is_answer, choice4_text, choice4_is_answer)
             VALUES (3, 3, 1, 'Q3', 'C1', 0, 'C2', 1, 'C3', NULL, 'C4', 1)",
            [],
        ).unwrap();

        let read_qbank = db.read_qbank().unwrap();
        assert_eq!(read_qbank.get_questions().len(), 3);

        let q1 = &read_qbank.get_questions()[0];
        assert_eq!(q1.get_category(), 4);
        assert_eq!(q1.get_choices().len(), 0);

        let q2 = &read_qbank.get_questions()[1];
        assert_eq!(q2.get_choices().len(), 1);
        assert_eq!(q2.get_choices()[0], ("C1".to_string(), true));

        let q3 = &read_qbank.get_questions()[2];
        assert_eq!(q3.get_choices().len(), 2);
        assert_eq!(q3.get_choices()[0], ("C1".to_string(), false));
        assert_eq!(q3.get_choices()[1], ("C2".to_string(), true));
    }
}