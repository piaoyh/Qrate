// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


/// Represents the metadata header for a question bank.
///
/// This struct holds information like the title of the exam, author's name,
/// categories of questions, and general notices.
#[derive(Debug, Clone)]
pub struct Header
{
    version: u32,
    title: String,
    name: String,
    id: String,
    categories: Vec<String>,
    notice: String,
}

impl Header
{
    // pub fn new_empty() -> Self
    /// Creates a new, empty `Header`.
    ///
    /// # Output `Self` - A new, empty `Header` instance.
    /// 
    /// # Features
    /// All fields are initialized as empty strings or an empty vector.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_empty();
    /// assert_eq!(header.get_title(), "");
    /// ```
    pub fn new_empty() -> Self
    {
        Self
        {
            version: 1,
            title: String::new(),
            name: String::new(),
            id: String::new(),
            categories: Vec::<String>::new(),
            notice: String::new(),
        }
    }

    // pub fn new_with_default() -> Self
    /// Creates a new `Header` with default values.
    ///
    /// # Output
    /// `Self` - A new `Header` instance with default values.
    ///
    /// # Features
    /// This is useful for creating a template or a new question bank with
    /// standard instructions and categories.
    /// 
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_with_default();
    /// assert_eq!(header.get_title(), "Examination");
    /// assert!(!header.get_categories().is_empty());
    /// ```
    pub fn new_with_default() -> Self
    {
        Self
        {
            version: 1,
            title: "Examination".to_string(),
            name: "Full Name".to_string(),
            id: "Student ID Number".to_string(),
            categories: vec!["Type A".to_string(), "Type B".to_string(), "Type C".to_string(), "Type D".to_string()],
            notice: r##"Examination Guidelines:
# General Instructions
Contextual Understanding: All questions must be interpreted and answered within the specific context of the Information Security course. Failure to do so may lead to incorrect interpretations.
# Question Types & Scoring
* Type A: Single-choice objective question; select the one most appropriate answer from the given options. Assigned points are awarded for a correct answer. Selecting no answer or two or more answers results in 0 points. Selecting one wrong answer results in a deduction of `assigned points` / (number of choices - 1).
* Type B: Multiple-choice objective question; select all applicable correct answers from the given options. Assigned points are awarded if the number of selected answers equals the number of correct answers and all selected answers are correct. If the number of selected answers differs from the number of correct answers, 0 points are awarded. If the number of selected answers equals the number of correct answers, then (`number of wrong options` x `number of selected correct answers` - `number of correct options` x `number of selected wrong answers`) x `assigned points` / `number of choices` points are awarded.
* Type C: Short answer subjective question; write the appropriate word, phrase, or short expression according to the requirements.
* Type D: Essay subjective question; write a detailed and comprehensive answer to the given question."##.to_string(),
        }
    }

    // pub fn new(title: String, name: String, id: String, category: Vec<String>, notice: String) -> Self
    /// Creates a new `Header` with the given values.
    ///
    /// # Arguments
    /// * `title` - The title of the examination or document.
    /// * `name` - The name of the author or creator.
    /// * `id` - An identifier for the document or creator.
    /// * `category` - A vector of strings representing categories or types of questions.
    /// * `notice` - A string containing any important notices or instructions.
    ///
    /// # Output
    /// `Self` - A new `Header` instance with the specified values.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new(
    ///     "Math Quiz".to_string(),
    ///     "John Doe".to_string(),
    ///     "12345".to_string(),
    ///     vec!["Algebra".to_string(), "Geometry".to_string()],
    ///     "Solve carefully.".to_string(),
    /// );
    /// assert_eq!(header.get_title(), "Math Quiz");
    /// ```
    #[inline]
    pub fn new(title: String, name: String, id: String, categories: Vec<String>, notice: String) -> Self
    {
        Self { version: 1, title, name, id, categories, notice }
    }

    // pub fn get_title(&self) -> &String
    /// Gets the title from the header.
    ///
    /// # Output
    /// `&String` - A reference to the title string.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_with_default();
    /// assert_eq!(header.get_title(), "Examination");
    /// ```
    #[inline]
    pub fn get_title(&self) -> &String
    {
        &self.title
    }

    // pub fn set_title(&mut self, title: String)
    /// Sets the title in the header.
    ///
    /// # Arguments
    /// * `title` - The new title string.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let mut header = Header::new_empty();
    /// header.set_title("New Title".to_string());
    /// assert_eq!(header.get_title(), "New Title");
    /// ```
    #[inline]
    pub fn set_title(&mut self, title: String)
    {
        self.title = title;
    }

    // pub fn get_name(&self) -> &String
    /// Gets the name from the header.
    ///
    /// # Output
    /// `&String` - A reference to the name string.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_with_default();
    /// assert_eq!(header.get_name(), "Name");
    /// ```
    #[inline]
    pub fn get_name(&self) -> &String
    {
        &self.name
    }

