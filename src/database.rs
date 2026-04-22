// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::ptr::copy_nonoverlapping;
use rusqlite::{ Connection, ffi, Error };

use crate::check_path;


/// Represents an SQLite database connection.
///
/// This struct provides a simple interface for opening and closing an SQLite database connection.
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
    // pub fn open_with_ext(path: String, extention: &str) -> Option<Self>
    /// Opens a new connection to an SQLite database.
    ///
    /// # Arguments
    /// * `path` - The path to the database file.
    /// * `extention` - The file extension to append if the path does not have one.
    ///
    /// # Output
    /// An `Option<Self>` which is `Some(SQLiteDB)` on successful connection, or `None` on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// // In a real scenario, you would provide a file path.
    /// let db = SQLiteDB::open_with_ext("./CProgramming".to_string(), "qbdb");
    /// assert!(db.is_some());
    /// ```
    pub fn open_with_ext(path: String, extention: &str) -> Option<Self>
    {
        let extended_path = check_path(path, extention);
        if let Ok(con) = Connection::open(&extended_path)
            { Some(Self { path: extended_path, conn: con }) }
        else
            { None }
    }

    // pub fn open_in_memory(data: &[u8]) -> Option<Self>
    /// Opens a new connection to an SQLite database in memory.
    ///
    /// # Arguments
    /// * `data` - contains the contents of the SQLite database opened outside and read.
    ///
    /// # Output
    /// An `Option<Self>` which is `Some(SQLiteDB)` on successful connection, or `None` on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let data = std::fs::read("./CProgramming.qbdb").unwrap();
    /// 
    /// // Using an in-memory database for the example.
    /// let db = SQLiteDB::open_in_memory(&data);
    /// assert!(db.is_some());
    /// ```
    pub fn open_in_memory(data: &[u8]) -> Option<Self>
    {
        if let Ok(conn) = Connection::open_in_memory()
        {
            let db_handle = unsafe { conn.handle() };
            let size = data.len() as i64;
            let data_ptr = unsafe { ffi::sqlite3_malloc(size as i32) as *mut u8 };
            if data_ptr.is_null()
                { return None; }
            unsafe { copy_nonoverlapping(data.as_ptr(), data_ptr, data.len()); }

            // 4. sqlite3_deserialize 호출
            // SQLITE_DESERIALIZE_FREEONCLOSE(1) | SQLITE_DESERIALIZE_RESIZEABLE(2)
            let flags = 1 | 2;
            let result = unsafe { ffi::sqlite3_deserialize(db_handle, b"main\0".as_ptr() as *const i8, data_ptr as *mut u8, size, size, flags) };
            if result == 0 { Some(Self { path: String::new(), conn }) } else { None }
        }
        else
        {
            None
        }
    }

    // pub fn close(self) -> Result<(), (Connection, Error)>
    /// Closes the database connection.
    ///
    /// # Output
    /// `Ok(())` if the connection is closed successfully, `Err(())` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string(), ".db").unwrap();
    /// let result = db.close();
    /// assert!(result.is_ok());
    /// ```
    pub fn close(self) -> Result<(), (Connection, Error)>
    {
        match self.conn.close()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
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
    pub fn set_path(&mut self, path: String)
    {
        self.path = path + ".db"; // Appending .db for consistency
    }

    // pub fn get_path(&self) -> &String
    /// Gets the path of the database file.
    ///
    /// # Output
    /// `&String` - A reference to the path of the database file.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    ///
    /// let db = SQLiteDB::open("my_db".to_string(), ".db").unwrap();
    /// assert_eq!(db.get_path(), "my_db.db");
    /// ```
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
    pub fn get_connection(&self) -> &Connection
    {
        &self.conn
    }
}
