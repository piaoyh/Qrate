// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::{ SBDB, SBank, Student, SQLiteDB, ErrorMessage };

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

    // fn make_table(&mut self) -> Result<(), String>
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
    fn make_table(&mut self) -> Result<(), ErrorMessage>
    {
        let sql = r#"CREATE TABLE IF NOT EXISTS tblHeader (version INTEGER NOT NULL);"#;
        let mut ret = Err(ErrorMessage::FailedToMakeTableForSBank);
        let mut tx = self.conn.transaction();
        if tx.is_ok()
        {
            let tx_ = tx.unwrap();
            if tx_.execute(sql, []).is_ok()
            {
                let sql = r#"CREATE TABLE IF NOT EXISTS tblStudents (
    name        TEXT NOT NULL,
    id          TEXT NOT NULL
);"#;
                if tx_.execute(sql, []).is_ok()
                    { ret = Ok(()); }
            }
            else
            {
                ret = Err(ErrorMessage::FailedToCreateHeaderForSBank)
            }
            tx = Ok(tx_);
        }
        let _ = tx.unwrap().commit();
        ret
    }

    // fn read_sbank(&mut self) -> Result<SBank, ErrorMessage>
    /// Implements `read_sbank` for `SQLiteDB`.
    /// Queries the `tblStudents` table and maps each row to a `Student` struct.
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
        let mut ret = Err(ErrorMessage::FailedToOpenSBank);
        let mut tx = self.conn.transaction();
        if tx.is_ok()
        {
            let tx_ = tx.unwrap();
            if let Ok(mut stmt) = tx_.prepare("SELECT * FROM tblHeader;")
            {
                if let Ok(version) = stmt.query_row([], |row| row.get(0))
                {
                    if let Ok(mut stmt) = tx_.prepare("SELECT * FROM tblStudents ORDER BY id;")
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
                            ret = Ok(sbank);
                        }
                    }
                }
                else
                {
                    ret = Err(ErrorMessage::FailedToReadHeaderForSBank);
                }
            }
            else
            {
                ret = Err(ErrorMessage::FailedToReadHeaderForSBank);
            }
            tx = Ok(tx_);
        }
        let _ = tx.unwrap().commit();
        ret
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
        self.make_table()?;
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
            { Ok(()) }
        else
            { Err(ErrorMessage::FailedToWriteSBank) }
    }
}