    // pub fn set_name(&mut self, name: String)
    /// Sets the name in the header.
    ///
    /// # Arguments
    /// * `name` - The new name string.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let mut header = Header::new_empty();
    /// header.set_name("New Name".to_string());
    /// assert_eq!(header.get_name(), "New Name");
    /// ```
    #[inline]
    pub fn set_name(&mut self, name: String)
    {
        self.name = name;
    }

    // pub fn get_id(&self) -> &String
    /// Gets the ID from the header.
    ///
    /// # Output
    /// `&String` - A reference to the ID string.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_with_default();
    /// assert_eq!(header.get_id(), "ID");
    /// ```
    #[inline]
    pub fn get_id(&self) -> &String
    {
        &self.id
    }

    // pub fn set_id(&mut self, id: String)
    /// Sets the ID in the header.
    ///
    /// # Arguments
    /// * `id` - The new ID string.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let mut header = Header::new_empty();
    /// header.set_id("New ID".to_string());
    /// assert_eq!(header.get_id(), "New ID");
    /// ```
    #[inline]
    pub fn set_id(&mut self, id: String)
    {
        self.id = id;
    }

    // pub fn get_categories(&self) -> &Vec<String>
    /// Gets the vector of categories from the header.
    ///
    /// # Output
    /// `&Vec<String>` - A reference to the vector of category strings.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_with_default();
    /// assert_eq!(header.get_categories().len(), 2);
    /// ```
    #[inline]
    pub fn get_categories(&self) -> &Vec<String>
    {
        &self.categories
    }

    // pub fn get_category(&self, cat: u8) -> Option<&String>
    /// Gets a specific category by its ID.
    ///
    /// # Arguments
    /// * `cat` - The 1-based ID of the category to retrieve.
    ///
    /// # Output
    /// `Option<&String>` - An optional reference to the category string at the specified ID.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_with_default();
    /// assert_eq!(header.get_category(1), Some(&"Type A".to_string()));
    /// assert_eq!(header.get_category(100), None);
    /// ```
    pub fn get_category(&self, cat: u8) -> Option<&String>
    {
        if cat > 0 && cat <= self.categories.len() as u8
            { Some(&self.categories[cat as usize - 1]) }
        else
            { None }
    }

    // pub fn set_categories(&mut self, category: Vec<String>)
    /// Sets the entire vector of categories.
    ///
    /// # Arguments
    /// * `category` - The new vector of category strings.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let mut header = Header::new_empty();
    /// header.set_categories(vec!["Category A".to_string()]);
    /// assert_eq!(header.get_categories().len(), 1);
    /// ```
    #[inline]
    pub fn set_categories(&mut self, categories: Vec<String>)
    {
        self.categories = categories;
    }

    // pub fn push_category(&mut self, q_type: String)
    /// Adds a new category to the list.
    ///
    /// # Arguments
    /// * `q_type` - The category string to add.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let mut header = Header::new_empty();
    /// header.push_category("New Category".to_string());
    /// assert_eq!(header.get_categories().len(), 1);
    /// ```
    #[inline]
    pub fn push_category(&mut self, q_type: String)
    {
        self.categories.push(q_type);
    }

    // pub fn get_notice(&self) -> &String
    /// Gets the notice text from the header.
    ///
    /// # Output
    /// `&String` - A reference to the notice text string.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_with_default();
    /// assert!(header.get_notice().starts_with("Notice:"));
    /// ```
    #[inline]
    pub fn get_notice(&self) -> &String
    {
        &self.notice
    }

    // pub fn set_notice(&mut self, notice: String)
    /// Sets the notice text in the header.
    ///
    /// # Arguments
    /// * `notice` - The new notice text string.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let mut header = Header::new_empty();
    /// header.set_notice("Important information.".to_string());
    /// assert_eq!(header.get_notice(), "Important information.");
    /// ```
    #[inline]
    pub fn set_notice(&mut self, notice: String)
    {
        self.notice = notice;
    }

    // pub fn get_version(&self) -> u32
    /// Gets the version number from the header.
    ///
    /// # Output
    /// `u32` - The version number.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let header = Header::new_with_default();
    /// assert_eq!(header.get_version(), 1);
    /// ```
    #[inline]
    pub fn get_version(&self) -> u32
    {
        self.version
    }

    // pub fn set_version(&mut self, version: u32)
    /// Sets the version number in the header.
    ///
    /// # Arguments
    /// * `version` - The new version number.
    ///
    /// # Examples
    /// ```
    /// use qrate::Header;
    /// let mut header = Header::new_empty();
    /// header.set_version(2);
    /// assert_eq!(header.get_version(), 2);
    /// ```
    #[inline]
    pub fn set_version(&mut self, version: u32)
    {
        self.version = version;
    }
}