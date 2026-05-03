// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use calamine::{ DataType, Reader, open_workbook_auto };
use rust_xlsxwriter::{ Format, FormatBorder, Workbook };

use crate::Header;
use crate::QBank;
use crate::SQLiteDB;
use crate::Excel;
use crate::{ Choices, Question };

/// A trait defining the database operations for a Question Bank (`QBank`).
///
/// This abstracts the storage mechanism for question banks, allowing for different
/// backend implementations (e.g., SQLite, flat files).
pub trait QBDB
{
    // fn open(path: String, extention: &str) -> Option<Self> where Self: Sized;
    /// Opens a connection to the question bank database.
    /// If the path has no extension, proper extention is appended.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    /// * `extention` - The file extension to append.
    ///
    /// # Output
    /// `Option<Self>` - An optional `Self` instance if the connection is successful.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{ SQLiteDB, QBDB };
    ///
    /// let db = SQLiteDB::open(":memory:".to_string());
    /// assert!(db.is_some());
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{ Excel, QBDB };
    ///
    /// let excel = Excel::open("test_quiz.qb.xlsx".to_string());
    /// assert!(excel.is_some());
    /// ```
    fn open(path: String) -> Option<Self> where Self: Sized;

    // fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>
    /// Creates the necessary tables in the database.
    ///
    /// # Arguments
    /// * `categories` - The number of category columns to create in Header table.
    /// * `choices` - The number of choice columns to create in Questions table.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error string on failure.
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
    fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>;

    // fn read_header(&self) -> Option<Header>
    /// Reads the `Header` data from the database.
    ///
    /// # Output
    /// `Option<Header>` - An `Option<Header>` which is `Some(Header)` on success, or `None` if not found or on error.
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
    /// assert!(header.is_some());
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
    /// assert!(header.is_some());
    /// assert_eq!(header.unwrap().get_title(), "Examination");
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn read_header(&self) -> Option<Header>;

    // fn write_header_with_default(&self) -> Result<(), String>
    /// Writes a default `Header` to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
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
    fn write_header_with_default(&mut self) -> Result<(), String>;

    // fn write_header(&self, header: &Header) -> Result<(), String>
    /// Writes a given `Header` to the database.
    ///
    /// # Arguments
    /// * `header` - A reference to the `Header` to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
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
    fn write_header(&mut self, header: &Header) -> Result<(), String>;

    // fn read_qbank(&self) -> Option<QBank>
    /// Reads the entire `QBank` (header and all questions) from the database.
    ///
    /// # Output
    /// `Option<QBank>` - An `Option<QBank>` which is `Some(QBank)` on success, or `None` on failure.
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
    /// assert!(qbank_read.is_some());
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
    /// assert!(qbank_read.is_some());
    /// let read_bank = qbank_read.unwrap();
    /// assert_eq!(read_bank.get_questions().len(), 1);
    /// assert_eq!(read_bank.get_header().get_title(), "Examination");
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn read_qbank(&self) -> Option<QBank>;

    // fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    /// Writes an entire `QBank` (header and all questions) to the database.
    ///
    /// Note: This typically writes the questions. The header should be written separately
    /// if it's not already present.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
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
    fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>;
}

impl QBDB for SQLiteDB
{
    // fn open(path: String) -> Option<Self> where Self: Sized
    /// Implements `open` for `SQLiteDB`.
    /// Appends `.qbdb` to the path if no extension is present and opens a connection.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    ///
    /// # Output
    /// `Option<SQLiteDB>` - An optional `SQLiteDB` instance if the connection is successful.
    #[inline]
    fn open(path: String) -> Option<Self>
    where Self: Sized
    {
        if path.is_empty()
            { return SQLiteDB::open_empty_in_memory(); }
        else
        {
            SQLiteDB::open_with_ext(path, "qbdb")
        }
    }

    // fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>
    /// Creates tables for `SQLiteDB`.
    ///
    /// Dynamically constructs `CREATE TABLE` SQL statements for `tblHeader` and `tblQuestions`
    /// based on the number of categories and choices required.
    ///
    /// # Arguments
    /// * `categories` - The number of category columns to create in `tblHeader`.
    /// * `choices` - The number of choice columns to create in `tblQuestions`.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>
    {
        let mut sql = r#"CREATE TABLE IF NOT EXISTS tblHeader (
    title	TEXT NOT NULL,
    name	TEXT NOT NULL,
    id  	TEXT NOT NULL,
    notice	TEXT NOT NULL"#.to_string();
        for i in 1..=categories
            { sql += format!(",\n\tcategory{}\tTEXT NOT NULL", i).as_str(); }
        sql += "\n);";
        if let Err(e) = self.conn.execute(sql.as_str(), [])
            { return Err(format!("Failed to create table tblHeader!! {}", e)) }
        if choices == 0
            { return Ok(()); }

        let mut sql = r#"CREATE TABLE IF NOT EXISTS tblQuestions (
    id	        INTEGER NOT NULL UNIQUE,
    modum       INTEGER NOT NULL,
    category    INTEGER NOT NULL,
    question	TEXT NOT NULL"#.to_string();
        for i in 1..=choices
        {
            sql += &format!(",\n\tchoice{}_text\tTEXT", i);
            sql += &format!(",\n\tchoice{}_is_answer\tBOOLEAN", i);
        }
        sql += ",\n\tPRIMARY KEY(id)\n);";
        match self.conn.execute(sql.as_str(), [])
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to create table tblQuestions!! {}", e)),
        }
    }

    // fn read_header(&self) -> Option<Header>
    /// Implements `read_header` for `SQLiteDB`.
    ///
    /// Queries the `tblHeader` table and maps the first row to a `Header` struct.
    ///
    /// # Output
    /// `Option<Header>` - An optional `Header` containing the header data from the database.
    fn read_header(&self) -> Option<Header>
    {
        let mut stmt = self.conn.prepare("SELECT * FROM tblHeader;").ok()?;
        let vec_header = stmt.query_map([], |row| {
            let mut categories = Vec::new();
            let mut i = 4;
            while let Ok(c) = row.get(i)
            {
                categories.push(c);
                i += 1;
            }
            Ok(Header::new(row.get(0)?, row.get(1)?, row.get(2)?, categories, row.get(3)?))
        }).ok()?;

        for info in vec_header
        {
            if let Ok(ff) = info
                { return Some(ff); }
        }
        None
    }

    // fn write_header_with_default(&self) -> Result<(), String>
    /// Implements `write_header_with_default` for `SQLiteDB`.
    /// Creates a default `Header` and calls `write_header`.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    #[inline]
    fn write_header_with_default(&mut self) -> Result<(), String>
    {
        self.write_header(&Header::new_with_default())
    }

    // fn write_header(&self, header: &Header) -> Result<(), String>
    /// Implements `write_header` for `SQLiteDB`.
    ///
    /// Constructs and executes an `INSERT` statement for the `tblHeader` table.
    /// It dynamically binds parameters based on the number of categories in the `Header`.
    ///
    /// # Arguments
    /// * `header` - A reference to the `Header` to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn write_header(&mut self, header: &Header) -> Result<(), String>
    {
        let _ = self.make_tables(header.get_categories().len() as u8, 0);
        let length = header.get_categories().len();
        let mut sql = format!("INSERT INTO tblHeader values (?1, ?2, ?3, ?4");
        for i in 5..(5 + length)
            { sql += format!(", ?{}", i).as_str(); }
        sql += ");";

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        params.push(header.get_title());
        params.push(header.get_name());
        params.push(header.get_id());
        params.push(header.get_notice());
        for category in header.get_categories()
            { params.push(category); }

        self.conn.execute(sql.as_str(), &params[..]).map_err(|e| e.to_string())?;
        Ok(())
    }

    // fn read_qbank(&self) -> Option<QBank>
    /// Implements `read_qbank` for `SQLiteDB`.
    ///
    /// First, it reads the header using `read_header`. Then, it queries the `tblQuestions` table,
    /// maps each row to a `Question` struct, and collects them into a new `QBank`.
    ///
    /// # Output
    /// `Option<QBank>` - An optional `QBank` containing the header and all questions from the database.
    fn read_qbank(&self) -> Option<QBank>
    {
        let header = self.read_header()?;
        let mut stmt = self.conn.prepare("SELECT * FROM tblQuestions;").ok()?;
        let vec_question = stmt.query_map([], |row| {
            let id: u16 = row.get(0)?;
            let group: u16 = row.get(1)?;
            let category: u8 = row.get(2)?;
            let question: String = row.get(3)?;
            let mut choices = Choices::new();

            // The loop will attempt to read pairs of choice_text and choice_is_answer.
            // It stops when it can't read a pair, which is safer than a fixed limit.
            
            let mut idx = 4;
            loop
            {
                if let (Ok(choice), Ok(is_answer)) = (row.get(idx), row.get(idx + 1))
                    { choices.push((choice, is_answer)); }
                else    // Stop if we can't read a complete choice pair.
                    { break; }
                idx += 2;
            }
            Ok(Question::new(id, group, category, question, choices))
        }).ok()?;

        let mut question_bank = QBank::new_with_header(header);
        for result_question in vec_question
        {
            if let Ok(question) = result_question
                { question_bank.push_question(question); }
            else
                { return None; }
        }
        Some(question_bank)
    }

    // fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    /// Implements `write_qbank` for `SQLiteDB`.
    ///
    /// Iterates through the questions in the provided `QBank` and inserts each one
    /// into the `tblQuestions` table. It dynamically constructs the `INSERT` statement
    /// and binds parameters based on the number of choices in the questions.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing questions to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    {
        let categories = qbank.get_header().get_categories().len() as u8;
        // 1. Determine the maximum number of choices in the entire bank to create a uniform SQL statement.
        let max_choices = qbank.get_max_choices();
        let _ = self.make_tables(categories, max_choices as u8);
        let _ = self.write_header(qbank.get_header());
        if qbank.get_questions().is_empty()   // Nothing to write
            { return Err("Empty QBank".to_string()); }

        // 2. Build the SQL statement dynamically.
        let mut sql = "INSERT INTO tblQuestions (id, modum, category, question".to_string();
        let mut values = "?, ?, ?, ?".to_string();
        for i in 1..=max_choices
        {
            sql += &format!(", choice{}_text, choice{}_is_answer", i, i);
            values += ", ?, ?";
        }
        sql += &format!(") VALUES ({});", values);

        // 3. Iterate through questions and execute the INSERT statement.
        for elem in qbank.get_questions()
        {
            let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
            params.push(Box::new(elem.get_id()));
            params.push(Box::new(elem.get_group()));
            params.push(Box::new(elem.get_category()));
            params.push(Box::new(elem.get_question().clone()));

            let choices = elem.get_choices();
            for i in 0..max_choices
            {
                if i < choices.len()
                {
                    params.push(Box::new(choices[i].0.clone())); // choice_text
                    params.push(Box::new(choices[i].1)); // choice_is_answer
                }
                else    // Pad with NULLs if the question has fewer choices than the max.
                {
                    params.push(Box::new(rusqlite::types::Value::Null));
                    params.push(Box::new(rusqlite::types::Value::Null));
                }
            }

            let params_for_exec: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
            self.conn.execute(&sql, &params_for_exec[..]).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}



