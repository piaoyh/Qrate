// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use calamine::{ Reader, DataType }; // Add DataType here

use crate::{ SBank, SQLiteDB, Excel, Student, ErrorMessage };

/// A trait defining the database operations for a Student Bank (`SBank`).
///
/// This abstracts the storage mechanism for student data.
/// Implementors of this trait can provide different storage backends, such as
/// SQLite databases or Excel files, while exposing a consistent interface for
/// reading and writing student data.
pub trait SBDB
{
    // fn open(path: String) -> Result<Self, ErrorMessage> where Self: Sized
    /// Opens a connection to the student database.
    ///
    /// If the path does not have a file extension, a default extension
    /// specific to the database type (e.g., `.sbdb`) is appended.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    ///   For in-memory SQLite databases, use `":memory:"`.
    /// * `extention` - The file extension to append.
    ///
    /// # Returns
    /// `Result<SQLiteDB, ErrorMessage>` - A `Result` containing:
    /// * `Ok(SQLiteDB)` if the connection is successful, or
    /// * `Err(ErrorMessage::FailedToOpenSBank)` if it fails.
    /// 
    /// For in-memory databases, use `":memory:"` as the path.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{SBDB, SQLiteDB};
    ///
    /// let db = SQLiteDB::open(":memory:".to_string(), ".db");
    /// assert!(db.is_ok());
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{SBDB, Excel};
    ///
    /// let excel = Excel::open("students.sb.xlsx".to_string(), ".sb.xlsx");
    /// assert!(excel.is_ok());
    /// // assert_eq!(excel.unwrap().get_path(), "students.sb.xlsx"); // Excel doesn't have get_path method directly on SBDB
    /// ```
    fn open(path: String) -> Result<Self, ErrorMessage> where Self: Sized;

    // fn make_table(&self) -> Result<(), ErrorMessage>
    /// Creates the necessary table(s) for storing student data.
    ///
    /// For a database that already has the table,
    /// this should not produce an error.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success, or
    /// * `Err(ErrorMessage::FailedToCreateHeaderForQBank)` if it fails to
    ///   create the header table, or
    /// * `Err(ErrorMessage::FailedToMakeTableForSBank)` if it fails to
    ///   create the student list table.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{SBDB, SQLiteDB};
    ///
    /// let db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// let result = db.make_table();
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{SBDB, Excel};
    /// use std::path::Path;
    ///
    /// let file_path = "test_make_table.sb.xlsx";
    /// let excel = Excel::open(file_path.to_string(), ".sb.xlsx").unwrap();
    /// let result = excel.make_table();
    /// assert!(result.is_ok());
    /// assert!(Path::new(file_path).exists());
    /// std::fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn make_table(&self) -> Result<(), ErrorMessage>;

    // fn read_sbank(&self) -> Result<SBank, ErrorMessage>
    /// Reads all student data from the database into an `SBank`.
    ///
    /// # Returns
    /// `Result<SBank, ErrorMessage>`
    /// * `Ok(SBank)` if the SBank is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForSBank)` if reading
    ///    the header fails, or
    /// * `Err(ErrorMessage::FailedToOpenSBank)` if it fails.
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{SBank, Student, SBDB, SQLiteDB};
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// db.make_table().unwrap();
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push(Student::new("Alice".to_string(), "s123".to_string()));
    /// db.write_sbank(&sbank).unwrap();
    ///
    /// let read_sbank = db.read_sbank();
    /// assert!(read_sbank.is_ok());
    /// let read_bank = read_sbank.unwrap();
    /// assert_eq!(read_bank.len(), 1);
    /// assert_eq!(read_bank.get(0).unwrap().get_name(), "Alice");
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{SBank, Student, SBDB, Excel};
    /// use std::fs;
    ///
    /// let file_path = "test_read_sbank.sb.xlsx";
    /// let mut excel = Excel::open(file_path.to_string(), ".sb.xlsx").unwrap();
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push(Student::new("Bob".to_string(), "s456".to_string()));
    /// excel.write_sbank(&sbank).unwrap();
    ///
    /// let read_sbank = excel.read_sbank();
    /// assert!(read_sbank.is_ok());
    /// let read_bank = read_sbank.unwrap();
    /// assert_eq!(read_bank.len(), 1);
    /// assert_eq!(read_bank.get(0).unwrap().get_name(), "Bob");
    ///
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn read_sbank(&self) -> Result<SBank, ErrorMessage>;

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
    ///
    /// # Example 1 for SQLiteDB
    /// ```
    /// use qrate::{SBank, Student, SBDB, SQLiteDB};
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// db.make_table().unwrap();
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push(Student::new("Bob".to_string(), "s456".to_string()));
    ///
    /// let result = db.write_sbank(&sbank);
    /// assert!(result.is_ok());
    ///
    /// // Verify by reading back
    /// assert_eq!(db.read_sbank().unwrap().len(), 1);
    /// ```
    ///
    /// # Example 2 for Excel
    /// ```
    /// use qrate::{SBank, Student, SBDB, Excel};
    /// use std::path::Path;
    /// use std::fs;
    ///
    /// let file_path = "test_write_sbank.sb.xlsx";
    /// let mut excel = Excel::open(file_path.to_string(), ".sb.xlsx").unwrap();
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push(Student::new("Charlie".to_string(), "s789".to_string()));
    ///
    /// let result = excel.write_sbank(&sbank);
    /// assert!(result.is_ok());
    /// assert!(Path::new(file_path).exists());
    ///
    /// // Verify by reading back
    /// assert_eq!(excel.read_sbank().unwrap().len(), 1);
    ///
    /// fs::remove_file(file_path).unwrap(); // Clean up
    /// ```
    fn write_sbank(&mut self, sbank: &SBank) -> Result<(), ErrorMessage>;
}


