// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::{ QBDB, Header, QBank, SQLiteDB, Choices, Question, ErrorMessage };


impl QBDB for SQLiteDB
{
    // fn open(path: String) -> Result<Self, ErrorMessage> where Self: Sized
    /// Implements `open` for `SQLiteDB`.
    /// Appends `.qbdb` to the path if no extension is present and opens a connection.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    ///
    /// # Returns
    /// `Result<SQLiteDB, ErrorMessage>` - A `Result` containing:
    /// * `Ok(SQLiteDB)` if the connection is successful, or
    /// * `Err(ErrorMessage::FailedToOpenQBank)` if it fails.
    /// 
    /// For in-memory databases, use `":memory:"` as the path.
    fn open(path: String) -> Result<Self, ErrorMessage>
    where Self: Sized
    {
        if path == ":memory:"
        {
            if let Ok(db) = SQLiteDB::open_empty_in_memory()
                { return Ok(db); }
        }
        else if !path.is_empty()
        {
            if let Ok(db) = SQLiteDB::open_with_ext(path, "qbdb")
                { return Ok(db); }
        }
        Err(ErrorMessage::FailedToOpenQBank)
    }

    // fn make_tables(&mut self, categories: u8, choices: u8) -> Result<(), String>
    /// Creates tables for `SQLiteDB`.
    ///
    /// Dynamically constructs `CREATE TABLE` SQL statements for `tblHeader` and `tblQuestions`
    /// based on the number of categories and choices required.
    ///
    /// # Arguments
    /// * `categories` - The number of category columns to create in `tblHeader`.
    /// * `choices` - The number of choice columns to create in `tblQuestions`.
    ///
    /// # Returns
    /// `Result<(), ErrorMessage>`
    /// * `Ok(())` on success, or
    /// * `Err(ErrorMessage::FailedToCreateHeaderForQBank)` if it fails to
    ///   create the header table, or
    /// * `Err(ErrorMessage::FailedToMakeTableForQBank)` if `categories` is `0`
    ///   or it fails to create the question table.
    fn make_tables(&mut self, categories: u8, choices: u8) -> Result<(), ErrorMessage>
    {
        let categories = if categories == 0 { 4 } else { categories };

        let mut sql = r#"CREATE TABLE IF NOT EXISTS tblHeader (
    version INTEGER NOT NULL,
    title	TEXT NOT NULL,
    name	TEXT NOT NULL,
    id  	TEXT NOT NULL,
    notice	TEXT NOT NULL"#.to_string();
        for i in 1..=categories
            { sql += format!(",\n\tcategory{}\tTEXT NOT NULL", i).as_str(); }
        sql += "\n);";

        let mut ret = Ok(());
        let mut tx = self.conn.transaction();
        if tx.is_ok()
        {
            let tx_ = tx.unwrap();
            if tx_.execute(sql.as_str(), []).is_err()
            {
                ret = Err(ErrorMessage::FailedToCreateHeaderForQBank);
            }
            else
            {
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
                if tx_.execute(sql.as_str(), []).is_err()
                    { ret = Err(ErrorMessage::FailedToMakeTableForQBank); }
            }
            tx = Ok(tx_);
            let _ = tx.unwrap().commit();
        }
        ret
    }

    // fn read_header(&mut self) -> Result<Header, ErrorMessage>
    /// Implements `read_header` for `SQLiteDB`.
    ///
    /// Queries the `tblHeader` table and maps the first row to a `Header` struct.
    ///
    /// # Returns
    /// `Result<Header, ErrorMessage>`
    /// * `Ok(Header)` if the header is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForQBank)` if it fails.
    fn read_header(&mut self) -> Result<Header, ErrorMessage>
    {
        let mut ret = Err(ErrorMessage::FailedToReadHeaderForQBank);
        let mut tx = self.conn.transaction();
        if tx.is_ok()
        {
            let tx_ = tx.unwrap();
            if let Ok(mut stmt) = tx_.prepare("SELECT * FROM tblHeader;")
            {
                if let Ok(vec_header) = stmt.query_map([], |row| {
                    let mut categories = Vec::new();
                    let mut i = 5; // Start from the 6th column (index 5) for categories
                    while let Ok(c) = row.get(i)
                    {
                        categories.push(c);
                        i += 1;
                    }
                    let mut header = Header::new(row.get(1)?, row.get(2)?,row.get(3)?,  categories, row.get(4)?);
                    header.set_version(row.get(0)?);
                    Ok(header)
                })
                {
                    for info in vec_header
                    {
                        if let Ok(header) = info
                        {
                            ret = Ok(header);
                            break;
                        }
                    }
                }
            }
            tx = Ok(tx_);
        }
        let _ = tx.unwrap().commit();
        ret
    }

