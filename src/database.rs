// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::ptr::copy_nonoverlapping;
// use std::time::Duration;
use rusqlite::{ Connection, ffi };
// use rusqlite::backup::Backup;

use crate::{ ErrorMessage, check_path };


/// Represents an SQLite database connection.
/// 
/// This struct provides a simple interface for opening and closing an SQLite
/// database connection.
/// It includes methods for opening a database from a file or in memory, saving
/// the database to a file or in memory, and closing the connection.
/// The `SQLiteDB` struct encapsulates the database path and the
/// `rusqlite::Connection` object, allowing for easy management of the database
/// connection throughout the application.
#[derive(Debug)]
pub struct SQLiteDB
{
    /// The path to the SQLite database file.
    pub(crate) path: String,

    /// The `rusqlite::Connection` object.
    pub(crate) conn: Connection,
}

impl SQLiteDB
{
    // pub fn open_with_ext(path: String, extention: &str) -> Result<Self, ErrorMessage>
    /// Opens a new connection to an SQLite database.
    ///
    /// # Arguments
    /// * `path` - The path to the database file.
    /// * `extention` - The file extension to append
    ///   if the path does not have one.
    ///
    /// # Returns
    /// An `Result<Self, ErrorMessage>`which is:
    /// * `Ok(SQLiteDB)` on successful connection, or
    /// * `Err(ErrorMessage::FailedToOpenDatabase)` on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// // In a real scenario, you would provide a file path.
    /// let db = SQLiteDB::open_with_ext("./CProgramming".to_string(), "qbdb");
    /// assert!(db.is_ok());
    /// ```
    pub fn open_with_ext(path: String, extention: &str) -> Result<Self, ErrorMessage>
    {
        let extended_path = check_path(path, extention);
        if let Ok(con) = Connection::open(&extended_path)
            { Ok(Self { path: extended_path, conn: con }) }
        else
            { Err(ErrorMessage::FailedToOpenDatabase) }
    }

    // pub fn open_in_memory(data: &[u8]) -> Result<Self, ErrorMessage>
    /// Opens a new connection to an SQLite database in memory.
    ///
    /// # Arguments
    /// * `data` - contains the contents of the SQLite database opened outside
    /// of the application and read.
    ///
    /// # Output
    /// An `Result<Self, ErrorMessage>` which is:
    /// * `Ok(SQLiteDB)` on successful connection, or
    /// * `Err(ErrorMessage::FailedToReceiveDatabaseFromMemory)` on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let data = std::fs::read("./CProgramming.qbdb").unwrap();
    /// 
    /// // Using an in-memory database for the example.
    /// let db = SQLiteDB::open_in_memory(&data);
    /// assert!(db.is_ok());
    /// ```
    pub fn open_in_memory(data: &[u8]) -> Result<Self, ErrorMessage>
    {
        if let Ok(conn) = Connection::open_in_memory()
        {
            let db_handle = unsafe { conn.handle() };
            let size = data.len() as i64;
            let data_ptr = unsafe { ffi::sqlite3_malloc(size as i32) as *mut u8 };
            if data_ptr.is_null()
                { return Err(ErrorMessage::FailedToReceiveDatabaseFromMemory); }
            unsafe { copy_nonoverlapping(data.as_ptr(), data_ptr, data.len()); }

            // SQLITE_DESERIALIZE_FREEONCLOSE(1) | SQLITE_DESERIALIZE_RESIZEABLE(2)
            let flags = 1 | 2;
            let result = unsafe { ffi::sqlite3_deserialize(db_handle, b"main\0".as_ptr() as *const i8, data_ptr as *mut u8, size, size, flags) };
            if result == 0
                { Ok(Self { path: String::new(), conn }) }
            else
                { Err(ErrorMessage::FailedToReceiveDatabaseFromMemory) }
        }
        else
        {
            Err(ErrorMessage::FailedToReceiveDatabaseFromMemory)
        }
    }

    // pub fn open_empty_in_memory() -> Result<Self, ErrorMessage>
    /// Opens a new connection to an empty SQLite database in memory.
    /// 
    /// # Returns
    /// An `Result<Self, ErrorMessage>` which is:
    /// * `Ok(SQLiteDB)` on successful connection, or
    /// * `Err(ErrorMessage::FailedToOpenEmptyDatabaseInMemory)` on failure.
    /// 
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// 
    /// // Using an in-memory database for the example.
    /// let db = SQLiteDB::open_empty_in_memory();
    /// assert!(db.is_ok());
    /// ```
    pub fn open_empty_in_memory() -> Result<Self, ErrorMessage>
    {
        if let Ok(conn) = Connection::open_in_memory()
            { Ok(Self { path: String::new(), conn }) }
        else
            { Err(ErrorMessage::FailedToOpenEmptyDatabaseInMemory) }
    }

