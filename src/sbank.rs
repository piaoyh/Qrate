// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::Student;

/// A type alias for a vector of `Student`s, representing a bank of students.
pub type SBank = Vec<Student>;

pub trait SBankHelper
{
    // fn get_student(&self, number: usize) -> Option<Student>
    /// Gets a `Student` object by its 1-based index.
    ///
    /// # Arguments
    /// * `number` - The 1-based index of the question to retrieve.
    ///
    /// # Returns
    /// `Option<Student>` - An optional `Student` object at the specified index.
    ///
    /// # Examples
    /// ```
    /// use qrate::{SBank, SBankHelper, Student};
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push(Student::new("Alice".to_string(), "s123".to_string()));
    /// sbank.push(Student::new("Bob".to_string(), "s456".to_string()));
    ///
    /// assert_eq!(sbank.get_student(1).unwrap().get_name(), "Alice");
    /// assert!(sbank.get_student(3).is_none());
    /// ```
    fn get_student(&self, number: usize) -> Option<Student>;

    // fn set_student(&mut self, number: usize, student: Student) -> bool
    /// Sets a `Student` object at a specific 1-based index.
    ///
    /// # Arguments
    /// * `number` - The 1-based index of the student to set.
    /// * `student` - The `Student` object to set at the specified index.
    ///
    /// # Returns
    /// `bool` - `true` if the student was successfully set, `false` if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use qrate::{SBank, SBankHelper, Student};
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push(Student::new("Alice".to_string(), "s123".to_string()));
    /// sbank.push(Student::new("Bob".to_string(), "s456".to_string()));
    ///
    /// let new_student = Student::new("Charlie".to_string(), "s789".to_string());
    /// assert!(sbank.set_student(2, new_student.clone()));
    /// assert_eq!(sbank.get_student(2).unwrap().get_name(), "Charlie");
    /// assert!(!sbank.set_student(3, new_student)); // Out of bounds
    /// ```
    fn set_student(&mut self, number: usize, student: Student) -> bool;

    // fn push_student(&mut self, student: Student)
    /// Adds a `Student` to the bank.
    ///
    /// # Arguments
    /// * `student` - The `Student` to add to the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{SBank, SBankHelper, Student};
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("Charlie".to_string(), "s789".to_string()));
    /// assert_eq!(sbank.len(), 1);
    /// assert_eq!(sbank.get_student(1).unwrap().get_name(), "Charlie");
    /// ```
    fn push_student(&mut self, student: Student);

    // fn remove_student(&mut self, number: usize) -> bool
    /// Removes a `Student` from the bank by their 1-based index.
    ///
    /// # Arguments
    /// * `number` - The 1-based index of the student to remove.
    ///
    /// # Returns
    /// `bool` - `true` if the student was successfully removed, `false` if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use qrate::{SBank, SBankHelper, Student};
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("Charlie".to_string(), "s789".to_string()));
    /// assert_eq!(sbank.len(), 1);
    /// assert!(sbank.remove_student(1));
    /// assert_eq!(sbank.len(), 0);
    /// ```
    fn remove_student(&mut self, number: usize) -> bool;

    // fn get_length(&self) -> usize
    /// Returns the number of students in the bank.
    ///
    /// # Returns
    /// The number of students in the bank.
    /// This is equivalent to the length of the underlying vector.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{SBank, SBankHelper, Student};
    /// let mut sbank = SBank::new();
    /// assert_eq!(sbank.get_length(), 0);
    /// sbank.push_student(Student::new("Dave".to_string(), "s012".to_string()));
    /// assert_eq!(sbank.get_length(), 1);
    /// ```
    fn get_length(&self) -> usize;
}

impl SBankHelper for SBank
{
    fn get_student(&self, number: usize) -> Option<Student>
    {
        if (number <= self.len()) && number > 0
            { Some(self[number - 1].clone()) }
        else
            { None }
    }

    fn set_student(&mut self, number: usize, student: Student) -> bool
    {
        if (number <= self.len()) && number > 0
        {
            self[number - 1] = student;
            true
        }
        else
        {
            false
        }
    }

    #[inline]
    fn push_student(&mut self, student: Student)
    {
        self.push(student);
    }

    fn remove_student(&mut self, number: usize) -> bool
    {
        if (number <= self.len()) && number > 0
        {
            self.remove(number - 1);
            true
        }
        else
        {
            false
        }
    }

    #[inline]
    fn get_length(&self) -> usize
    {
        self.len()
    }
}