impl QBDB for Excel
{
    // fn open(path: String) -> Option<Self> where Self: Sized
    /// Implements `open` for `Excel`.
    /// Appends `.qb.xlsx` to the path if no extension is present.
    #[inline]
    fn open(path: String) -> Option<Self>
    where Self: Sized
    {
        Excel::open_with_ext(path, "qb.xlsx")
    }

    // fn make_tables(&self, choices: u8) -> Result<(), String>
    /// Creates sheets for `Excel`.
    fn make_tables(&self, _categories: u8, choices: u8) -> Result<(), String>
    {
        let mut workbook = Workbook::new();
        let bold_border_format = Format::new().set_bold().set_border(FormatBorder::Thin);

        // 1. Create "Header" sheet
        let header_sheet = workbook.add_worksheet().set_name("Header").map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(0, 0, "Title", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(1, 0, "Name", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(2, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(3, 0, "Notice", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(4, 0, "Categories", &bold_border_format).map_err(|e| e.to_string())?;

        
        // 2. Create "Questions" sheet
        if choices != 0
        {
            let questions_sheet = workbook.add_worksheet().set_name("Questions").map_err(|e| e.to_string())?;
            questions_sheet.write_string_with_format(0, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
            questions_sheet.write_string_with_format(0, 1, "Group", &bold_border_format).map_err(|e| e.to_string())?;
            questions_sheet.write_string_with_format(0, 2, "Category", &bold_border_format).map_err(|e| e.to_string())?;
            questions_sheet.write_string_with_format(0, 3, "Question", &bold_border_format).map_err(|e| e.to_string())?;

            let mut current_col = 4;
            for i in 1..=choices
            {
                questions_sheet.write_string_with_format(0, current_col, &format!("Choice{}", i), &bold_border_format).map_err(|e| e.to_string())?;
                current_col += 1;
                questions_sheet.write_string_with_format(0, current_col, &format!("IsAnswer{}", i), &bold_border_format).map_err(|e| e.to_string())?;
                current_col += 1;
            }
        }
        workbook.save(&self.path).map_err(|e| e.to_string())
    }

    // fn read_header(&self) -> Option<Header>
    /// Implements `read_header` for `Excel`.
    fn read_header(&self) -> Option<Header>
    {
        let mut excel = open_workbook_auto(&self.path).ok()?;
        let range = excel.worksheet_range("Header").ok()?;
        let title = range.get((0, 1)).and_then(|c| c.as_string()).unwrap_or_default();
        let name = range.get((1, 1)).and_then(|c| c.as_string()).unwrap_or_default();
        let id = range.get((2, 1)).and_then(|c| c.as_string()).unwrap_or_default();
        let notice = range.get((3, 1)).and_then(|c| c.as_string()).unwrap_or_default();
        
        let mut categories = Vec::new();
        let mut col = 1;
        while let Some(cat) = range.get((4, col)).and_then(|c| c.as_string())
        {
            if cat.is_empty() { break; }
            categories.push(cat);
            col += 1;
        }
        Some(Header::new(title, name, id, categories, notice))
    }

    // fn write_header_with_default(&mut self) -> Result<(), String>
    /// Implements `write_header_with_default` for `Excel`.
    #[inline]
    fn write_header_with_default(&mut self) -> Result<(), String>
    {
        self.write_header(&Header::new_with_default())
    }

    // fn write_header(&mut self, header: &Header) -> Result<(), String>
    /// Implements `write_header` for `Excel`.
    /// This is done by reading the existing questions, creating a new QBank in memory with the new header,
    /// and then writing the entire QBank back to the file. This is necessary due to the write-only nature
    /// of the Excel writer library.
    fn write_header(&mut self, header: &Header) -> Result<(), String>
    {
        
        // Create a new QBank with the new header.
        let mut qbank = QBank::new_with_header(header.clone());

        // Read questions from the existing file, if it exists.
        if let Ok(mut excel) = open_workbook_auto(&self.path)
        {
            if let Some(range) = excel.worksheet_range("Questions").ok()
            {
                // Safely read questions, skipping header row
                for row in range.rows().skip(1) {
                    if let Some(question) = Excel::parse_question_row(row)
                    {
                        qbank.push_question(question);
                    }
                }
            }
        }
        
        // Write the entire QBank (new header + old/existing questions) back to the file.
        self.write_qbank(&qbank)
    }

    // fn read_qbank(&self) -> Option<QBank>
    /// Implements `read_qbank` for `Excel`.
    fn read_qbank(&self) -> Option<QBank> {
        let header = self.read_header()?;
        let mut qbank = QBank::new_with_header(header);

        let mut excel = open_workbook_auto(&self.path).ok()?;
        let range = excel.worksheet_range("Questions").ok()?;

        for row in range.rows().skip(1) // Skip header row
        {
            if let Some(question) = Excel::parse_question_row(row)
                { qbank.push_question(question); }
        }
        Some(qbank)
    }

    // fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    /// Implements `write_qbank` for `Excel`.
    fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    {
        let mut workbook = Workbook::new();
        let border_format = Format::new().set_border(FormatBorder::Thin);
        let bold_border_format = Format::new().set_bold().set_border(FormatBorder::Thin);
        
        // 1. Write "Header" sheet
        let header_sheet = workbook.add_worksheet().set_name("Header").map_err(|e| e.to_string())?;
        let header = qbank.get_header();
        header_sheet.write_string_with_format(0, 0, "Title", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(0, 1, header.get_title(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(1, 0, "Name", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(1, 1, header.get_name(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(2, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(2, 1, header.get_id(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(3, 0, "Notice", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(3, 1, header.get_notice(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(4, 0, "Categories", &bold_border_format).map_err(|e| e.to_string())?;
        for (i, cat) in header.get_categories().iter().enumerate()
            { header_sheet.write_string_with_format(4, i as u16 + 1, cat, &border_format).map_err(|e| e.to_string())?; }

        // 2. Write "Questions" sheet
        let questions_sheet = workbook.add_worksheet().set_name("Questions").map_err(|e| e.to_string())?;
        questions_sheet.write_string_with_format(0, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        questions_sheet.write_string_with_format(0, 1, "Group", &bold_border_format).map_err(|e| e.to_string())?;
        questions_sheet.write_string_with_format(0, 2, "Category", &bold_border_format).map_err(|e| e.to_string())?;
        questions_sheet.write_string_with_format(0, 3, "Question", &bold_border_format).map_err(|e| e.to_string())?;

        let max_choices = qbank.get_max_choices();
        for i in 1..=max_choices
        {
            questions_sheet.write_string_with_format(0, (i * 2 + 2) as u16, &format!("Choice{}", i), &bold_border_format).map_err(|e| e.to_string())?;
            questions_sheet.write_string_with_format(0, (i * 2 + 3) as u16, &format!("IsAnswer{}", i), &bold_border_format).map_err(|e| e.to_string())?;
        }

        for (row_idx, question) in qbank.get_questions().iter().enumerate()
        {
            let current_row = (row_idx + 1) as u32;
            questions_sheet.write_number_with_format(current_row, 0, question.get_id() as f64, &border_format).map_err(|e| e.to_string())?;
            questions_sheet.write_number_with_format(current_row, 1, question.get_group() as f64, &border_format).map_err(|e| e.to_string())?;
            questions_sheet.write_number_with_format(current_row, 2, question.get_category() as f64, &border_format).map_err(|e| e.to_string())?;
            questions_sheet.write_string_with_format(current_row, 3, question.get_question(), &border_format).map_err(|e| e.to_string())?;

            for (i, (choice_text, is_answer)) in question.get_choices().iter().enumerate()
            {
                let choice_col = (i * 2 + 4) as u16;
                questions_sheet.write_string_with_format(current_row, choice_col, choice_text, &border_format).map_err(|e| e.to_string())?;
                questions_sheet.write_string_with_format(current_row, choice_col + 1, &is_answer.to_string().to_uppercase(), &border_format).map_err(|e| e.to_string())?;
            }
        }
        
        workbook.save(&self.path).map_err(|e| e.to_string())
    }
}