    // fn write_header_with_default(&self) -> Result<(), ErrorMessage>
    /// Implements `write_header_with_default` for `SQLiteDB`.
    /// Creates a default `Header` and calls `write_header`.
    ///
    /// # Returns
    /// `Result<Header, ErrorMessage>`
    /// * `Ok(Header)` if the header is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForQBank)` if it fails.
    #[inline]
    fn write_header_with_default(&mut self) -> Result<(), ErrorMessage>
    {
        self.write_header(&Header::new_with_default())
    }

    // fn write_header(&mut self, header: &Header) -> Result<(), String>
    /// Implements `write_header` for `SQLiteDB`.
    ///
    /// Constructs and executes an `INSERT` statement for the `tblHeader` table.
    /// It dynamically binds parameters based on the number of categories in the `Header`.
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
        let cat_count = if header.get_categories().is_empty() { 4 } else { header.get_categories().len() };
        self.make_tables(cat_count as u8, 0)?;
        let mut sql = format!("INSERT INTO tblHeader values (?1, ?2, ?3, ?4, ?5");
        for i in 6..(6 + cat_count)
            { sql += format!(", ?{}", i).as_str(); }
        sql += ");";

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let version = header.get_version();
        let empty_str = String::new();
        params.push(&version);
        params.push(header.get_title());
        params.push(header.get_name());
        params.push(header.get_id());
        params.push(header.get_notice());
        if header.get_categories().is_empty()
        {
            for _ in 0..4
                { params.push(&empty_str); }
        }
        else
        {
            for category in header.get_categories()
                { params.push(category); }
        }