    // pub fn save_to_file(&self, file_path: &str) -> Result<(), ErrorMessage>
    /// Saves the database to a file.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file where the database will be saved.
    ///
    /// # Returns
    /// `Ok(())` if the database is saved successfully,
    /// `Err(ErrorMessage::FailedToWriteDatabase)` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// let result = db.save_to_file("my_db.db");
    /// assert!(result.is_ok());
    /// ```
    pub fn save_to_file(&self, file_path: &str) -> Result<(), ErrorMessage>
    {
        let sql = format!("VACUUM INTO '{}'", file_path);
        match self.conn.execute(sql.as_str(), [])
        {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorMessage::FailedToWriteDatabase),
        }
    }

    // pub fn save_in_memory(&self) -> Result<Vec<u8>, Error>
    /// Saves the database to a byte vector in memory.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` containing the serialized database if successful,
    /// * `Err(ErrorMessage::FailedToWriteDatabaseToMemory)` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// let result = db.save_in_memory();
    /// assert!(result.is_ok());
    /// let data = result.unwrap();
    /// assert!(!data.is_empty());
    /// ```
    pub fn save_in_memory(&mut self) -> Result<Vec<u8>, ErrorMessage>
    {
        let db_handle = unsafe { self.conn.handle() };
        let mut size: i64 = 0;
        let data_ptr = unsafe { ffi::sqlite3_serialize(db_handle, b"main\0".as_ptr() as *const i8, &mut size as *mut i64, 0) };
        if data_ptr.is_null()
        {
            Err(ErrorMessage::FailedToWriteDatabaseToMemory)
        }
        else
        {
            let data = unsafe { std::slice::from_raw_parts(data_ptr as *const u8, size as usize).to_vec() };
            unsafe { ffi::sqlite3_free(data_ptr as *mut std::ffi::c_void) };
            Ok(data)
        }
    }

    // pub fn close(self) -> Result<(), ErrorMessage>
    /// Closes the database connection.
    ///
    /// # Returns
    /// `Ok(())` if the connection is closed successfully,
    /// `Err(ErrorMessage::FailedToCloseDatabase)` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// let result = db.close();
    /// assert!(result.is_ok());
    /// ```
    #[inline]
    pub fn close(self) -> Result<(), ErrorMessage>
    {
        match self.conn.close()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorMessage::FailedToCloseDatabase),
        }
    }

    // pub fn vacuum(&mut self) -> Result<usize, ErrorMessage>
    /// Vacuum database
    /// 
    /// # Returns
    /// * `Ok(usize)` containing the number of rows affected by the VACUUM
    /// command.
    /// * `Err(ErrorMessage)` otherwise
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// db.vacuum();
    /// assert!(db.is_ok());
    /// ```
    #[inline]
    pub fn vacuum(&mut self) -> Result<usize, ErrorMessage>
    {
        match self.conn.execute("VACUUM", [])
        {
            Ok(rows_affected) => Ok(rows_affected),
            Err(_) => Err(ErrorMessage::FailedToVacuumDatabase),
        }
    }

    // pub fn set_path(&mut self, path: String)
    /// Sets the path of the database file.
    ///
    /// # Arguments
    /// * `path` - The new path for the database file.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// db.set_path("new_path".to_string());
    /// assert_eq!(db.get_path(), "new_path.db");
    /// ```
    #[inline]
    pub fn set_path(&mut self, path: String)
    {
        self.path = path + ".db"; // Appending .db for consistency
    }

    // pub fn get_path(&self) -> &String
    /// Gets the path of the database file.
    ///
    /// # Returns
    /// `&String` - A reference to the path of the database file.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let db = SQLiteDB::open("my_db".to_string(), ".db").unwrap();
    /// assert_eq!(db.get_path(), "my_db.db");
    /// ```
    #[inline]
    pub fn get_path(&self) -> &String
    {
        &self.path
    }

    // pub fn set_connection(&mut self, conn: Connection)
    /// Sets the database connection.
    ///
    /// # Arguments
    /// * `conn` - The new `rusqlite::Connection` to be used by the database.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use rusqlite::Connection;
    ///
    /// let mut db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// let new_conn = Connection::open_in_memory().unwrap();
    /// db.set_connection(new_conn);
    /// // You can't directly compare connections, but you can check if it's not null/default if applicable.
    /// // For example, by attempting an operation.
    /// db.get_connection().execute_batch("CREATE TABLE test (id INTEGER);").unwrap();
    /// ```
    #[inline]
    pub fn set_connection(&mut self, conn: Connection)
    {
        self.conn = conn;
    }

    // pub fn get_connection(&self) -> &Connection
    /// Gets a reference to the database connection.
    ///
    /// # Output
    /// `&Connection` - A reference to the `rusqlite::Connection` object.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// let conn_ref = db.get_connection();
    /// assert!(conn_ref.is_autocommit());
    /// ```
    #[inline]
    pub fn get_connection(&self) -> &Connection
    {
        &self.conn
    }
}