impl SBDB for SQLiteDB
{
    // fn open(path: String, extention: &str) ->Result<Self, ErrorMessage>
    /// Implements `open` for `SQLiteDB`.
    /// 
    /// Appends `.sbdb` to the path
    /// if no extension is present and opens a connection.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    /// * `extention` - The file extension to append.
    ///
    /// # Returns
    /// `Result<SQLiteDB, ErrorMessage>` - A `Result` containing:
    /// * `Ok(SQLiteDB)` if the connection is successful, or
    /// * `Err(ErrorMessage::FailedToOpenSBank)` if it fails.
    /// 
    /// For in-memory databases, use `":memory:"` as the path.
    fn open(path: String) -> Result<Self, ErrorMessage>
    {
        if !path.is_empty()
        {
            if let Ok(db) = SQLiteDB::open_with_ext(path, "sbdb")
                { return Ok(db); }
        }
        Err(ErrorMessage::FailedToOpenSBank)
    }

    // fn make_table(&self) -> Result<(), String>
    /// Implements `make_table` for `SQLiteDB`.
    /// Executes a `CREATE TABLE` SQL statement for `tblStudents`.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success, or
    /// * `Err(ErrorMessage::FailedToCreateHeaderForQBank)` if it fails to
    ///   create the header table, or
    /// * `Err(ErrorMessage::FailedToMakeTableForSBank)` if it fails to
    ///   create the student list table.
    fn make_table(&self) -> Result<(), ErrorMessage>
    {
        let sql = r#"CREATE TABLE IF NOT EXISTS tblHeader (version INTEGER NOT NULL);"#;
        if let Err(e) = self.conn.execute(sql, [])
            { return Err(ErrorMessage::FailedToCreateHeaderForSBank); }
        
        let sql = r#"CREATE TABLE IF NOT EXISTS tblStudents (
    name        TEXT NOT NULL,
    id          TEXT NOT NULL
);"#;
        match self.conn.execute(sql, [])
        {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorMessage::FailedToMakeTableForSBank),
        }
    }

    // fn read_sbank(&self) -> Result<SBank, ErrorMessage>
    /// Implements `read_sbank` for `SQLiteDB`.
    /// Queries the `tblStudents` table and maps each row to a `Student` struct.
    ///
    /// # Returns
    /// `Result<SBank, ErrorMessage>`
    /// * `Ok(SBank)` if the SBank is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForSBank)` if reading
    ///    the header fails, or
    /// * `Err(ErrorMessage::FailedToOpenSBank)` if it fails.
    fn read_sbank(&self) -> Result<SBank, ErrorMessage>
    {
        let mut sbank = SBank::new();
        if let Ok(mut stmt) = self.conn.prepare("SELECT * FROM tblHeader;")
        {
            if let Ok(version) = stmt.query_row([], |row| row.get(0))
            {
                if let Ok(mut stmt) = self.conn.prepare("SELECT * FROM tblStudents ORDER BY id;")
                {
                    if let Ok(student_iter) = stmt.query_map([], |row| {
                        Ok(Student::new(row.get(0)?, row.get(1)?))
                    })
                    {
                        let students = student_iter.filter_map(|res| res.ok()).collect::<Vec<Student>>();
                        for student in students
                            { sbank.push_student(student); }
                        sbank.set_version(version);
                        sbank.sort();
                        return Ok(sbank);
                    }
                }
            }
            else
            {
                return Err(ErrorMessage::FailedToReadHeaderForSBank);
            }
        }
        Err(ErrorMessage::FailedToOpenSBank)
    }

    // fn write_sbank(&self, sbank: &SBank) -> Result<(), String>
    /// Implements `write_sbank` for `SQLiteDB`.
    /// Iterates through the `SBank` and inserts each `Student` into the `tblStudents` table.
    ///
    /// # Arguments
    /// * `sbank` - A reference to the `SBank` to be written to the database.
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
        if let Ok(_) = self.make_table()
        {
            if sbank.is_empty()
                { return Err(ErrorMessage::EmptySBank); }  // Nothing to write

            let mut tx = self.conn.transaction();
            if tx.is_ok()
            {
                let tx_ = tx.unwrap();
                if let Ok(mut stmt) = tx_.prepare("INSERT INTO tblHeader (version) VALUES (?1);")
                {
                    if stmt.execute((sbank.get_version(),)).is_ok()
                    {
                        if let Ok(mut stmt) = tx_.prepare("INSERT INTO tblStudents (name, id) VALUES (?1, ?2);")
                        {
                            for student in sbank.get_students()
                            {
                                if stmt.execute((student.get_name(), student.get_id())).is_err()
                                    { break; }
                            }
                        }
                    }
                }
                tx = Ok(tx_);
            }
            if tx.unwrap().commit().is_ok()
                { return Ok(()); }
        }
        else
        {
            return Err(ErrorMessage::FailedToMakeTableForSBank);
        }
        Err(ErrorMessage::FailedToWriteSBank)
    }
}


