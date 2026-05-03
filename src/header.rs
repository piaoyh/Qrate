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
            title: "Examination".to_string(),
            name: "Name".to_string(),
            id: "ID".to_string(),
            categories: vec!["Type A".to_string(), "Type B".to_string()],
            notice: r#"Notice:
* All the questions should be considered, understood and interpreted in the context of the software engineering course you learned. Otherwise, the questions may or may not make sense.
* Type A: Multiple Choice 1 – you have to choose one answer from the list.
    # If your answer is correct, you will get 3 points.
    # If your answer is incorrect, you will lose 1 point.
    # If you choose nothing from the list, you will get 0 points.
* Type B: Multiple Choice 2 – you have to choose two answers from the list.
    # If both answers that you chose are correct, you will get 3 points.
    # If one answer you chose is correct and the other one you chose is incorrect, you will get 0 points.
    # If both answers that you chose are incorrect, you will lose 3 points.
    # If you choose one answer or nothing from the list, you will get 0 points."#.to_string(),
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
    pub fn new(title: String, name: String, id: String, categories: Vec<String>, notice: String) -> Self
    {
        Self { title, name, id, categories, notice }
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
    pub fn set_notice(&mut self, notice: String)
    {
        self.notice = notice;
    }
}