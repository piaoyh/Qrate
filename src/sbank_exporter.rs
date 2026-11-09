// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::io::Cursor;

use docx_rs::{ Docx, Paragraph, Run, PageMargin, AlignmentType,
                Footer, InstrText, InstrPAGE, InstrNUMPAGES, FieldCharType };

use crate::{ Generator, SBank };

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::fs::File;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::io::Write;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::path::Path;


            #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            use hwpers::{ HwpWriter, HwpxWriter };
            #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            use hwpers::style::TextStyle;
            #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            use hwpers::hwpx::{ HwpxTextStyle, StyledText };
            #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            use hwpers::model::PageNumberFormat::Numeric;


#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use genpdfi::error::Error;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use genpdfi::{ Document, elements, fonts, style, Element, SimplePageDecorator, Alignment };

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use crate::check_path;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use serde_json::json;


pub struct SBankExporter
{
    sbank: SBank,
    current_student_number: u16,
    body_font_size: f32,
    title_font_size: f32,
    footer_font_size: f32,
    attributes: u16,
    margin_left_in_mm: f32,
    margin_right_in_mm: f32,
    margin_top_in_mm: f32,
    margin_buttom_in_mm: f32,
    line_spacing: f32,
    student_list_title: String,
}

impl SBankExporter
{
    // pub new(sbank: SBank, student_list_title: String) -> Self
    /// Creates a new `SBankExporter` instance for exporting student bank.
    ///
    /// # Arguments
    /// * `sbank` - A reference to the `SBank` containing the student list.
    /// * `student_list_title` - The title for the student list.
    ///
    /// # Returns
    /// A `Self` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// ```
    pub fn new(sbank: SBank, student_list_title: String) -> Self
    {
        Self
        {
            sbank,
            current_student_number: 0,
            title_font_size: Generator::DEFAULT_TITLE_FONT_SIZE,
            body_font_size: Generator::DEFAULT_BODY_FONT_SIZE,
            footer_font_size: Generator::DEFAULT_FOOTER_FONT_SIZE,
            attributes: Generator::DEFAULT_ATTRIBUTES,
            margin_left_in_mm: Generator::DEFAULT_MARGIN_LEFT_IN_MM,
            margin_right_in_mm: Generator::DEFAULT_MARGIN_RIGHT_IN_MM,
            margin_top_in_mm: Generator::DEFAULT_MARGIN_TOP_IN_MM,
            margin_buttom_in_mm: Generator::DEFAULT_MARGIN_BUTTOM_IN_MM,
            line_spacing: Generator::DEFAULT_LINE_SPACING,
            student_list_title
        }
    }

    // pub fn get_title_font_size(&self) -> f32
    /// Retrieves the current title font size in points.
    ///
    /// # Returns
    /// `f32` - The current font size used for titles.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let font_size = exporter.get_title_font_size();
    /// assert_eq!(font_size, 14.0);
    /// ```
    #[inline]
    pub fn get_title_font_size(&self) -> f32
    {
        self.title_font_size
    }
    
    // pub fn set_title_font_size(&mut self, title_font_size: f32)
    /// Sets the title font siz in points.
    ///
    /// # Arguments
    /// * `title_font_size` - The new font size to be used for titles.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_title_font_size(16.0);
    /// assert_eq!(exporter.get_title_font_size(), 16.0);
    /// ```
    #[inline]
    pub fn set_title_font_size(&mut self, title_font_size: f32)
    {
        self.title_font_size = title_font_size;
    }
    
    // pub fn get_body_font_size(&self) -> f32
    /// Retrieves the current default font size in points.
    ///
    /// # Returns
    /// `f32` - The current default font size.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let font_size = exporter.get_body_font_size();
    /// assert_eq!(font_size, 11.0);
    /// ```
    #[inline]
    pub fn get_body_font_size(&self) -> f32
    {
        self.body_font_size
    }
    
    // pub fn set_body_font_size(&mut self, body_font_size: f32)
    /// Sets the default font size in points.
    ///
    /// # Arguments
    /// * `body_font_size` - The new default font size.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_body_font_size(12.0);
    /// assert_eq!(exporter.get_body_font_size(), 12.0);
    /// ```
    #[inline]
    pub fn set_body_font_size(&mut self, body_font_size: f32)
    {
        self.body_font_size = body_font_size
    }
    
    // pub fn get_footer_font_size(&self) -> f32
    /// Retrieves the current footer font size in points.
    ///
    /// # Returns
    /// `f32` - The current font size used for footers.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let font_size = exporter.get_footer_font_size();
    /// assert_eq!(font_size, 9.0);
    /// ```
    #[inline]
    pub fn get_footer_font_size(&self) -> f32
    {
        self.footer_font_size
    }
    
    // pub fn set_footer_font_size(&mut self, footer_font_size: f32)
    /// Sets the footer font size in points.
    ///
    /// # Arguments
    /// * `footer_font_size` - The new font size to be used for footers.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_footer_font_size(10.0);
    /// assert_eq!(exporter.get_footer_font_size(), 10.0);
    /// ```
    #[inline]
    pub fn set_footer_font_size(&mut self, footer_font_size: f32)
    {
        self.footer_font_size = footer_font_size
    }
    
    // // pub fn get_student_list_font_size(&self) -> f32
    // /// Retrieves the current answer sheet font size in points.
    // ///
    // /// # Returns
    // /// `f32` - The current font size used for the answer sheet.
    // ///
    // /// # Examples
    // /// ```
    // /// use qrate::{ SBank, SBankExporter };
    // ///
    // /// let mut sbank = SBank::new();
    // /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    // /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    // /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    // /// 
    // /// // Exports a student bank with 3 students
    // /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    // /// let font_size = exporter.get_student_list_font_size();
    // /// assert_eq!(font_size, 12.0);
    // /// ```
    // #[inline]
    // pub fn get_student_list_font_size(&self) -> f32
    // {
    //     self.student_list_font_size
    // }
    
    // // pub fn set_student_list_font_size(&mut self, student_list_font_size: f32)
    // /// Sets the answer sheet font size in points.
    // ///
    // /// # Arguments
    // /// * `student_list_font_size` - The new font size to be used for the answer sheet.
    // ///
    // /// # Examples
    // /// ```
    // /// use qrate::{ SBank, SBankExporter };
    // ///
    // /// let mut sbank = SBank::new();
    // /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    // /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    // /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    // /// 
    // /// // Exports a student bank with 3 students
    // /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    // /// exporter.set_student_list_font_size(13.0);
    // /// assert_eq!(exporter.get_student_list_font_size(), 13.0);
    // /// ```
    // #[inline]
    // pub fn set_student_list_font_size(&mut self, student_list_font_size: f32)
    // {
    //     self.student_list_font_size = student_list_font_size;
    // }

    // pub fn is_body_bold(&self) -> bool
    /// Checks if the body text is set to bold.
    ///
    /// # Returns
    /// `bool` - `true` if the body text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_body_bold());
    /// exporter.set_body_bold(true);
    /// assert!(exporter.is_body_bold());
    /// ```
    #[inline]
    pub fn is_body_bold(&self) -> bool
    {
        (self.attributes & Generator::BODY_BOLD) == Generator::BODY_BOLD
    }

