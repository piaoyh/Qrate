// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::Student;

/// A struct representing a student bank,
/// which is a collection of `Student` objects.
/// The `SBank` struct contains a version number for future compatibility checks
/// and a vector of `Student` objects that represent the students in the bank.
#[derive(Debug, Clone)]
pub struct SBank
{
    version: u32,   // The version of the student bank format. This can be used for future compatibility checks.
    students: Vec<Student>, // A vector of `Student`s.
}

impl SBank
{
    pub const VERSION: u32 = 1;

    // pub fn new() -> Self
    /// Creates a new, empty `SBank` with the default version number.
    /// The version number is set to 1 for the initial implementation,
    /// and can be used in the future to handle compatibility
    /// issues with different versions of the student bank format.
    /// 
    /// # Returns
    /// A new, empty `SBank` with the default version number.
    /// 
    /// # Examples
    /// ```
    /// use qrate::SBank;
    /// let sbank = SBank::new();
    /// assert!(sbank.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self
    {
        Self { version: Self::VERSION, students: Vec::new() }
    }

    // pub fn new_with_students(students: Vec<Student>) -> Self
    /// Creates a new `SBank` with a given vector of `Student` objects.
    /// The version number is set to 1 for the initial implementation,
    /// and can be used in the future to handle compatibility
    /// issues with different versions of the student bank format.
    /// 
    /// # Arguments
    /// * `students` - A vector of `Student` objects to initialize the bank with.
    /// 
    /// # Returns
    /// A new `SBank` initialized with the provided vector of `Student` objects.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{SBank, Student};
    /// let students = vec![
    ///     Student::new("Alice".to_string(), "s123".to_string()),
    ///     Student::new("Bob".to_string(), "s456".to_string()),
    /// ];
    /// let sbank = SBank::new_with_students(students);
    /// assert_eq!(sbank.get_length(), 2);
    /// assert_eq!(sbank.get_student(1).unwrap().get_name(), "Alice");
    /// assert_eq!(sbank.get_student(2).unwrap().get_name(), "Bob");
    /// ```
    #[inline]
    pub fn new_with_students(students: Vec<Student>) -> Self
    {
        Self { version: Self::VERSION, students }
    }

    // pub fn get_version(&self) -> u32
    /// Gets the version number of the `SBank`.
    /// 
    /// # Returns
    /// The version number of the `SBank`.
    /// This can be used for future compatibility checks when loading or saving the student bank.
    /// 
    /// # Examples
    /// ```
    /// use qrate::SBank;
    /// let sbank = SBank::new();
    /// assert_eq!(sbank.get_version(), 1);
    /// ```
    #[inline]
    pub fn get_version(&self) -> u32
    {
        self.version
    }

    // pub fn set_version(&mut self, version: u32)
    /// Sets the version number of the `SBank`.
    /// 
    /// # Arguments
    /// * `version` - The version number to set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::SBank;
    /// let mut sbank = SBank::new();
    /// sbank.set_version(2);
    /// assert_eq!(sbank.get_version(), 2);
    /// ```
    #[inline]
    pub fn set_version(&mut self, version: u32)
    {
        self.version = version;
    }

    // pub fn is_empty(&self) -> bool
    /// Checks if the `SBank` is empty, meaning it contains no students.
    /// This is determined by checking if the underlying vector of students
    /// (`self.students`) is empty.
    /// 
    /// # Returns
    /// `true` if the `SBank` is empty, `false` otherwise.
    /// 
    /// # Examples
    /// ```
    /// use qrate::SBank;
    /// let sbank = SBank::new();
    /// assert!(sbank.is_empty());
    /// sbank.push_student(Student::new("Alice".to_string(), "s123".to_string()));
    /// assert!(!sbank.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool
    {
        self.students.is_empty()
    }

    // pub fn get_student(&self, number: usize) -> Option<Student>
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
    /// use qrate::{SBank, Student};
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push(Student::new("Alice".to_string(), "s123".to_string()));
    /// sbank.push(Student::new("Bob".to_string(), "s456".to_string()));
    ///
    /// assert_eq!(sbank.get_student(1).unwrap().get_name(), "Alice");
    /// assert!(sbank.get_student(3).is_none());
    /// ```
    pub fn get_student(&self, number: usize) -> Option<Student>
    {
        if (number <= self.get_length()) && number > 0
            { Some(self.students[number - 1].clone()) }
        else
            { None }
    }

    // pub fn set_student(&mut self, number: usize, student: Student) -> bool
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
    /// use qrate::{SBank, Student};
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
    pub fn set_student(&mut self, number: usize, student: Student) -> bool
    {
        let res = (number <= self.get_length()) && number > 0;
        if res
            { self.students[number - 1] = student; }
        res
    }
    