#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl SBDB for Excel
{
    // fn open(path: String) -> Result<Self, ErrorMessage>
    /// Implements `open` for `Excel`.
    /// Appends `.sb.xlsx` to the path if no extension is present and opens an Excel file.
    ///
    /// # Arguments
    /// * `path` - The file path for the Excel file.
    /// * `extention` - The file extension to append.
    ///
    /// # Returns
    /// An `Result<Self, ErrorMessage>`which is:
    /// * `Ok(Excel)` always.
    #[inline]
    fn open(path: String) -> Result<Self, ErrorMessage>
    where Self: Sized
    {
        Excel::open_with_ext(path, "sb.xlsx")
    }

    // fn make_table(&self) -> Result<(), String>
    /// Creates a new Excel file with a "Students" sheet and headers.
    /// 
    /// # Returns
    /// `Result<(), String>` - `Ok(())` on success,
    /// or an error message string on failure.
    fn make_table(&self) -> Result<(), String>
    {
        let mut workbook = rust_xlsxwriter::Workbook::new();
        let format = rust_xlsxwriter::Format::new().set_bold();

        let sheet = workbook.add_worksheet().set_name("Header").map_err(|e| e.to_string())?;
        sheet.write_string_with_format(0, 0, "Version", &format).map_err(|e| e.to_string())?;

        let sheet = workbook.add_worksheet().set_name("Students").map_err(|e| e.to_string())?;

        sheet.write_string_with_format(0, 0, "Name", &format).map_err(|e| e.to_string())?;
        sheet.write_string_with_format(0, 1, "ID", &format).map_err(|e| e.to_string())?;

        workbook.save(&self.path).map_err(|e| e.to_string())
    }

    // fn read_sbank(&self) -> Option<SBank>
    /// Reads students from the "Students" sheet in an Excel file.
    /// Assumes the first row contains headers
    /// and starts reading from the second row.
    /// 
    /// # Returns
    /// `Result<SBank, ErrorMessage>` - A `Result` containing the `SBank` if successful,
    /// or an error message if reading fails.
    fn read_sbank(&self) -> Result<SBank, ErrorMessage>
    {
        let mut sbank = SBank::new();
        let mut excel = calamine::open_workbook_auto(&self.path).ok()?;
        let range = excel.worksheet_range("Header").ok()?;
        for row in range.rows().skip(1)
            { sbank.set_version(row.get(0).and_then(|d| d.as_i64())? as u32); }

        let range = excel.worksheet_range("Students").ok()?;
        for row in range.rows().skip(1)
        { // Skip header row
            sbank.push_student(Student::new(
                row.get(0).and_then(|d| d.as_string())?,
                row.get(1).and_then(|d| d.as_string())? // Assuming ID is always string or convertible
            ));
        }
        Ok(sbank)
    }
    
    // fn write_sbank(&mut self, sbank: &SBank) -> Result<(), String>
    /// Writes a collection of students to a "Students" sheet in an Excel file.
    /// If the file does not exist, it will be created. If it already exists,
    /// the "Students" sheet will be overwritten.
    /// 
    /// # Arguments
    /// * `sbank` - A reference to the `SBank`
    ///   containing the students to be written to the Excel file.
    /// 
    /// # Returns
    /// `Result<(), String>` - `Ok(())` on success,
    /// or an error message string on failure.
    fn write_sbank(&mut self, sbank: &SBank) -> Result<(), String>
    {
        let mut workbook = rust_xlsxwriter::Workbook::new();
        let header_format = rust_xlsxwriter::Format::new().set_bold();

        let sheet = workbook.add_worksheet().set_name("Header").map_err(|e| e.to_string())?;
        sheet.write_string_with_format(0, 0, "Version", &header_format).map_err(|e| e.to_string())?;
        sheet.write_string(1, 0, "1").map_err(|e| e.to_string())?;

        let sheet = workbook.add_worksheet().set_name("Students").map_err(|e| e.to_string())?;
        // Write header
        sheet.write_string_with_format(0, 0, "Name", &header_format).map_err(|e| e.to_string())?;
        sheet.write_string_with_format(0, 1, "ID", &header_format).map_err(|e| e.to_string())?;

        // Write student data
        for (row_idx, student) in sbank.get_students().iter().enumerate()
        {
            let row = (row_idx + 1) as u32;
            sheet.write_string(row, 0, student.get_name()).map_err(|e| e.to_string())?;
            sheet.write_string(row, 1, student.get_id()).map_err(|e| e.to_string())?;
        }

        workbook.save(&self.path).map_err(|e| e.to_string())
    }
}