    // pub fn set_body_bold(&mut self, on: bool)
    /// Sets or unsets the bold attribute for the body text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the body text to bold, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_body_bold(true);
    /// assert!(exporter.is_body_bold());
    /// exporter.set_body_bold(false);
    /// assert!(!exporter.is_body_bold());
    /// ```
    #[inline]
    pub fn set_body_bold(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::BODY_BOLD; }
        else
            { self.attributes &= !Generator::BODY_BOLD; }
    }

    // pub fn is_body_italic(&self) -> bool
    /// Checks if the body text is set to italic.
    ///
    /// # Returns
    /// `bool` - `true` if the body text is italic, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_body_italic());
    /// exporter.set_body_italic(true);
    /// assert!(exporter.is_body_italic());
    /// ```
    #[inline]
    pub fn is_body_italic(&self) -> bool
    {
        (self.attributes & Generator::BODY_ITALIC) == Generator::BODY_ITALIC
    }

    // pub fn set_body_italic(&mut self, on: bool)
    /// Sets or unsets the italic attribute for the body text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the body text to italic, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_body_italic(true);
    /// assert!(exporter.is_body_italic());
    /// exporter.set_body_italic(false);
    /// assert!(!exporter.is_body_italic());
    /// ```
    #[inline]
    pub fn set_body_italic(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::BODY_ITALIC; }
        else
            { self.attributes &= !Generator::BODY_ITALIC; }
    }

    // pub fn is_body_underline(&self) -> bool
    /// Checks if the body text is set to underline.
    ///
    /// # Returns
    /// `bool` - `true` if the body text is underlined, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_body_underline());
    /// exporter.set_body_underline(true);
    /// assert!(exporter.is_body_underline());
    /// ```
    #[inline]
    pub fn is_body_underline(&self) -> bool
    {
        (self.attributes & Generator::BODY_UNDERLINE) == Generator::BODY_UNDERLINE
    }

    // pub fn set_body_underline(&mut self, on: bool)
    /// Sets or unsets the underline attribute for the body text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the body text to underline, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_body_underline(true);
    /// assert!(exporter.is_body_underline());
    /// exporter.set_body_underline(false);
    /// assert!(!exporter.is_body_underline());
    /// ```
    #[inline]
    pub fn set_body_underline(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::BODY_UNDERLINE; }
        else
            { self.attributes &= !Generator::BODY_UNDERLINE; }
    }

    // pub fn is_body_strike(&self) -> bool
    /// Checks if the body text is set to strikethrough.
    ///
    /// # Returns
    /// `bool` - `true` if the body text is strikethrough, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_body_strike());
    /// exporter.set_body_strike(true);
    /// assert!(exporter.is_body_strike());
    /// ```
    #[inline]
    pub fn is_body_strike(&self) -> bool
    {
        (self.attributes & Generator::BODY_STRIKE) == Generator::BODY_STRIKE
    }

    // pub fn set_body_strike(&mut self, on: bool)
    /// Sets or unsets the strikethrough attribute for the body text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the body text to strikethrough, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_body_strike(true);
    /// assert!(exporter.is_body_strike());
    /// exporter.set_body_strike(false);
    /// assert!(!exporter.is_body_strike());
    /// ```
    #[inline]
    pub fn set_body_strike(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::BODY_STRIKE; }
        else
            { self.attributes &= !Generator::BODY_STRIKE; }
    }


    // pub fn is_title_bold(&self) -> bool
    /// Checks if the title text is set to bold.
    ///
    /// # Returns
    /// `bool` - `true` if the title text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(exporter.is_title_bold()); // Default is true
    /// exporter.set_title_bold(false);
    /// assert!(!exporter.is_title_bold());
    /// ```
    #[inline]
    pub fn is_title_bold(&self) -> bool
    {
        (self.attributes & Generator::TITLE_BOLD) == Generator::TITLE_BOLD
    }

    // pub fn set_title_bold(&mut self, on: bool)
    /// Sets or unsets the bold attribute for the title text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the title text to bold, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_title_bold(false); // Default is true, so unset it
    /// assert!(!exporter.is_title_bold());
    /// exporter.set_title_bold(true);
    /// assert!(exporter.is_title_bold());
    /// ```
    #[inline]
    pub fn set_title_bold(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::TITLE_BOLD; }
        else
            { self.attributes &= !Generator::TITLE_BOLD; }
    }

    // pub fn is_title_italic(&self) -> bool
    /// Checks if the title text is set to italic.
    ///
    /// # Returns
    /// `bool` - `true` if the title text is italic, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_title_italic());
    /// exporter.set_title_italic(true);
    /// assert!(exporter.is_title_italic());
    /// ```
    #[inline]
    pub fn is_title_italic(&self) -> bool
    {
        (self.attributes & Generator::TITLE_ITALIC) == Generator::TITLE_ITALIC
    }

    // pub fn set_title_italic(&mut self, on: bool)
    /// Sets or unsets the italic attribute for the title text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the title text to italic, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_title_italic(true);
    /// assert!(exporter.is_title_italic());
    /// exporter.set_title_italic(false);
    /// assert!(!exporter.is_title_italic());
    /// ```
    #[inline]
    pub fn set_title_italic(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::TITLE_ITALIC; }
        else
            { self.attributes &= !Generator::TITLE_ITALIC; }
    }

    // pub fn is_title_underline(&self) -> bool
    /// Checks if the title text is set to underline.
    ///
    /// # Returns
    /// `bool` - `true` if the title text is underlined, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_title_underline());
    /// exporter.set_title_underline(true);
    /// assert!(exporter.is_title_underline());
    /// ```
    #[inline]
    pub fn is_title_underline(&self) -> bool
    {
        (self.attributes & Generator::TITLE_UNDERLINE) == Generator::TITLE_UNDERLINE
    }

    // pub fn set_title_underline(&mut self, on: bool)
    /// Sets or unsets the underline attribute for the title text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the title text to underline, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_title_underline(true);
    /// assert!(exporter.is_title_underline());
    /// exporter.set_title_underline(false);
    /// assert!(!exporter.is_title_underline());
    /// ```
    #[inline]
    pub fn set_title_underline(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::TITLE_UNDERLINE; }
        else
            { self.attributes &= !Generator::TITLE_UNDERLINE; }
    }

    // pub fn is_title_strike(&self) -> bool
    /// Checks if the title text is set to strikethrough.
    ///
    /// # Returns
    /// `bool` - `true` if the title text is strikethrough, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_title_strike());
    /// exporter.set_title_strike(true);
    /// assert!(exporter.is_title_strike());
    /// ```
    #[inline]
    pub fn is_title_strike(&self) -> bool
    {
        (self.attributes & Generator::TITLE_STRIKE) == Generator::TITLE_STRIKE
    }

    // pub fn set_title_strike(&mut self, on: bool)
    /// Sets or unsets the strikethrough attribute for the title text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the title text to strikethrough, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_title_strike(true);
    /// assert!(exporter.is_title_strike());
    /// exporter.set_title_strike(false);
    /// assert!(!exporter.is_title_strike());
    /// ```
    #[inline]
    pub fn set_title_strike(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::TITLE_STRIKE; }
        else
            { self.attributes &= !Generator::TITLE_STRIKE; }
    }

    // pub fn is_footer_bold(&self) -> bool
    /// Checks if the footer text is set to bold.
    ///
    /// # Returns
    /// `bool` - `true` if the footer text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_footer_bold());
    /// exporter.set_footer_bold(true);
    /// assert!(exporter.is_footer_bold());
    /// ```
    #[inline]
    pub fn is_footer_bold(&self) -> bool
    {
        (self.attributes & Generator::FOOTER_BOLD) == Generator::FOOTER_BOLD
    }

    // pub fn set_footer_bold(&mut self, on: bool)
    /// Sets or unsets the bold attribute for the footer text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the footer text to bold, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_footer_bold(true);
    /// assert!(exporter.is_footer_bold());
    /// exporter.set_footer_bold(false);
    /// assert!(!exporter.is_footer_bold());
    /// ```
    #[inline]
    pub fn set_footer_bold(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::FOOTER_BOLD; }
        else
            { self.attributes &= !Generator::FOOTER_BOLD; }
    }

    // pub fn is_footer_italic(&self) -> bool
    /// Checks if the footer text is set to italic.
    ///
    /// # Returns
    /// `bool` - `true` if the footer text is italic, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_footer_italic());
    /// exporter.set_footer_italic(true);
    /// assert!(exporter.is_footer_italic());
    /// ```
    #[inline]
    pub fn is_footer_italic(&self) -> bool
    {
        (self.attributes & Generator::FOOTER_ITALIC) == Generator::FOOTER_ITALIC
    }

    // pub fn set_footer_italic(&mut self, on: bool)
    /// Sets or unsets the italic attribute for the footer text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the footer text to italic, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_footer_italic(true);
    /// assert!(exporter.is_footer_italic());
    /// exporter.set_footer_italic(false);
    /// assert!(!exporter.is_footer_italic());
    /// ```
    #[inline]
    pub fn set_footer_italic(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::FOOTER_ITALIC; }
        else
            { self.attributes &= !Generator::FOOTER_ITALIC; }
    }

    // pub fn is_footer_underline(&self) -> bool
    /// Checks if the footer text is set to underline.
    ///
    /// # Returns
    /// `bool` - `true` if the footer text is underlined, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_footer_underline(true);
    /// assert!(exporter.is_footer_underline());
    /// ```
    #[inline]
    pub fn is_footer_underline(&self) -> bool
    {
        (self.attributes & Generator::FOOTER_UNDERLINE) == Generator::FOOTER_UNDERLINE
    }

    // pub fn set_footer_underline(&mut self, on: bool)
    /// Sets or unsets the underline attribute for the footer text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the footer text to underline, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_footer_underline(true);
    /// assert!(exporter.is_footer_underline());
    /// exporter.set_footer_underline(false);
    /// assert!(!exporter.is_footer_underline());
    /// ```
    #[inline]
    pub fn set_footer_underline(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::FOOTER_UNDERLINE; }
        else
            { self.attributes &= !Generator::FOOTER_UNDERLINE; }
    }

    // pub fn is_footer_strike(&self) -> bool
    /// Checks if the footer text is set to strikethrough.
    ///
    /// # Returns
    /// `bool` - `true` if the footer text is strikethrough, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert!(!exporter.is_footer_strike());
    /// exporter.set_footer_strike(true);
    /// assert!(exporter.is_footer_strike());
    /// ```
    #[inline]
    pub fn is_footer_strike(&self) -> bool
    {
        (self.attributes & Generator::FOOTER_STRIKE) == Generator::FOOTER_STRIKE
    }

    // pub fn set_footer_strike(&mut self, on: bool)
    /// Sets or unsets the strikethrough attribute for the footer text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the footer text to strikethrough, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_footer_strike(true);
    /// assert!(exporter.is_footer_strike());
    /// exporter.set_footer_strike(false);
    /// assert!(!exporter.is_footer_strike());
    /// ```
    #[inline]
    pub fn set_footer_strike(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::FOOTER_STRIKE; }
        else
            { self.attributes &= !Generator::FOOTER_STRIKE; }
    }

    // pub fn get_attributes(&self) -> u16
    /// Retrieves the current attribute bitmask.
    ///
    /// # Returns
    /// `u16` - The current attribute bitmask.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let attributes = exporter.get_attributes();
    /// // Example: Check if the default title is bold
    /// // const TITLE_BOLD: u16 = 0b_1_0000;
    /// // assert_eq!(attributes & TITLE_BOLD, TITLE_BOLD);
    /// ```
    #[inline]
    pub fn get_attributes(&self) -> u16
    {
        self.attributes
    }

    // pub fn set_attributes(&mut self, attr: u16)
    /// Sets the specified attributes by performing a bitwise OR operation.
    ///
    /// This method allows combining multiple attributes (e.g., bold, italic)
    /// by passing a bitmask.
    ///
    /// # Arguments
    /// * `attr` - A `u16` bitmask representing the attributes to set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// // For example purposes, let's assume direct access or similar constants
    /// const BODY_BOLD: u16 = 0b_1;
    /// const BODY_ITALIC: u16 = 0b_10;
    /// exporter.set_attributes(BODY_BOLD | BODY_ITALIC);
    /// // assert_eq!(exporter.get_attributes() & (BODY_BOLD | BODY_ITALIC), BODY_BOLD | BODY_ITALIC);
    /// ```
    #[inline]
    pub fn set_attributes(&mut self, attr: u16)
    {
        self.attributes |= attr;
    }

    // pub fn reset_attributes(&mut self, attr: u16)
    /// Resets the specified attributes by performing a bitwise AND NOT operation.
    ///
    /// This method allows unsetting multiple attributes (e.g., bold, italic)
    /// by passing a bitmask.
    ///
    /// # Arguments
    /// * `attr` - A `u16` bitmask representing the attributes to reset.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// // For example purposes, let's assume direct access or similar constants
    /// const BODY_BOLD: u16 = 0b_1;
    /// const BODY_ITALIC: u16 = 0b_10;
    /// exporter.set_attributes(BODY_BOLD | BODY_ITALIC);
    /// // At this point, BODY_BOLD and BODY_ITALIC are set.
    /// exporter.reset_attributes(BODY_ITALIC);
    /// // Now only BODY_BOLD should be set.
    /// // assert_eq!(exporter.get_attributes() & BODY_BOLD, BODY_BOLD);
    /// // assert_eq!(exporter.get_attributes() & BODY_ITALIC, 0);
    /// ```
    #[inline]
    pub fn reset_attributes(&mut self, attr: u16)
    {
        self.attributes &= !attr;
    }
    
    // pub fn get_margin_left_in_mm(&self) -> f32
    /// Retrieves the current left margin in millimeters.
    ///
    /// # Returns
    /// `f32` - The current left margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let margin = exporter.get_margin_left_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_left_in_mm(&self) -> f32
    {
        self.margin_left_in_mm
    }
    
    // pub fn set_margin_left_in_mm(&mut self, margin_left_in_mm: f32)
    /// Sets the left margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_left_in_mm` - The new left margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_margin_left_in_mm(15.0);
    /// assert_eq!(exporter.get_margin_left_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_left_in_mm(&mut self, margin_left_in_mm: f32)
    {
        self.margin_left_in_mm = margin_left_in_mm;
    }
    
    // pub fn get_margin_right_in_mm(&self) -> f32
    /// Retrieves the current right margin in millimeters.
    ///
    /// # Returns
    /// `f32` - The current right margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let margin = exporter.get_margin_right_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_right_in_mm(&self) -> f32
    {
        self.margin_right_in_mm
    }
    
    // pub fn set_margin_right_in_mm(&mut self, margin_right_in_mm: f32)
    /// Sets the right margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_right_in_mm` - The new right margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_margin_right_in_mm(15.0);
    /// assert_eq!(exporter.get_margin_right_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_right_in_mm(&mut self, margin_right_in_mm: f32)
    {
        self.margin_right_in_mm = margin_right_in_mm;
    }
    
    // pub fn get_margin_top_in_mm(&self) -> f32
    /// Retrieves the current top margin in millimeters.
    ///
    /// # Returns
    /// `f32` - The current top margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let margin = exporter.get_margin_top_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_top_in_mm(&self) -> f32
    {
        self.margin_top_in_mm
    }
    
    // pub fn set_margin_top_in_mm(&mut self, margin_top_in_mm: f32)
    /// Sets the top margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_top_in_mm` - The new top margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_margin_top_in_mm(15.0);
    /// assert_eq!(exporter.get_margin_top_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_top_in_mm(&mut self, margin_top_in_mm: f32)
    {
        self.margin_top_in_mm = margin_top_in_mm;
    }
    
    // pub fn get_margin_buttom_in_mm(&self) -> f32
    /// Retrieves the current bottom margin in millimeters.
    ///
    /// # Returns
    /// `f32` - The current bottom margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let margin = exporter.get_margin_buttom_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_buttom_in_mm(&self) -> f32
    {
        self.margin_buttom_in_mm
    }
    
    // pub fn set_margin_buttom_in_mm(&mut self, margin_buttom_in_mm: f32)
    /// Sets the bottom margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_buttom_in_mm` - The new bottom margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_margin_buttom_in_mm(15.0);
    /// assert_eq!(exporter.get_margin_buttom_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_buttom_in_mm(&mut self, margin_buttom_in_mm: f32)
    {
        self.margin_buttom_in_mm = margin_buttom_in_mm;
    }
    
    // pub fn get_line_spacing(&self) -> f32
    /// Retrieves the current line spacing in lines.
    ///
    /// # Returns
    /// `f32` - The current line spacing value.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let spacing = exporter.get_line_spacing();
    /// assert_eq!(spacing, 1.0);
    /// ```
    #[inline]
    pub fn get_line_spacing(&self) -> f32
    {
        self.line_spacing
    }
    
    // pub fn set_line_spacing(&mut self, line_spacing: f32)
    /// Sets the line spacing in lines.
    ///
    /// # Arguments
    /// * `line_spacing` - The new line spacing value.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// exporter.set_line_spacing(1.5);
    /// assert_eq!(exporter.get_line_spacing(), 1.5);
    /// ```
    #[inline]
    pub fn set_line_spacing(&mut self, line_spacing: f32)
    {
        self.line_spacing = line_spacing;
    }

    // pub fn next(&mut self) -> Option<(String, String)>
    /// Advances to the next student in the Student bank
    /// and returns its details.
    ///
    /// This function acts as an iterator for the student bank. Each
    /// call increments the internal student counter and provides the details
    /// of the next student, including name and ID.
    ///
    /// It is primarily used for exporting scenarios.
    ///
    /// # Returns
    /// `Option<(String, String)>` - An `Option` containing
    /// a tuple with:
    ///   - `String`: The name string of the current student.
    ///   - `String`: The ID string of the current student.
    ///
    /// Returns `None` if there are no more students in the set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    ///
    /// if let Some((name, id)) = exporter.next()
    ///     { assert_eq!(name, "홍길동"); }
    ///
    /// if let Some((name, id)) = exporter.next()
    ///     { assert_eq!(id, 2); }
    ///
    /// if let Some((name, id)) = exporter.next()
    ///     { assert_eq!(name, "Иванов Иван Иванович"); }
    /// 
    /// assert!(exporter.next().is_none());
    /// ```
    pub fn next(&mut self) -> Option<(String, String)>
    {
        self.current_student_number += 1;
        self.get_student_by_number(self.current_student_number)
    }

    // pub fn prev(&mut self) -> Option<(String, String)>
    /// Withdraws to the previous student in the student bank
    /// and returns its details.
    ///
    /// This function acts as an iterator for the student bank. Each
    /// call decrements the internal student counter and provides the details
    /// of the previous student, including name and ID.
    ///
    /// It is primarily used for exporting scenarios.
    ///
    /// # Returns
    /// `Option<(String, String)>` - An `Option` containing
    /// a tuple with:
    ///   - `String`: The name string of the current student.
    ///   - `String`: The ID string of the current student.
    ///
    /// Returns `None` if there are no more students in the set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    ///
    /// if let Some((name, id)) = exporter.next()
    ///     { assert_eq!(name, "홍길동"); }
    ///
    /// if let Some((name, id)) = exporter.next()
    ///     { assert_eq!(id, 2); }
    ///
    /// if let Some((name, id)) = exporter.prev()
    ///     { assert_eq!(name, "홍길동"); }
    /// 
    /// assert!(exporter.prev().is_none());
    /// ```
    pub fn prev(&mut self) -> Option<(String, String)>
    {
        self.current_student_number -= 1;
        self.get_student_by_number(self.current_student_number)
    }

    // pub fn get_student_by_number(&self, num: u16) -> Option<(String, String)>
    /// Retrieves a specific student by its number within the student bank.
    /// 
    /// # Arguments
    /// * `num` - The number of the student to retrieve.
    /// 
    /// # Returns
    /// `Option<(String, String)>` - An `Option` containing
    /// a tuple with:
    ///   - `String`: The name string of the current student.
    ///   - `String`: The ID string of the current student.
    /// 
    /// Returns `None` if the student with the given number is not found.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    /// 
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// 
    /// if let Some((name, id)) = exporter.get_student_by_number(3)
    ///     { assert_eq!(name, "Иванов Иван Иванович"); }
    /// 
    /// if let Some((name, id)) = exporter.get_student_by_number(1)
    ///     { assert_eq!(name, "홍길동"); }
    /// 
    /// assert!(exporter.get_student_by_number(4).is_none());
    /// ```
    pub fn get_student_by_number(&mut self, student_number: u16) -> Option<(String, String)>
    {
        self.current_student_number = student_number;
        if student_number as usize > self.sbank.get_length() || student_number == 0
            { return None; }

        let student = self.sbank.get_student(student_number)?;
        let name = student.get_name();
        let id = student.get_id();
        Some((name, id))
    }

    // pub fn get_number_of_students(&self) -> usize
    /// Returns the total number of shuffled students for the first set (self-study).
    /// 
    /// # Returns
    /// `usize` - The total number of shuffled students for the first set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    /// 
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert_eq!(exporter.get_number_of_students(), 3);
    /// ```
    #[inline]
    pub fn get_number_of_students(&self) -> usize
    {
        self.sbank.get_length()
    }

    // pub fn get_current_student_number(&self) -> u16
    /// Retrieves the current student number within the shuffled set.
    /// 
    /// # Returns
    /// `u16` - The current student number within the shuffled set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    /// 
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert_eq!(exporter.get_current_student_number(), 0);
    /// 
    /// if let Some((name, id)) = exporter.next()
    ///     { assert_eq!(name, "홍길동"); }
    /// assert_eq!(exporter.get_current_student_number(), 1);
    /// 
    /// if let Some((name, id)) = exporter.next()
    ///     { assert_eq!(name, "John Doe"); }
    /// assert_eq!(exporter.get_current_student_number(), 2);
    /// assert!(exporter.next().is_none());
    /// ```
    #[inline]
    pub fn get_current_student_number(&self) -> u16
    {
        self.current_student_number
    }

    // pub fn set_current_student_number(&mut self, num: u16)
    /// Sets the current student number within the shuffled set.
    /// 
    /// # Arguments
    /// * `num` - The new student number to set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    /// 
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// assert_eq!(exporter.get_current_student_number(), 0);
    /// 
    /// exporter.set_current_student_number(3);
    /// assert_eq!(exporter.get_current_student_number(), 3);
    /// ```
    #[inline]
    pub fn set_current_student_number(&mut self, num: u16)
    {
        self.current_student_number = num;
    }

    // pub fn save_sbank(&self, path: String, extention: &str) -> Result<(), String>
    /// Saves the student bank to a single file.
    ///
    /// The output format is determined by the file extension of the provided path.
    /// Supported formats are: .txt, .docx, and .pdf.
    /// This function delegates the actual saving process to format-specific private functions.
    ///
    /// # Arguments
    /// * `path` - The file path where the exams will be saved.
    /// * `extention` - The desired file extension (e.g., "txt", "docx", "pdf").
    ///
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter };
    /// use std::fs; // For std::fs::remove_file
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let mut result = exporter.save_sbank("sbank.txt".to_string(), "txt");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("sbank.txt").unwrap();
    ///
    /// result = exporter.save_sbank("sbank.docx".to_string(), "docx");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("sbank.docx").unwrap();
    ///
    /// result = exporter.save_sbank("sbank.hwpx".to_string(), "hwpx");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("sbank.hwpx").unwrap();
    ///
    /// result = exporter.save_sbank("sbank.hwp".to_string(), "hwp");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("sbank.hwp").unwrap();
    ///
    /// result = exporter.save_sbank("sbank.pdf".to_string(), "pdf");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("sbank.pdf").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_sbank(&self, path: String, extention: &str) -> Result<(), String>
    {
        let checked = check_path(path, extention);
        let file_path = Path::new(&checked);
        match file_path.extension().and_then(|s| s.to_str())
        {
            Some("txt") => self.save_sbank_in_txt(file_path),
            Some("docx") => self.save_sbank_in_docx(file_path),
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            Some("hwpx") => self.save_sbank_in_hwpx(file_path),
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            Some("hwp") => self.save_sbank_in_hwp(file_path),
            Some("pdf") => self.save_sbank_in_pdf(file_path),
            _ => Err("Unsupported file format. Please use .txt, .docx, hwpx, hwp or .pdf.".to_string()),
        }
    }

    // pub fn save_sbank_in_txt(&self, path: &Path) -> Result<(), String>
    /// Saves the student bank to a text file.
    ///
    /// # Arguments
    /// * `path` - The file path where the text document will be saved.
    ///
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    ///
    /// let result = exporter.save_sbank_in_txt(Path::new("student_bank.txt"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("student_bank.txt").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_sbank_in_txt(&self, path: &Path) -> Result<(), String>
    {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        let content = self.write_students_to_txt();
        writeln!(file, "{}", content).map_err(|e| e.to_string())?;

        Ok(())
    }

    // pub fn export_sbank_in_txt(&self) -> Vec<u8>
    /// Exports the student bank to a Vec<u8> object.
    ///
    /// This function generates a Vec<u8> object containing the student bank
    /// in text format.
    ///
    /// # Returns
    /// `Vec<u8>` that contains the contents of String object.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    /// 
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let content_bytes = exporter.export_sbank_in_txt();
    /// assert!(!content_bytes.is_empty());
    /// 
    /// let content_str = String::from_utf8(content_bytes).unwrap();
    /// assert!(content_str.contains("홍길동"));
    /// assert!(content_str.contains("John Doe"));
    /// assert!(content_str.contains("Иванов Иван Иванович"));
    /// ```
    pub fn export_sbank_in_txt(&self) -> Vec<u8>
    {
        let content = self.write_students_to_txt();
        content.into_bytes()
    }

    // fn write_students_to_txt(&self) -> String
    /// Writes the students of the student bank content
    /// into a human-readable string.
    ///
    /// This private helper function generates the textual representation of a
    /// student bank. It includes the student's name, the exam title, and all
    /// students with their choices.
    ///
    /// # Returns
    /// A `String` containing the fully formatted student bank content.
    fn write_students_to_txt(&self) -> String
    {
        let mut content = self.student_list_title.clone();
        content.push_str("\n\n");
        for student in self.sbank.get_students()
            { content.push_str(&format!("{}    {}\n", student.get_name(), student.get_id())); }
        content
    }

    // pub fn save_sbank_in_docx(&self, path: &Path) -> Result<(), String>
    /// Saves the student bank to a DOCX file.
    ///
    /// This function generates a DOCX document containing the student bank,
    /// applying specified page margins and a footer with page numbers.
    ///
    /// # Arguments
    /// * `path` - The file path where the DOCX document will be saved.
    ///
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    ///
    /// let result = exporter.save_sbank_in_docx(Path::new("student_bank.docx"), true);
    /// assert!(result.is_ok());
    /// std::fs::remove_file("student_bank.docx").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_sbank_in_docx(&self, path: &Path) -> Result<(), String>
    {
        let file = File::create(path).map_err(|e| e.to_string())?;
        self.build_sbank_in_docx()?
            .build()
            .pack(file)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // pub fn export_sbank_in_docx(&self) -> Result<Vec<u8>, String>
    /// Exports the sstudent bank to a Vec<u8> object.
    ///
    /// This function generates a DOCX document containing the student bank,
    /// applying specified page margins and a footer with page numbers.
    /// The resulting DOCX content is returned as a `Vec<u8>`, which can be used
    /// for in-memory operations or further processing without writing to disk.
    /// 
    /// # Returns
    /// `Result<Vec<u8>, String>` - Returns `Ok(Vec<u8>)` which is binary data
    /// on success, or an `Err` with a `String` describing the error on failure.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ SBank, SBankExporter };
    /// 
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let docx_bytes = exporter.export_sbank_in_docx().unwrap();
    /// assert!(!docx_bytes.is_empty());
    /// // Further processing of docx_bytes can be done here, such as saving to a file or sending over a network.
    /// ```
    pub fn export_sbank_in_docx(&self) -> Result<Vec<u8>, String>
    {
        let mut buffer = Cursor::new(Vec::new());
        self.build_sbank_in_docx()?
            .build()
            .pack(&mut buffer)
            .map_err(|e| e.to_string())?;
        Ok(buffer.into_inner())
    }

    fn build_sbank_in_docx(&self) -> Result<Docx, String>
    {
        let mut docx = self.write_students_to_docx()?;
        docx = docx.add_paragraph(Paragraph::new()); // Blank line

        Ok(docx)
    }

    // fn write_students_to_docx(&self) -> Result<Docx, String>
    /// Writes the formatted student bank content to a DOCX document.
    ///
    /// This private helper function takes a DOCX `Docx` object and appends
    /// the student bank contents, returning the modified `Docx` object.
    ///
    /// # Returns
    /// `Result<Docx, String>` - Returns `Ok(Docx)` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.write_students_to_docx(Docx::new());
    /// ```
    /// This example demonstrates how the `write_students_to_docx` function
    /// can be called to write sbank content into a DOCX document.
    /// The actual content of the students and formatting will depend on the
    /// properties of the `SBankExporter` instance and the data in the `SBank`.
    /// Note that since `write_students_to_docx` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the DOCX document.
    fn write_students_to_docx(&self) -> Result<Docx, String>
    {
        let pt_to_usize = |pt: f32| -> usize { (pt as usize) << 1 };
        let footer_font_size = pt_to_usize(self.footer_font_size);
        let mut footer_run = Run::new();
        if self.is_footer_bold()
            { footer_run = footer_run.bold(); }
        if self.is_footer_italic()
            { footer_run = footer_run.italic(); }
        if self.is_footer_underline()
            { footer_run = footer_run.underline("single"); }
        if self.is_footer_strike()
            { footer_run = footer_run.strike(); }
        let footer = Footer::new()
            .add_paragraph(
                Paragraph::new()
                    .add_run(footer_run.clone()
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::PAGE(InstrPAGE::default()))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // Placeholder text
                            .add_field_char(FieldCharType::End, false)
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .add_run(footer_run.clone()
                            .add_text(" / ")
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .add_run(footer_run
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::NUMPAGES(InstrNUMPAGES::default()))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // Placeholder text
                            .add_field_char(FieldCharType::End, false)
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .align(AlignmentType::Center)
            );
        let mm_to_twips = |mm: f32| -> i32  { (mm * 56.6929).round() as i32 };
        let left = mm_to_twips(self.margin_left_in_mm);
        let right = mm_to_twips(self.margin_right_in_mm);
        let top = mm_to_twips(self.margin_top_in_mm);
        let buttom = mm_to_twips(self.margin_buttom_in_mm);
        let mut docx = Docx::new()
                        .page_margin(
                            PageMargin::new()
                                .left(left)
                                .right(right)
                                .top(top)
                                .bottom(buttom)
            ) // 1 cm for default left, right, top, bottom
            .footer(footer);
        
        // student Bank Title
        let title_font_size = pt_to_usize(self.title_font_size);
        let mut title_run = Run::new();
        if self.is_title_bold()
            { title_run = title_run.bold(); }
        if self.is_title_italic()
            { title_run = title_run.italic(); }
        if self.is_title_underline()
            { title_run = title_run.underline("single"); }
        if self.is_title_strike()
            { title_run = title_run.strike(); }
        
        let ex = Paragraph::new()
                .add_run(
                    title_run
                    .add_text(format!("{}", self.student_list_title))
                    .size(title_font_size)
                )
                .align(AlignmentType::Center);
        
        let body_font_size = pt_to_usize(self.body_font_size);
        let mut body_run = Run::new();
        if self.is_body_bold()
            { body_run = body_run.bold(); }
        if self.is_body_italic()
            { body_run = body_run.italic(); }
        if self.is_body_underline()
            { body_run = body_run.underline("single"); }
        if self.is_body_strike()
            { body_run = body_run.strike(); }

        // Blank line
        let blank_line = Paragraph::new();
        docx = docx.add_paragraph(ex);
        docx = docx.add_paragraph(blank_line.clone());

        for student in self.sbank.get_students()
        {
            let name = student.get_name();
            let id = student.get_id();
            let txt = format!("{}    {}", name, id);
            let mut q_para = Paragraph::new();
            let run = body_run.clone().add_text(txt).size(body_font_size);
            q_para = q_para.add_run(run);
            docx = docx.add_paragraph(q_para);
        }
        Ok(docx)
    }
    
    // pub fn save_sbank_in_hwpx(&self, path: &Path) -> Result<(), String>
    /// Saves the student bank to a HWPX file.
    ///
    /// This function generates a HWPX document containing the student bank,
    /// with specified page margins and a footer with page numbers.
    /// 
    /// # Arguments
    /// * `path` - The file path where the HWPX document will be saved.
    /// 
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    /// 
    /// # Cautions
    /// This function is unstable at the moment! Don't use this.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.save_sbank_in_hwpx(Path::new("student_bank.hwpx"), true);
    /// assert!(result.is_ok());
    /// std::fs::remove_file("student_bank.hwpx").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_sbank_in_hwpx(&self, path: &Path) -> Result<(), String>
    {
        self.build_sbank_in_hwpx()?
            .save_to_file(path)
            .map_err(|e| e.to_string())
    }

    // pub fn export_sbank_in_hwpx(&self) -> Result<Vec<u8>, String>
    /// Exports the student bank to a Vec<u8> object.
    ///
    /// This function generates a HWPX document containing the student bank,
    /// with specified page margins and a footer with page numbers,
    /// and returns the document as a Vec<u8> object.
    /// 
    /// # Arguments
    /// 
    /// # Returns
    /// `Result<Vec<u8>, String>` - Returns `Ok(Vec<u8>)` containing the HWPX
    ///                             document on success, or an `Err` with a
    ///                             `String` describing the error on failure.
    /// 
    /// # Cautions
    /// This function is unstable at the moment! Don;t use this.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.export_sbank_in_hwpx(true);
    /// assert!(result.is_ok());
    /// std::fs::write("student_bank.hwpx", result.unwrap()).unwrap();
    /// std::fs::remove_file("student_bank.hwpx").unwrap();
    /// ```
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn export_sbank_in_hwpx(&self) -> Result<Vec<u8>, String>
    {
        self.build_sbank_in_hwpx()?
            .to_bytes()
            .map_err(|e| e.to_string())
    }

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    #[inline]
    fn build_sbank_in_hwpx(&self) -> Result<HwpxWriter, String>
    {
        self.write_students_to_hwpx()
    }

    // fn write_students_to_hwpx(&self) -> Result<HwpxWriter, String>
    /// Writes the formatted exam content for a single student to a HWPX document.
    ///
    /// This private helper function takes a mutable HWPX `HwpxWriter` object and
    /// appends the exam content for the given student and their shuffled
    /// student bank, applying HWPX-specific formatting such as font sizes.
    ///
    /// # Returns
    /// `Result<HwpxWriter, String>` - Returns `Ok(HwpxWriter)` on success, or
    ///                                an `Err` with a
    ///                                `String` describing the error on failure.
    /// 
    /// # Cautions
    /// This function is unstable at the moment! Don;t use this.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.write_students_to_hwpx()?;
    /// ```
    /// This example demonstrates how the `write_students_to_hwpx` function
    /// can be called to write sbank content for a student into a HWPX document.
    /// The actual content of the students and formatting will depend on the
    /// properties of the `SBankExporter` instance and the data in the `SBank`.
    /// Note that since `write_students_to_hwpx` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the HWPX document.
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn write_students_to_hwpx(&self) -> Result<HwpxWriter, String>
    {
        let mut hwpx = HwpxWriter::new();
        hwpx.add_footer_with_page_number(" ");

        let mut title_style = HwpxTextStyle::new().size(self.title_font_size.round() as u32);
        if self.is_title_bold()
            { title_style = title_style.bold(); }
        if self.is_title_italic()
            { title_style = title_style.italic(); }
        if self.is_title_underline()
            { title_style = title_style.underline(); }
        if self.is_title_strike()
            { title_style = title_style.strikethrough(); }

        hwpx.add_mixed_styled_paragraph(vec![StyledText {
            text: self.student_list_title.clone(),
            style: title_style,
        }]).map_err(|e| e.to_string())?;

        let mut body_style = HwpxTextStyle::new().size(self.body_font_size.round() as u32);
        if self.is_body_bold()
            { body_style = body_style.bold(); }
        if self.is_body_italic()
            { body_style = body_style.italic(); }
        if self.is_body_underline()
            { body_style = body_style.underline(); }
        if self.is_body_strike()
            { body_style = body_style.strikethrough(); }

        hwpx.add_paragraph("").map_err(|e| e.to_string())?;

        for student in self.sbank.get_students()
        {
            let text = format!("{}    {}", student.get_name(), student.get_id());
            let style = body_style.clone();
            hwpx.add_mixed_styled_paragraph(vec![ StyledText { text, style }])
                .map_err(|e| e.to_string())?;
        }

        Ok(hwpx)
    }

    // pub fn save_sbank_in_hwp(&self, path: &Path) -> Result<(), String>
    /// Saves the student bank to a HWP file.
    ///
    /// This function generates a HWP document containing the student bank,
    /// with specified page margins and a footer with page numbers.
    /// 
    /// # Arguments
    /// * `path` - The file path where the HWP document will be saved.
    /// 
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    /// 
    /// # Cautions
    /// This function is unstable at the moment! Don;t use this.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.save_sbank_in_hwp(Path::new("student_bank.hwp"), true);
    /// assert!(result.is_ok());
    /// std::fs::remove_file("student_bank.hwp").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_sbank_in_hwp(&self, path: &Path) -> Result<(), String>
    {
        self.build_sbank_in_hwp()?
            .save_to_file(path)
            .map_err(|e| e.to_string())
    }

    // pub fn export_sbank_in_hwp(&self) -> Result<Vec<u8>, String>
    /// Exports the student bank to a Vec<u8> object.
    ///
    /// This function generates a HWP document containing the student bank,
    /// with specified page margins and a footer with page numbers,
    /// and returns the document as a Vec<u8> object.
    /// 
    /// # Returns
    /// `Result<Vec<u8>, String>` - Returns `Ok(Vec<u8>)` containing the HWP
    ///                             document on success, or an `Err` with a
    ///                             `String` describing the error on failure.
    /// 
    /// # Cautions
    /// This function is unstable at the moment! Don;t use this.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.export_sbank_in_hwp(true);
    /// assert!(result.is_ok());
    /// std::fs::write("student_bank.hwp", result.unwrap()).unwrap();
    /// std::fs::remove_file("student_bank.hwp").unwrap();
    /// ```
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn export_sbank_in_hwp(&self) -> Result<Vec<u8>, String>
    {
        self.build_sbank_in_hwp()?
            .to_bytes()
            .map_err(|e| e.to_string())
    }

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    #[inline]
    fn build_sbank_in_hwp(&self) -> Result<HwpWriter, String>
    {
        self.write_students_to_hwp()
    }

    // fn write_students_to_hwp(&self) -> Result<HwpWriter, String>
    /// Writes the student bank content to a HWP document.
    ///
    /// This private helper function takes a mutable HWP `HwpWriter` object and
    /// appends the exam content, applying HWP-specific formatting
    /// such as font sizes.
    ///
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    /// 
    /// # Cautions
    /// This function is unstable at the moment! Don;t use this.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.write_students_to_hwp();
    /// assert!(result.is_ok());
    /// std::fs::write("question_bank.hwp", result.unwrap()).unwrap();
    /// std::fs::remove_file("question_bank.hwp").unwrap();
    /// ```
    /// This example demonstrates how the `write_students_to_hwp` function
    /// can be called to write student bank content into a HWP document.
    /// The actual content of the students and formatting will depend on the
    /// properties of the `SBankExporter` instance and the data in the `SBank`.
    /// Note that since `write_students_to_hwp` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the HWP document.
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn write_students_to_hwp(&self) -> Result<HwpWriter, String>
    {
        use hwpers::writer::style::StyledText;

        let mut hwp = HwpWriter::new();
        hwp.set_page_margins_mm(
            self.margin_left_in_mm,
            self.margin_right_in_mm,
            self.margin_top_in_mm,
            self.margin_buttom_in_mm,
        );
        hwp.add_footer_with_page_number("", Numeric);

        let mut title_style = TextStyle::new().size(self.title_font_size.round() as u32);
        if self.is_title_bold()
            { title_style = title_style.bold(); }
        if self.is_title_italic()
            { title_style = title_style.italic(); }
        if self.is_title_underline()
            { title_style = title_style.underline(); }
        if self.is_title_strike()
            { title_style = title_style.strikethrough(); }

        let mut title_styled = StyledText::new(self.student_list_title.clone());
        title_styled = title_styled.add_range(0, self.student_list_title.len(), title_style);
        hwp.add_styled_paragraph(&title_styled).map_err(|e| e.to_string())?;
        hwp.add_paragraph("").map_err(|e| e.to_string())?;

        let mut body_style = TextStyle::new().size(self.body_font_size.round() as u32);
        if self.is_title_bold()
            { body_style = body_style.bold(); }
        if self.is_title_italic()
            { body_style = body_style.italic(); }
        if self.is_title_underline()
            { body_style = body_style.underline(); }
        if self.is_title_strike()
            { body_style = body_style.strikethrough(); }

        for student in self.sbank.get_students()
        {
            let text = format!("{}    {}", student.get_name(), student.get_id());
            let mut styled = StyledText::new(text.clone());
            styled = styled.add_range(0, text.len(), body_style.clone());
            hwp.add_styled_paragraph(&styled).map_err(|e| e.to_string())?;
        }
        Ok(hwp)
    }

    // pub fn save_sbank_in_pdf(&self, path: &Path) -> Result<(), String>
    /// Saves the student bank to a PDF file.
    ///
    /// This function generates a PDF document containing the student bank,
    /// with a footer showing page numbers.
    ///
    /// # Arguments
    /// * `path` - The file path where the PDF document will be saved.
    ///
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    /// `String` describing the error on failure.
    ///
    /// # Caution
    /// - This method searches for four specific font files within a `./fonts` 
    ///   subdirectory relative to the current working directory.
    /// - The attributes of underline and strike are not working.
    ///
    /// The following files must be present for the function to operate correctly:
    /// * `font-Regular.ttf`
    /// * `font-Italic.ttf`
    /// * `font-Bold.ttf`
    /// * `font-BoldItalic.ttf`
    ///
    /// If the directory or any of these files are missing, the function will fail. 
    /// Ensure that the `fonts` directory is created and all four files are 
    /// correctly named before calling this method.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.save_sbank_in_pdf(Path::new("student_bank.pdf"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("student_bank.pdf").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_sbank_in_pdf(&self, path: &Path) -> Result<(), String>
    {
        let doc = self.save_students_to_pdf()?;
        doc.render_to_file(path).map_err(|e: Error| e.to_string())?;

        Ok(())
    }

    // fn save_students_to_pdf(&self, mut doc: Document) -> Result<Document, String>
    /// Writes the student bank content for a single student to a PDF document.
    ///
    /// This private helper function takes a mutable PDF `Document` object
    /// and appends the student bank content, applying PDF-specific formatting
    /// such as font sizes.
    ///
    /// # Arguments
    /// * `doc` - A mutable reference to the `Document` object.
    ///
    /// # Returns
    /// It returns `Ok(Document)` on success, or
    ///            an `Err` with a `String` describing the error on failure.
    /// 
    /// # Caution
    /// - The attributes of underline and strike are not working.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let mut doc = genpdfi::Document::new(genpdfi::fonts::from_files("./fonts", "font", None).unwrap());
    /// let result = exporter.save_students_to_pdf(&mut doc);
    /// ```
    /// This example demonstrates how the `save_students_to_pdf` function
    /// can be called to write student bank content for a student into a PDF
    /// document.
    /// The actual content of the students and formatting will depend on the
    /// properties of the `SBankExporter` instance and the data in the `SBank`.
    /// Note that since `save_students_to_pdf` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the PDF document.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn save_students_to_pdf(&self) -> Result<Document, String>
    {
        let font_family = fonts::from_files("./fonts", "font", None).map_err(|e| format!("Failed to load font: {}", e))?;
        let mut doc = Document::new(font_family);
        // Set 1cm margins (10mm) and page numbers for all sides
        let mut decorator = SimplePageDecorator::new();
        let margin = (self.margin_left_in_mm + self.margin_right_in_mm + self.margin_top_in_mm + self.margin_buttom_in_mm) / 4.0;
        decorator.set_margins(margin); // 10mm = 1cm
        doc.set_page_decorator(decorator);

        // Define font sizes
        let title_font_size = self.title_font_size as u8;       // 14 pt for default
        let normal_font_size = self.body_font_size as u8;    // 11 pt for default

        let mut title_style = style::Style::new();
        title_style.set_font_size(title_font_size);
        if self.is_title_bold()
            { title_style.set_bold(); }
        if self.is_title_italic()
            { title_style.set_italic(); }

        // Exam Title
        let mut title_paragraph = elements::Paragraph::new(format!("{}", self.student_list_title));
        title_paragraph.set_alignment(Alignment::Center);
        doc.push(title_paragraph.styled(title_style));

        let mut body_style = style::Style::new();
        body_style.set_font_size(normal_font_size);
        if self.is_body_bold()
            { body_style.set_bold(); }
        if self.is_body_italic()
            { body_style.set_italic(); }

        for student in self.sbank.get_students()
        {
            let txt = format!("{}    {}", student.get_name(), student.get_id());
            doc.push(elements::Paragraph::new(txt).styled(body_style));
            doc.push(elements::Paragraph::new("")); // Blank line after each student
        }

        Ok(doc)
    }

    // pub fn export_sbank_in_pdf(&self) -> Result<Vec<u8>, String>
    /// Exports the student bank as a PDF-compatible JSON byte vector.
    ///
    /// This function generates a JSON structure that can be used with the `pdfmake`
    /// library ot javascript to create a PDF document.
    /// It includes the student bankfor all students and an answer sheet.
    ///
    /// # Returns
    /// `Result<Vec<u8>, String>` - Returns a byte vector containing the JSON string
    ///                            on success, or an `Err` with a `String`
    ///                            describing the error on failure.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ SBank, SBankExporter, student };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut sbank = SBank::new();
    /// sbank.push_student(Student::new("홍길동".to_string(), "1".to_string()));
    /// sbank.push_student(Student::new("John Doe".to_string(), "2".to_string()));
    /// sbank.push_student(Student::new("Иванов Иван Иванович".to_string(), "3".to_string()));
    /// 
    /// // Exports a student bank with 3 students
    /// let exporter = SBankExporter::new(&sbank, "Student List".to_string());
    /// let result = exporter.export_shuffled_exams_in_pdf();
    /// assert!(result.is_ok());
    /// std::fs::write("exam_exported.json", result.unwrap()).unwrap();
    /// std::fs::remove_file("exam_exported.json").unwrap();
    /// ```
    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    pub fn export_sbank_in_pdf(&self) -> Result<Vec<u8>, String>
    {

        let mut content = Vec::new();

        let title_font_size = self.title_font_size;
        let body_font_size = self.body_font_size;

        let title_style = json!({
            "fontSize": title_font_size,
            "bold": self.is_title_bold(),
            "italics": self.is_title_italic(),
            "decoration": if self.is_title_underline() { Some("underline") } else if self.is_title_strike() { Some("lineThrough") } else { None },
            "alignment": "center",
            "margin": [0, 0, 0, 10]
        });

        let body_style = json!({
            "fontSize": body_font_size,
            "bold": self.is_body_bold(),
            "italics": self.is_body_italic(),
            "decoration": if self.is_body_underline() { Some("underline") } else if self.is_body_strike() { Some("lineThrough") } else { None },
            "margin": [0, 2, 0, 2]
        });
        
        // Title
        content.push(json!({"text": self.student_list_title, "style": "title"}));

        // students
        for student in self.sbank.get_students()
        {
            content.push(json!({
                                "text": format!("{}    {}", student.get_name(), student.get_id()),
                                "style": "body"
                }));
            content.push(json!({"text": "\n"}));
        }

        let doc_definition = json!({
            "content": content,
            "styles": {
                "title": title_style,
                "body": body_style,
            }
        });

        serde_json::to_vec(&doc_definition).map_err(|e| e.to_string())
    }

    // #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    // fn export_students_to_pdf(&self) -> Result<Vec<u8>, String>
    // {
    //     let mut content = Vec::new();

    //     let title_font_size = self.title_font_size;
    //     let body_font_size = self.body_font_size;
    //     let student_list_font_size = self.student_list_font_size;

    //     let title_style = json!({
    //         "fontSize": title_font_size,
    //         "bold": self.is_title_bold(),
    //         "italics": self.is_title_italic(),
    //         "decoration": if self.is_title_underline() { Some("underline") } else if self.is_title_strike() { Some("lineThrough") } else { None },
    //         "alignment": "center",
    //         "margin": [0, 0, 0, 10]
    //     });

    //     let body_style = json!({
    //         "fontSize": body_font_size,
    //         "bold": self.is_body_bold(),
    //         "italics": self.is_body_italic(),
    //         "decoration": if self.is_body_underline() { Some("underline") } else if self.is_body_strike() { Some("lineThrough") } else { None },
    //         "margin": [0, 2, 0, 2]
    //     });

    //     let student_list_style = json!({
    //         "fontSize": student_list_font_size,
    //         "bold": self.is_student_list_bold(),
    //         "italics": self.is_student_list_italic(),
    //         "decoration": if self.is_student_list_underline() { Some("underline") } else if self.is_student_list_strike() { Some("lineThrough") } else { None },
    //         "margin": [0, 2, 0, 2]
    //     });
        
    //     // Title
    //     content.push(json!({"text": self.student_list_title, "style": "title"}));

    //     // students
    //     for student in self.sbank.get_students()
    //     {
    //         content.push(json!({
    //                             "text": format!("{}    {}", student.get_name(), student.get_id()),
    //                             "style": "body"
    //             }));
    //         content.push(json!({"text": "\n"}));
    //     }

    //     let doc_definition = json!({
    //         "content": content,
    //         "styles": {
    //             "title": title_style,
    //             "body": body_style,
    //             "answer_sheet": answer_sheet_style
    //         }
    //     });

    //     serde_json::to_vec(&doc_definition).map_err(|e| e.to_string())
    // }
}