        let mut ret = Ok(());
        let mut tx = self.conn.transaction();
        if tx.is_ok()
        {
            let tx_ = tx.unwrap();
            if tx_.execute(sql.as_str(), &params[..]).is_err()
                { ret = Err(ErrorMessage::FailedToWriteHeaderForQBank); }
            tx = Ok(tx_);
        }
        let _ = tx.unwrap().commit();
        ret
    }

    // fn read_qbank(&mut self) -> Result<QBank, ErrorMessage>
    /// Implements `read_qbank` for `SQLiteDB`.
    ///
    /// First, it reads the header using `read_header`. Then, it queries the `tblQuestions` table,
    /// maps each row to a `Question` struct, and collects them into a new `QBank`.
    ///
    /// # Returns
    /// `Result<QBank, ErrorMessage>`
    /// * `Ok(QBank)` if the QBank is successfully read, or
    /// * `Err(ErrorMessage::FailedToReadHeaderForQBank)` if reading
    ///    the header fails, or
    /// * `Err(ErrorMessage::FailedToOpenQBank)` if it fails.
    fn read_qbank(&mut self) -> Result<QBank, ErrorMessage>
    {
        let header = self.read_header()?;
        let mut ret = Err(ErrorMessage::FailedToOpenQBank);
        let mut tx = self.conn.transaction();
        if tx.is_ok()
        {
            let tx_ = tx.unwrap();
            if let Ok(mut stmt) = tx_.prepare("SELECT * FROM tblQuestions ORDER BY id;")
            {
                if let Ok(vec_question) = stmt.query_map([], |row| {
                    let id: u16 = row.get(0)?;
                    let group: u16 = row.get::<_, Option<u16>>(1).ok().flatten().unwrap_or(id);
                    let question: String = row.get::<_, Option<String>>(3).ok().flatten().unwrap_or_default();
                    let mut choices = Choices::new();

                    let mut k = 0;
                    let mut is_category_4 = false;
                    loop
                    {
                        let choice_text_idx = 4 + k * 2;
                        let is_answer_idx = 5 + k * 2;

                        let is_answer_opt: Option<bool> = row.get(is_answer_idx).ok().flatten();
                        match is_answer_opt
                        {
                            None => {
                                if k == 0
                                    { is_category_4 = true; }
                                break;
                            }
                            Some(is_answer) => {
                                let choice_text = row
                                    .get::<_, Option<String>>(choice_text_idx)
                                    .ok()
                                    .flatten()
                                    .unwrap_or_default();
                                choices.push((choice_text, is_answer));
                                k += 1;
                            }
                        }
                    }

                    let category: u8 = if is_category_4
                        { 4 }
                    else
                        { row.get::<_, Option<u8>>(2).ok().flatten().unwrap_or(0) };

                    Ok(Question::new(id, group, category, question, choices))
                })
                {
                    let mut ok = true;
                    let mut question_bank = QBank::new_with_header(header);
                    for result_question in vec_question
                    {
                        if let Ok(question) = result_question
                        {
                            question_bank.push_question(question);
                        }
                        else
                        {
                            ok = false;
                            break;
                        }
                    }
                    if ok
                    {
                        question_bank.sort();
                        ret = Ok(question_bank);
                    }
                }
            }
            tx = Ok(tx_);
        }
        let _ = tx.unwrap().commit();
        ret
    }

    // fn write_qbank(&mut self, qbank: &QBank) -> Result<(), ErrorMessage>
    /// Implements `write_qbank` for `SQLiteDB`.
    ///
    /// Iterates through the questions in the provided `QBank` and inserts each one
    /// into the `tblQuestions` table. It dynamically constructs the `INSERT` statement
    /// and binds parameters based on the number of choices in the questions.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing questions to be written to the database.
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

        // 2. Create necessary tables in the database.
        self.make_tables(categories, max_choices as u8)?;

        // 3. Write "Header" data to the database.
        self.write_header(qbank.get_header())?;

        // 4. Build the SQL statement dynamically.
        let mut sql = "INSERT INTO tblQuestions (id, modum, category, question".to_string();
        let mut values = "?, ?, ?, ?".to_string();
        for i in 1..=max_choices
        {
            sql += &format!(", choice{}_text, choice{}_is_answer", i, i);
            values += ", ?, ?";
        }
        sql += &format!(") VALUES ({});", values);

        let mut ret = Ok(());
        let mut tx = self.conn.transaction();
        if tx.is_ok()
        {
            let tx_ = tx.unwrap();

            let mut fallback_questions = Vec::new();
            let questions = if qbank.get_questions().is_empty()
            {
                fallback_questions.push(Question::new(1, 1, 4, String::new(), Choices::new()));
                &fallback_questions
            }
            else
            {
                qbank.get_questions()
            };

            // 3. Iterate through questions and execute the INSERT statement.
            for elem in questions
            {
                let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
                params.push(Box::new(elem.get_id()));
                params.push(Box::new(elem.get_group()));
                params.push(Box::new(elem.get_category()));
                params.push(Box::new(elem.get_question().clone()));

                let choices = elem.get_choices();
                let num_choices = elem.get_number_of_choices();
                for i in 0..max_choices
                {
                    if i < num_choices
                    {
                        params.push(Box::new(choices[i].0.clone())); // choice_text
                        params.push(Box::new(choices[i].1)); // choice_is_answer
                    }
                    else    // Pad with null if the question has fewer choices than max_choices.
                    {
                        params.push(Box::new(rusqlite::types::Null));
                        params.push(Box::new(rusqlite::types::Null));
                    }
                }

                let params_for_exec: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
                if tx_.execute(&sql, &params_for_exec[..]).is_err()
                {
                    ret = Err(ErrorMessage::FailedToWriteQBank);
                    break;
                }
            }
            tx = Ok(tx_);
        }
        let _ = tx.unwrap().commit();
        ret
    }
}