    // pub fn push_student(&mut self, student: Student)
    /// Adds a `Student` to the bank.
    ///
    /// # Arguments
    /// * `student` - The `Student` to add to the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{SBank, Student};
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("Charlie".to_string(), "s789".to_string()));
    /// assert_eq!(sbank.len(), 1);
    /// assert_eq!(sbank.get_student(1).unwrap().get_name(), "Charlie");
    /// ```
    #[inline]
    pub fn push_student(&mut self, student: Student)
    {
        self.students.push(student);
    }

    // pub fn remove_student(&mut self, number: usize) -> bool
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
    /// use qrate::{SBank, Student};
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("Charlie".to_string(), "s789".to_string()));
    /// assert_eq!(sbank.len(), 1);
    /// assert!(sbank.remove_student(1));
    /// assert_eq!(sbank.len(), 0);
    /// ```
    pub fn remove_student(&mut self, number: usize) -> bool
    {
        let res = (number <= self.get_length()) && number > 0;
        if res
            { self.students.remove(number - 1); }
        res
    }

    // pub fn get_students(&self) -> &Vec<Student>
    /// Returns a reference to the underlying vector of `Student` objects.
    /// This allows external code to access the list of students in the bank
    /// directly, while still maintaining control over how students are added,
    /// removed, or modified through the provided methods.
    /// 
    /// # Returns
    /// A reference to the vector of `Student` objects in the bank.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{SBank, Student};
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("Alice".to_string(), "s123".to_string()));
    /// sbank.push_student(Student::new("Bob".to_string(), "s456".to_string()));
    /// let students = sbank.get_students();
    /// assert_eq!(students.len(), 2);
    /// assert_eq!(students[0].get_name(), "Alice");
    /// assert_eq!(students[1].get_name(), "Bob");
    /// ```
    #[inline]
    pub fn get_students(&self) -> &Vec<Student>
    {
        &self.students
    }

    //  pub fn set_students(&mut self, students: Vec<Student>)
    /// Sets the underlying vector of `Student` objects in the bank.
    /// This method allows external code to replace the entire list of students
    /// in the bank with a new vector of `Student` objects. It is important to
    /// note that this method will replace the existing list of students,
    /// so it should be used with caution to avoid unintended data loss.
    /// 
    /// # Arguments
    /// * `students` - A vector of `Student` objects to set as the new list of students in the bank.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{SBank, Student};
    /// let mut sbank = SBank::new();
    /// let new_students = vec![
    ///     Student::new("Alice".to_string(), "s123".to_string()),
    ///     Student::new("Bob".to_string(), "s456".to_string()),
    /// ];
    /// sbank.set_students(new_students);
    /// assert_eq!(sbank.get_students().len(), 2);
    /// assert_eq!(sbank.get_students()[0].get_name(), "Alice");
    /// assert_eq!(sbank.get_students()[1].get_name(), "Bob");
    /// ```
    #[inline]
    pub fn set_students(&mut self, students: Vec<Student>)
    {
        self.students = students;
    }

    // pub fn get_length(&self) -> usize
    /// Returns the number of students in the bank.
    ///
    /// # Returns
    /// The number of students in the bank.
    /// This is equivalent to the length of the underlying vector.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{SBank, Student};
    /// let mut sbank = SBank::new();
    /// assert_eq!(sbank.get_length(), 0);
    /// sbank.push_student(Student::new("Dave".to_string(), "s012".to_string()));
    /// assert_eq!(sbank.get_length(), 1);
    /// ```
    #[inline]
    pub fn get_length(&self) -> usize
    {
        self.students.len()
    }

    // pub fn optimize(&mut self)
    /// Optimizes the student bank by ensuring that any students that have empty
    /// names and IDs are removed from the bank.
    /// 
    /// The optimization process iterates through the students in reverse order and removes
    /// any students that have empty names and IDs. This helps to clean up the bank
    /// and ensure that it only contains valid student entries.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{SBank, Student};
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("".to_string(), "".to_string())); // Invalid student
    /// sbank.push_student(Student::new("Eve".to_string(), "s345".to_string())); // Valid student
    /// assert_eq!(sbank.get_length(), 2);
    /// sbank.optimize();
    /// assert_eq!(sbank.get_length(), 1);
    /// assert_eq!(sbank.get_student(1).unwrap().get_name(), "Eve");
    /// ```
    pub fn optimize(&mut self)
    {
        let len = self.get_length();
        for number in (1..=len).rev()
        {
            // Because number is within the reange 1..=len, .unwrap() does not cause panic.
            let student = self.get_student(number).unwrap();
            if student.get_name().is_empty() && student.get_id().is_empty()
                { self.remove_student(number); }
        }
    }
}

