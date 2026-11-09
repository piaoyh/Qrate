// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::io::Cursor;

use docx_rs::{ Docx, Paragraph, Run, BreakType, PageMargin, AlignmentType,
                Footer, InstrText, InstrPAGE, InstrNUMPAGES, FieldCharType };
use hwpers::{ HwpWriter, HwpxWriter };
use hwpers::style::TextStyle;
use hwpers::hwpx::{ HwpxTextStyle, StyledText };

use crate::{ Choices, QBank, Generator };

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::fs::File;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::io::Write;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::path::Path;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use genpdfi::error::Error;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use genpdfi::{ Document, elements, fonts, style, Element, SimplePageDecorator, Alignment };

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use crate::check_path;


pub struct QBankExporter
{
    qbank: QBank,
    current_question_number: u16,
    body_font_size: f32,
    title_font_size: f32,
    answer_sheet_font_size: f32,
    footer_font_size: f32,
    attributes: u16,
    margin_left_in_mm: f32,
    margin_right_in_mm: f32,
    margin_top_in_mm: f32,
    margin_buttom_in_mm: f32,
    line_spacing: f32,
    answer_sheet_title: String,
}

impl QBankExporter
{
    const DUMMY_STRING: &str = "_____________________";

    // pub new(qbank: QBank, answer_sheet_title: String) -> Self
    /// Creates a new `Exporter` instance for exporting question bank.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `answer_sheet_title` - The title to be used for the answer sheet.
    ///
    /// # Returns
    /// A `Self` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// ```
    pub fn new(qbank: QBank, answer_sheet_title: String) -> Self
    {
        Self
        {
            qbank,
            current_question_number: 0,
            title_font_size: Generator::DEFAULT_TITLE_FONT_SIZE,
            body_font_size: Generator::DEFAULT_BODY_FONT_SIZE,
            answer_sheet_font_size: Generator::DEFAULT_ANSWER_SHEET_FONT_SIZE,
            footer_font_size: Generator::DEFAULT_FOOTER_FONT_SIZE,
            attributes: Generator::DEFAULT_ATTRIBUTES,
            margin_left_in_mm: Generator::DEFAULT_MARGIN_LEFT_IN_MM,
            margin_right_in_mm: Generator::DEFAULT_MARGIN_RIGHT_IN_MM,
            margin_top_in_mm: Generator::DEFAULT_MARGIN_TOP_IN_MM,
            margin_buttom_in_mm: Generator::DEFAULT_MARGIN_BUTTOM_IN_MM,
            line_spacing: Generator::DEFAULT_LINE_SPACING,
            answer_sheet_title
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_footer_font_size(10.0);
    /// assert_eq!(exporter.get_footer_font_size(), 10.0);
    /// ```
    #[inline]
    pub fn set_footer_font_size(&mut self, footer_font_size: f32)
    {
        self.footer_font_size = footer_font_size
    }
    
    // pub fn get_answer_sheet_font_size(&self) -> f32
    /// Retrieves the current answer sheet font size in points.
    ///
    /// # Returns
    /// `f32` - The current font size used for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let font_size = exporter.get_answer_sheet_font_size();
    /// assert_eq!(font_size, 12.0);
    /// ```
    #[inline]
    pub fn get_answer_sheet_font_size(&self) -> f32
    {
        self.answer_sheet_font_size
    }
    
    // pub fn set_answer_sheet_font_size(&mut self, answer_sheet_font_size: f32)
    /// Sets the answer sheet font size in points.
    ///
    /// # Arguments
    /// * `answer_sheet_font_size` - The new font size to be used for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_answer_sheet_font_size(13.0);
    /// assert_eq!(exporter.get_answer_sheet_font_size(), 13.0);
    /// ```
    #[inline]
    pub fn set_answer_sheet_font_size(&mut self, answer_sheet_font_size: f32)
    {
        self.answer_sheet_font_size = answer_sheet_font_size;
    }

    // pub fn is_body_bold(&self) -> bool
    /// Checks if the body text is set to bold.
    ///
    /// # Returns
    /// `bool` - `true` if the body text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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

    // pub fn is_answer_sheet_bold(&self) -> bool
    /// Checks if the answer sheet text is set to bold.
    ///
    /// # Returns
    /// `bool` - `true` if the answer sheet text is bold, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// assert!(!exporter.is_answer_sheet_bold());
    /// exporter.set_answer_sheet_bold(true);
    /// assert!(exporter.is_answer_sheet_bold());
    /// ```
    #[inline]
    pub fn is_answer_sheet_bold(&self) -> bool
    {
        (self.attributes & Generator::ANSWER_SHEET_BOLD) == Generator::ANSWER_SHEET_BOLD
    }

    // pub fn set_answer_sheet_bold(&mut self, on: bool)
    /// Sets or unsets the bold attribute for the answer sheet text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the answer sheet text to bold, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_answer_sheet_bold(true);
    /// assert!(exporter.is_answer_sheet_bold());
    /// exporter.set_answer_sheet_bold(false);
    /// assert!(!exporter.is_answer_sheet_bold());
    /// ```
    #[inline]
    pub fn set_answer_sheet_bold(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::ANSWER_SHEET_BOLD; }
        else
            { self.attributes &= !Generator::ANSWER_SHEET_BOLD; }
    }

    // pub fn is_answer_sheet_italic(&self) -> bool
    /// Checks if the answer sheet text is set to italic.
    ///
    /// # Returns
    /// `bool` - `true` if the answer sheet text is italic, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// assert!(!exporter.is_answer_sheet_italic());
    /// exporter.set_answer_sheet_italic(true);
    /// assert!(exporter.is_answer_sheet_italic());
    /// ```
    #[inline]
    pub fn is_answer_sheet_italic(&self) -> bool
    {
        (self.attributes & Generator::ANSWER_SHEET_ITALIC) == Generator::ANSWER_SHEET_ITALIC
    }

    // pub fn set_answer_sheet_italic(&mut self, on: bool)
    /// Sets or unsets the italic attribute for the answer sheet text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the answer sheet text to italic, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_answer_sheet_italic(true);
    /// assert!(exporter.is_answer_sheet_italic());
    /// exporter.set_answer_sheet_italic(false);
    /// assert!(!exporter.is_answer_sheet_italic());
    /// ```
    #[inline]
    pub fn set_answer_sheet_italic(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::ANSWER_SHEET_ITALIC; }
        else
            { self.attributes &= !Generator::ANSWER_SHEET_ITALIC; }
    }

    // pub fn is_answer_sheet_underline(&self) -> bool
    /// Checks if the answer sheet text is set to underline.
    ///
    /// # Returns
    /// `bool` - `true` if the answer sheet text is underlined, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// assert!(!exporter.is_answer_sheet_underline());
    /// exporter.set_answer_sheet_underline(true);
    /// assert!(exporter.is_answer_sheet_underline());
    /// ```
    #[inline]
    pub fn is_answer_sheet_underline(&self) -> bool
    {
        (self.attributes & Generator::ANSWER_SHEET_UNDERLINE) == Generator::ANSWER_SHEET_UNDERLINE
    }

    // pub fn set_answer_sheet_underline(&mut self, on: bool)
    /// Sets or unsets the underline attribute for the answer sheet text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the answer sheet text to underline, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_answer_sheet_underline(true);
    /// assert!(exporter.is_answer_sheet_underline());
    /// exporter.set_answer_sheet_underline(false);
    /// assert!(!exporter.is_answer_sheet_underline());
    /// ```
    #[inline]
    pub fn set_answer_sheet_underline(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::ANSWER_SHEET_UNDERLINE; }
        else
            { self.attributes &= !Generator::ANSWER_SHEET_UNDERLINE; }
    }

    // pub fn is_answer_sheet_strike(&self) -> bool
    /// Checks if the answer sheet text is set to strikethrough.
    ///
    /// # Returns
    /// `bool` - `true` if the answer sheet text is strikethrough, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_answer_sheet_strike(true);
    /// assert!(exporter.is_answer_sheet_strike());
    /// ```
    #[inline]
    pub fn is_answer_sheet_strike(&self) -> bool
    {
        (self.attributes & Generator::ANSWER_SHEET_STRIKE) == Generator::ANSWER_SHEET_STRIKE
    }

    // pub fn set_answer_sheet_strike(&mut self, on: bool)
    /// Sets or unsets the strikethrough attribute for the answer sheet text.
    ///
    /// # Arguments
    /// * `on` - `true` to set the answer sheet text to strikethrough, `false` to unset it.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_answer_sheet_strike(true);
    /// assert!(exporter.is_answer_sheet_strike());
    /// exporter.set_answer_sheet_strike(false);
    /// assert!(!exporter.is_answer_sheet_strike());
    /// ```
    #[inline]
    pub fn set_answer_sheet_strike(&mut self, on: bool)
    {
        if on
            { self.attributes |= Generator::ANSWER_SHEET_STRIKE; }
        else
            { self.attributes &= !Generator::ANSWER_SHEET_STRIKE; }
    }

    // pub fn get_attributes(&self) -> u16
    /// Retrieves the current attribute bitmask.
    ///
    /// # Returns
    /// `u16` - The current attribute bitmask.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
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
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_line_spacing(1.5);
    /// assert_eq!(exporter.get_line_spacing(), 1.5);
    /// ```
    #[inline]
    pub fn set_line_spacing(&mut self, line_spacing: f32)
    {
        self.line_spacing = line_spacing;
    }
    
    // pub fn get_answer_sheet_title(&self) -> String
    /// Retrieves the current answer sheet title.
    ///
    /// # Returns
    /// `String` - The current title for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let title = exporter.get_answer_sheet_title();
    /// assert_eq!(title, "Answer Sheet        정답지        Ответы".to_string());
    /// ```
    #[inline]
    pub fn get_answer_sheet_title(&self) -> String
    {
        self.answer_sheet_title.clone()
    }
    
    // pub fn set_answer_sheet_title(&mut self, answer_sheet_title: String)
    /// Sets the answer sheet title.
    ///
    /// # Arguments
    /// * `answer_sheet_title` - The new title for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// exporter.set_answer_sheet_title("New Answer Sheet Title".to_string());
    /// assert_eq!(exporter.get_answer_sheet_title(), "New Answer Sheet Title".to_string());
    /// ```
    #[inline]
    pub fn set_answer_sheet_title(&mut self, answer_sheet_title: String)
    {
        self.answer_sheet_title = answer_sheet_title;
    }

    // pub fn get_notice(&self) -> String
    /// Retrieves the notice string from the original question bank's header.
    ///
    /// This function accesses the header of the `QBank` used to create the
    /// `Exporter` instance and returns its notice string.
    ///
    /// # Output
    /// A `String` containing the notice from the question bank's header.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter, Header };
    ///
    /// let mut qbank = QBank::new_empty();
    /// let mut header = Header::new_empty();
    ///
    /// header.set_notice("Important Notice!".to_string());
    /// qbank.set_header(header);
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    /// 
    /// // Exports a question bank with 2 questions
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let notice = exporter.get_notice();
    /// assert_eq!(notice, "Important Notice!");
    /// ```
    #[inline]
    pub fn get_notice(&self) -> String
    {
        self.qbank.get_header().get_notice().clone()
    }

    // pub fn next(&mut self) -> Option<(u16, String, String, Choices)>
    /// Advances to the next question in the Question bank
    /// and returns its details.
    ///
    /// This function acts as an iterator for the question bank. Each
    /// call increments the internal question counter and provides the details
    /// of the next question, including the category, the question text,
    /// and the choices.
    ///
    /// It is primarily used for exporting scenarios.
    ///
    /// # Returns
    /// `Option<(u16, u8, String, String, Choices)>` - An `Option` containing
    /// a tuple with:
    ///   - `u16`: The current question number within the shuffled set.
    ///   - `u8`: The category ID of the current question.
    ///   - `String`: The category string of the current question.
    ///   - `String`: The text of the current question.
    ///   - `Choices`: A vector of tuples `(String, bool)` representing the
    ///                shuffled choices and whether each is a correct answer.
    ///
    /// Returns `None` if there are no more questions in the set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    ///
    /// let mut exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    ///
    /// if let Some((num, cat, q_text, choices)) = exporter.next()
    ///     { assert_eq!(num, 1); }  // The actual question text depends on the shuffled order.
    ///
    /// if let Some((num, cat, q_text, choices)) = exporter.next()
    ///     { assert_eq!(num, 2); }
    /// assert!(exporter.next().is_none());
    /// ```
    pub fn next(&mut self) -> Option<(u16, u8, String, String, Choices)>
    {
        self.current_question_number += 1;
        self.get_question_by_number(self.current_question_number)
    }

    // pub fn prev(&mut self) -> Option<(u16, String, String, Choices)>
    /// Withdraws to the previous question in the question bank
    /// and returns its details.
    ///
    /// This function acts as an iterator for the question bank. Each
    /// call decrements the internal question counter and provides the details
    /// of the previous question, including the category, the question text,
    /// and the choices in their shuffled order.
    ///
    /// It is primarily used for exporting scenarios.
    ///
    /// # Returns
    /// `Option<(u16, u8, String, String, Choices)>` - An `Option` containing
    /// a tuple with:
    ///   - `u16`: The current question number within the shuffled set.
    ///   - `u8`: The category ID of the current question.
    ///   - `String`: The category string of the current question.
    ///   - `String`: The text of the current question.
    ///   - `Choices`: A vector of tuples `(String, bool)` representing the
    ///                shuffled choices and whether each is a correct answer.
    ///
    /// Returns `None` if there are no more questions in the set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    ///
    /// let mut exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    ///
    /// if let Some((num, cat, q_text, choices)) = exporter.next()
    ///     { assert_eq!(num, 1); }  // The actual question text depends on the shuffled order.
    ///
    /// if let Some((num, cat, q_text, choices)) = exporter.next()
    ///     { assert_eq!(num, 2); }  // The actual question text depends on the shuffled order.
    ///
    /// if let Some((num, cat, q_text, choices)) = exporter.prev()
    ///     { assert_eq!(num, 1); }
    /// assert!(exporter.prev().is_none());
    /// ```
    pub fn prev(&mut self) -> Option<(u16, u8, String, String, Choices)>
    {
        self.current_question_number -= 1;
        self.get_question_by_number(self.current_question_number)
    }

    // pub fn get_question_by_number(&self, num: u16) -> Option<(u16, u8, String, String, Choices)>
    /// Retrieves a specific question by its number within the question bank.
    /// 
    /// This function returns a tuple containing the question number, category,
    /// question text, and the choices in their shuffled order.
    /// 
    /// # Arguments
    /// * `num` - The number of the question to retrieve.
    /// 
    /// # Returns
    /// `Option<(u16, u8, String, String, Choices)>` - An `Option` containing
    /// a tuple with:
    ///   - `u16`: The current question number within the shuffled set.
    ///   - `u8`: The category ID of the current question.
    ///   - `String`: The category string of the current question.
    ///   - `String`: The text of the current question.
    ///   - `Choices`: A vector of tuples `(String, bool)` representing the
    ///                shuffled choices and whether each is a correct answer.
    /// 
    /// Returns `None` if the question with the given number is not found.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    /// 
    /// let mut exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// 
    /// if let Some((num, cat_id, cat_str, q_text, choices)) = exporter.get_question_by_number(1)
    ///     { assert_eq!(num, 1); }  // The actual question text depends on the shuffled order.
    /// 
    /// if let Some((num, cat_id, cat_str, q_text, choices)) = exporter.get_question_by_number(2)
    ///     { assert_eq!(num, 2); }
    /// assert!(exporter.get_question_by_number(3).is_none());
    /// ```
    pub fn get_question_by_number(&mut self, question_number: u16) -> Option<(u16, u8, String, String, Choices)>
    {
        self.current_question_number = question_number;
        if question_number as usize > self.qbank.get_length() || question_number == 0
            { return None; }

        let question = self.qbank.get_question(question_number as usize)?;
        let category_id = question.get_category();
        let category_str = self.qbank.get_header().get_category(category_id)?.clone();
        let question_text = question.get_question().clone();
        let choices = question.get_choices();
        Some((question_number, category_id, category_str, question_text, choices.clone()))
    }

    // pub fn get_number_of_questions(&self) -> usize
    /// Returns the total number of shuffled questions for the first set (self-study).
    /// 
    /// # Returns
    /// `usize` - The total number of shuffled questions for the first set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    /// 
    /// let mut exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// assert_eq!(exporter.get_number_of_questions(), 2);
    /// ```
    #[inline]
    pub fn get_number_of_questions(&self) -> usize
    {
        self.qbank.get_length()
    }

    // pub fn get_current_question_number(&self) -> u16
    /// Retrieves the current question number within the shuffled set.
    /// 
    /// # Returns
    /// `u16` - The current question number within the shuffled set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    /// 
    /// let mut exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// assert_eq!(exporter.get_current_question_number(), 0);
    /// 
    /// if let Some((num, cat, q_text, choices)) = exporter.next()
    ///     { assert_eq!(num, 1); }
    /// assert_eq!(exporter.get_current_question_number(), 1);
    /// 
    /// if let Some((num, cat, q_text, choices)) = exporter.next()
    ///     { assert_eq!(num, 2); }
    /// assert_eq!(exporter.get_current_question_number(), 2);
    /// assert!(exporter.next().is_none());
    /// ```
    #[inline]
    pub fn get_current_question_number(&self) -> u16
    {
        self.current_question_number
    }

    // pub fn set_current_question_number(&mut self, num: u16)
    /// Sets the current question number within the shuffled set.
    /// 
    /// # Arguments
    /// * `num` - The new question number to set.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    /// 
    /// let mut exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// assert_eq!(exporter.get_current_question_number(), 0);
    /// 
    /// exporter.set_current_question_number(1);
    /// assert_eq!(exporter.get_current_question_number(), 1);
    /// ```
    #[inline]
    pub fn set_current_question_number(&mut self, num: u16)
    {
        self.current_question_number = num;
    }

    // pub fn save_qbank(&self, path: String, extention: &str, is_with_answer_sheet: bool) -> Result<(), String>
    /// Saves the question bank to a single file.
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
    /// use qrate::{ QBank, Exporter };
    /// use std::fs; // For std::fs::remove_file
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// // Generate exams with 1 question selected for each student
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    ///
    /// let mut result = exporter.save_qbank("qbank.txt".to_string(), "txt");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("qbank.txt").unwrap();
    ///
    /// result = exporter.save_qbank("qbank.docx".to_string(), "docx");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("qbank.docx").unwrap();
    ///
    /// result = exporter.save_qbank("qbank.hwpx".to_string(), "hwpx");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("qbank.hwpx").unwrap();
    ///
    /// result = exporter.save_qbank("qbank.hwp".to_string(), "hwp");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("qbank.hwp").unwrap();
    ///
    /// result = exporter.save_qbank("qbank.pdf".to_string(), "pdf");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("qbank.pdf").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_qbank(&self, path: String, extention: &str, is_with_answer_sheet: bool) -> Result<(), String>
    {
        let checked = check_path(path, extention);
        let file_path = Path::new(&checked);
        match file_path.extension().and_then(|s| s.to_str())
        {
            Some("txt") => self.save_qbank_in_txt(file_path, is_with_answer_sheet),
            Some("docx") => self.save_qbank_in_docx(file_path, is_with_answer_sheet),
            Some("hwpx") => self.save_qbank_in_hwpx(file_path, is_with_answer_sheet),
            Some("hwp") => self.save_qbank_in_hwp(file_path, is_with_answer_sheet),
            Some("pdf") => self.save_qbank_in_pdf(file_path, is_with_answer_sheet),
            _ => Err("Unsupported file format. Please use .txt, .docx, hwpx, hwp or .pdf.".to_string()),
        }
    }

    // pub fn save_qbank_in_txt(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    /// Saves the question bank to a text file.
    ///
    /// # Arguments
    /// * `path` - The file path where the text document will be saved.
    /// * `is_with_answer_sheet` indicates whether it is printed
    ///   with an answer sheet.
    ///
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Exporter };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices(
    ///     "What is 1+1?".to_string(),
    ///     vec![("1".to_string(), false), ("2".to_string(), true)]
    /// );
    /// qbank.add_question_with_choices(
    ///     "What is 2+2?".to_string(),
    ///     vec![("3".to_string(), false), ("4".to_string(), true)]
    /// );
    ///
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    ///
    /// let result = exporter.save_qbank_in_txt(Path::new("question_bank.txt"), true);
    /// assert!(result.is_ok());
    /// std::fs::remove_file("question_bank.txt").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_qbank_in_txt(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        let content = self.write_questions_in_txt();
        writeln!(file, "{}", content).map_err(|e| e.to_string())?;

        if is_with_answer_sheet
        {
            let answer_sheet = self.write_answer_sheet_in_txt();
            writeln!(file, "{}", answer_sheet).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    // pub fn export_qbank_in_txt(&self, is_with_answer_sheet: bool) -> Vec<u8>
    /// Exports the question bank to a Vec<u8> object.
    ///
    /// This function generates a Vec<u8> object containing the question bank
    /// in text format.
    ///
    /// # Returns
    /// `Vec<u8>` that contains the contents of String object.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices(
    ///     "What is 1+1?".to_string(),
    ///     vec![("1".to_string(), false), ("2".to_string(), true)]
    /// );
    /// qbank.add_question_with_choices(
    ///     "What is 2+2?".to_string(),
    ///     vec![("3".to_string(), false), ("4".to_string(), true)]
    /// );
    ///
    /// let exporter = Exporter::Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let content_bytes = exporter.export_qbank_in_txt(true);
    /// assert!(!content_bytes.is_empty());
    /// let content_str = String::from_utf8(content_bytes).unwrap();
    /// assert!(content_str.contains("What is 1+1?"));
    /// assert!(content_str.contains("What is 2+2?"));
    /// ```
    pub fn export_qbank_in_txt(&self, is_with_answer_sheet: bool) -> Vec<u8>
    {
        let mut content = String::new();
        let exam_content = self.write_questions_in_txt();
        content.push_str(exam_content.as_str());

        if is_with_answer_sheet
            { content.push_str(self.write_answer_sheet_in_txt().as_str()); }
        content.into_bytes()
    }

    // fn write_questions_in_txt(&self) -> String
    /// Writes the questions of the question bank content
    /// into a human-readable string.
    ///
    /// This private helper function generates the textual representation of a
    /// question bank. It includes the student's name, the exam title, and all
    /// questions with their choices.
    ///
    /// # Returns
    /// A `String` containing the fully formatted question bank content.
    fn write_questions_in_txt(&self) -> String
    {
        let mut content = String::new();
        let header = self.qbank.get_header();

        // Exam Title
        content.push_str(&format!("{}\n", header.get_title()));

        // Student Information
        content.push_str(&format!("{}: {}    {}: {}\n\n", header.get_name(), Self::DUMMY_STRING, header.get_id(), Self::DUMMY_STRING));

        // Instructions
        content.push_str(&format!("{}\n\n", header.get_notice()));

        // Questions
        for (question_index, question) in self.qbank.get_questions().iter().enumerate()
        {
            let question_id = question_index + 1;
            let group_id = question.get_group();
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s| s.as_str()).unwrap_or("");
            
            if question_id == group_id as usize
                { content.push_str(&format!("{}. [{}]   {}\n", question_id, category_text, question.get_question())); }
            else
                { content.push_str(&format!("{}{{={}}}. [{}]   {}\n", question_id, group_id, category_text, question.get_question())); }

            match category_id
            {
                3 => {
                    // Short answer: ( space * 3 * max_choice_len )
                    let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                    let spaces = " ".repeat(max_len * 3);
                    content.push_str(&format!("    ({})\n", spaces));
                },
                4 => {
                    // Essay: 15 blank lines
                    for _ in 0..15
                        { content.push_str("\n"); }
                },
                _ => {
                    // Category 1, 2: Standard choices
                    for (choice_index, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
                    {
                        let choice_char = (choice_index + 1).to_string();
                        content.push_str(&format!("    ({}) {}\n", choice_char, choice_text));
                    }
                },
            }
            content.push_str("\n"); // Blank line after each question
        }
        content
    }

    fn write_answer_sheet_in_txt(&self) -> String
    {
        let mut answer_sheet = String::new();
        answer_sheet.push_str(format!("{}\n", self.answer_sheet_title).as_str());

        let mut answer_line = String::new();
        for (i, question) in self.qbank.get_questions().iter().enumerate()
        {
            let category_id = question.get_category();
            let answer_string = match category_id
            {
                3 => {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                },
                4 => "(---)".to_string(),
                _ => {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                },
            };

            let entry = format!("{}. {}    ", i + 1, answer_string);

            // Simple line wrapping logic
            if answer_line.len() + entry.len() > 80 && !answer_line.is_empty()
            {
                answer_sheet.push_str(format!("{}\n", answer_line).as_str());
                answer_line.clear();
            }
            answer_line.push_str(entry.as_str());
        }
        if !answer_line.is_empty()
            { answer_sheet.push_str(format!("{}\n", answer_line).as_str()); }
        answer_sheet
    }

    // pub fn save_qbank_in_docx(&self, path: &Path) -> Result<(), String>
    /// Saves the question bank to a DOCX file.
    ///
    /// This function generates a DOCX document containing the question bank,
    /// applying specified page margins and a footer with page numbers.
    ///
    /// # Arguments
    /// * `path` - The file path where the DOCX document will be saved.
    /// * `is_with_answer_sheet` indicates whether it is printed
    ///   with an answer sheet.
    ///
    /// # Returns
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    ///
    /// let result = exporter.save_qbank_in_docx(Path::new("question_bank.docx"), true);
    /// assert!(result.is_ok());
    /// std::fs::remove_file("question_bank.docx").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_qbank_in_docx(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    {
        let file = File::create(path).map_err(|e| e.to_string())?;
        self.build_qbank_in_docx(is_with_answer_sheet)?
            .build()
            .pack(file)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // pub fn export_qbank_in_docx(&self, is_with_answer_sheet: bool) -> Result<Vec<u8>, String>
    /// Exports the squestion bank to a Vec<u8> object.
    ///
    /// This function generates a DOCX document containing the question bank,
    /// applying specified page margins and a footer with page numbers.
    /// The resulting DOCX content is returned as a `Vec<u8>`, which can be used
    /// for in-memory operations or further processing without writing to disk.
    /// 
    /// # Arguements
    /// * `is_with_answer_sheet` indicates whether it is printed
    ///   with an answer sheet.
    /// 
    /// # Returns
    /// `Result<Vec<u8>, String>` - Returns `Ok(Vec<u8>)` which is binary data
    /// on success, or an `Err` with a `String` describing the error on failure.
    /// 
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Exporter };
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices(
    ///     "What is 1+1?".to_string(),
    ///     vec![("1".to_string(), false), ("2".to_string(), true)]
    /// );
    /// qbank.add_question_with_choices(
    ///     "What is 2+2?".to_string(),
    ///     vec![("3".to_string(), false), ("4".to_string(), true)]
    /// );
    ///
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let docx_bytes = exporter.export_qbank_in_docx(true).unwrap();
    /// assert!(!docx_bytes.is_empty());
    /// // Further processing of docx_bytes can be done here, such as saving to a file or sending over a network.
    /// ```
    pub fn export_qbank_in_docx(&self, is_with_answer_sheet: bool) -> Result<Vec<u8>, String>
    {
        let mut buffer = Cursor::new(Vec::new());
        self.build_qbank_in_docx(is_with_answer_sheet)?
            .build()
            .pack(&mut buffer)
            .map_err(|e| e.to_string())?;
        Ok(buffer.into_inner())
    }

    fn build_qbank_in_docx(&self, is_with_answer_sheet: bool) -> Result<Docx, String>
    {
        let mut docx = self.write_questions_to_docx()?;

        if is_with_answer_sheet
            { docx = self.write_answer_sheet_to_docx(docx)?; }
        docx = docx.add_paragraph(Paragraph::new()); // Blank line

        Ok(docx)
    }

    // fn write_questions_to_docx(&self) -> Result<Docx, String>
    /// Writes the formatted question bank content to a DOCX document.
    ///
    /// This private helper function takes a DOCX `Docx` object and appends
    /// the question bank contents, returning the modified `Docx` object.
    ///
    /// # Returns
    /// `Result<Docx, String>` - Returns `Ok(Docx)` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let result = exporter.write_questions_to_docx(Docx::new());
    /// ```
    /// This example demonstrates how the `write_questions_to_docx` function
    /// can be called to write qbank content into a DOCX document.
    /// The actual content of the questions and formatting will depend on the
    /// properties of the `Exporter` instance and the data in the `QBank`.
    /// Note that since `write_questions_to_docx` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the DOCX document.
    fn write_questions_to_docx(&self) -> Result<Docx, String>
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
        
        // Question Bank Title
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
        
        let header = self.qbank.get_header();
        let ex = Paragraph::new()
                .add_run(
                    title_run
                    .add_text(format!("{}", header.get_title()))
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
        
        let paragraph = |run: Run, txt, size| -> Paragraph
        {
            let elem = run.add_text(txt).size(size);  // `size` pt
            Paragraph::new().add_run(elem)
        };

        // Student Information
        let st = paragraph(body_run.clone(), format!("{}: {}    {}: {}", header.get_name(), Self::DUMMY_STRING, header.get_id(), Self::DUMMY_STRING), body_font_size);

        // Blank line
        let blank_line = Paragraph::new();

        docx = docx.add_paragraph(ex);
        docx = docx.add_paragraph(st);
        docx = docx.add_paragraph(blank_line.clone());

        // Instructions (Handle multi-line notice)
        for line in header.get_notice().lines()
        {
            let notice_para = paragraph(body_run.clone(), format!("{}", line), body_font_size);
            docx = docx.add_paragraph(notice_para);
        }

        docx = docx.add_paragraph(blank_line.clone());

        for (question_index, question) in self.qbank.get_questions().iter().enumerate()
        {
            let question_id = question_index + 1;
            let group_id = question.get_group();
            let category = header.get_category(question.get_category()).map(|s| s.as_str()).unwrap_or("");
            let mut q_para = Paragraph::new();
            let mut lines = question.get_question().lines().peekable();
            let mut is_first_line = true;
            while let Some(line) = lines.next()
            {
                let txt = if is_first_line
                {
                    if question_id == group_id as usize
                        { format!("{}. [{}]   {}", question_id, category, line) }
                    else
                        { format!("{}{{={}}}. [{}]   {}", question_id, group_id, category, line) }
                }
                else
                {
                    line.to_string()
                };
                let mut run = body_run.clone().add_text(txt).size(body_font_size);
                if lines.peek().is_some()
                    { run = run.add_break(docx_rs::BreakType::TextWrapping); }
                q_para = q_para.add_run(run);
                is_first_line = false;
            }
            docx = docx.add_paragraph(q_para);

            let category_id = question.get_category();
            match category_id
            {
                3 => {
                    // Short answer: ( space * 3 * max_choice_len )
                    let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                    let spaces = " ".repeat(max_len * 3);
                    let c_para = paragraph(body_run.clone(), format!("    ({})", spaces), body_font_size);
                    docx = docx.add_paragraph(c_para);
                },
                4 => {
                    // Essay: 15 blank lines
                    for _ in 0..15
                        { docx = docx.add_paragraph(blank_line.clone()); }
                },
                _ => {
                    // Category 1, 2: Standard choices
                    for (choice_index, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
                    {
                        let mut c_para = Paragraph::new();
                        let mut lines = choice_text.lines().peekable();
                        let mut is_first_line = true;
                        while let Some(line) = lines.next()
                        {
                            let mut run = body_run.clone().add_text(if is_first_line { format!("    ({}) {}", choice_index + 1, line) } else { line.to_string() }).size(body_font_size);
                            if lines.peek().is_some()
                                { run = run.add_break(docx_rs::BreakType::TextWrapping); }
                            c_para = c_para.add_run(run);
                            is_first_line = false;
                        }
                        docx = docx.add_paragraph(c_para);
                    }
                },
            }
            // Blank line after each question
            docx = docx.add_paragraph(blank_line.clone());
        }
        Ok(docx)
    }

    fn write_answer_sheet_to_docx(&self, mut docx: Docx) -> Result<Docx, String>
    {
        let pt_to_usize = |pt: f32| -> usize { (pt as usize) << 1 };
        let linespacing_to_twips = |linespacing: f32| -> i32 { (linespacing * 240.0) as i32 };

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
        docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
        docx = docx.add_paragraph(Paragraph::new()
                                    .add_run(title_run.clone()
                                            .add_text(self.answer_sheet_title.as_str())
                                            .size(title_font_size)
                                    )
                                    .align(AlignmentType::Center)); // 14 pt for default font size
        docx = docx.add_paragraph(Paragraph::new()); // Blank line

        let answer_sheet_font_size = pt_to_usize(self.answer_sheet_font_size);
        let mut answer_sheet_run = Run::new();
        if self.is_answer_sheet_bold()
            { answer_sheet_run = answer_sheet_run.bold(); }
        if self.is_answer_sheet_italic()
            { answer_sheet_run = answer_sheet_run.italic(); }
        if self.is_answer_sheet_underline()
            { answer_sheet_run = answer_sheet_run.underline("single"); }
        if self.is_answer_sheet_strike()
            { answer_sheet_run = answer_sheet_run.strike(); }
        
        let line_spacing = linespacing_to_twips(self.line_spacing);

        // Answers
        let mut answers_text = String::new();
        for (i, question) in self.qbank.get_questions().iter().enumerate() {
            let category_id = question.get_category();
            let answer_string = match category_id
            {
                3 => {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                },
                4 => { "(---)".to_string() },
                _ => {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                },
            };
            answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
        }

        let answers_paragraph = Paragraph::new()
                                    .add_run(answer_sheet_run.clone()
                                                .add_text(answers_text)
                                                .size(answer_sheet_font_size)
                                            ) // 12 pt for default answer sheet font size
                                    .line_spacing(docx_rs::LineSpacing::new().line(line_spacing));   // Single line spacing
        docx = docx.add_paragraph(answers_paragraph);

        Ok(docx)
    }
    
    // pub fn save_qbank_in_hwpx(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    /// Saves the question bank to a HWPX file.
    ///
    /// This function generates a HWPX document containing the question bank,
    /// with specified page margins and a footer with page numbers.
    /// 
    /// # Arguments
    /// * `path` - The file path where the HWPX document will be saved.
    /// * `is_with_answer_sheet` indicates whether it is printed
    ///   with an answer sheet.
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
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let result = exporter.save_qbank_in_hwpx(Path::new("question_bank.hwpx"), true);
    /// assert!(result.is_ok());
    /// std::fs::remove_file("question_bank.hwpx").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_qbank_in_hwpx(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    {
        self.build_qbank_in_hwpx(is_with_answer_sheet)?
            .save_to_file(path)
            .map_err(|e| e.to_string())
    }

    // pub fn export_qbank_in_hwpx(&self) -> Result<Vec<u8>, String>
    /// Exports the question bank to a Vec<u8> object.
    ///
    /// This function generates a HWPX document containing the question bank,
    /// with specified page margins and a footer with page numbers,
    /// and returns the document as a Vec<u8> object.
    /// 
    /// # Arguments
    /// * `is_with_answer_sheet` indicates whether it is printed
    ///   with an answer sheet.
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
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let result = exporter.export_qbank_in_hwpx(true);
    /// assert!(result.is_ok());
    /// std::fs::write("question_bank.hwpx", result.unwrap()).unwrap();
    /// std::fs::remove_file("question_bank.hwpx").unwrap();
    /// ```
    pub fn export_qbank_in_hwpx(&self, is_with_answer_sheet: bool) -> Result<Vec<u8>, String>
    {
        self.build_qbank_in_hwpx(is_with_answer_sheet)?
            .to_bytes()
            .map_err(|e| e.to_string())
    }

    fn build_qbank_in_hwpx(&self, is_with_answer_sheet: bool) -> Result<HwpxWriter, String>
    {
        let mut hwpx = self.write_questions_to_hwpx()?;
        if is_with_answer_sheet
            { hwpx = self.write_answer_sheet_to_hwpx(hwpx)?; }

        Ok(hwpx)
    }

    // fn write_questions_to_hwpx(&self) -> Result<HwpxWriter, String>
    /// Writes the formatted exam content for a single student to a HWPX document.
    ///
    /// This private helper function takes a mutable HWPX `HwpxWriter` object and
    /// appends the exam content for the given student and their shuffled
    /// question bank, applying HWPX-specific formatting such as font sizes.
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
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let result = exporter.write_questions_to_hwpx()?;
    /// ```
    /// This example demonstrates how the `write_questions_to_hwpx` function
    /// can be called to write qbank content for a student into a HWPX document.
    /// The actual content of the questions and formatting will depend on the
    /// properties of the `Exporter` instance and the data in the `QBank`.
    /// Note that since `write_questions_to_hwpx` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the HWPX document.
    fn write_questions_to_hwpx(&self) -> Result<HwpxWriter, String>
    {
        let mut hwpx = HwpxWriter::new();
        hwpx.add_footer_with_page_number(" ");

        let header = self.qbank.get_header();
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
            text: header.get_title().to_string(),
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

        let student_info = format!("{}: {}    {}: {}", header.get_name(), Self::DUMMY_STRING, header.get_id(), Self::DUMMY_STRING);
        hwpx.add_mixed_styled_paragraph(vec![StyledText {
            text: student_info,
            style: body_style.clone(),
        }]).map_err(|e| e.to_string())?;

        hwpx.add_paragraph("").map_err(|e| e.to_string())?;

        // Instructions (Handle multi-line notice)
        for line in header.get_notice().lines()
        {
            hwpx.add_mixed_styled_paragraph(vec![StyledText {
                text: line.to_string(),
                style: body_style.clone(),
            }]).map_err(|e| e.to_string())?;
        }
        hwpx.add_paragraph("").map_err(|e| e.to_string())?;

        for (question_index, question) in self.qbank.get_questions().iter().enumerate()
        {
            let question_id = question_index + 1;
            let group_id = question.get_group();
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s| s.as_str()).unwrap_or("");
            let q_text = if question_id == group_id as usize
                { format!("{}. [{}]   {}", question_id, category_text, question.get_question()) }
            else
                { format!("{}{{={}}}. [{}]   {}", question_id, group_id, category_text, question.get_question()) };
            hwpx.add_mixed_styled_paragraph(vec![StyledText {
                text: q_text,
                style: body_style.clone(),
            }]).map_err(|e| e.to_string())?;

            match category_id
            {
                3 => {
                    // Short answer: ( space * 3 * max_choice_len )
                    let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                    let spaces = " ".repeat(max_len * 3);
                    hwpx.add_mixed_styled_paragraph(vec![StyledText {
                        text: format!("    ({})", spaces),
                        style: body_style.clone(),
                    }]).map_err(|e| e.to_string())?;
                },
                4 => {
                    // Essay: 15 blank lines
                    for _ in 0..15
                        { hwpx.add_paragraph("").map_err(|e| e.to_string())?; }
                },
                _ => {
                    // Category 1, 2: Standard choices
                    for (choice_index, (choice_text, _)) in question.get_choices().iter().enumerate()
                    {
                        let c_text = format!("    ({}) {}", choice_index + 1, choice_text);
                        hwpx.add_mixed_styled_paragraph(vec![StyledText {
                            text: c_text,
                            style: body_style.clone(),
                        }]).map_err(|e| e.to_string())?;
                    }
                },
            }
            hwpx.add_paragraph("").map_err(|e| e.to_string())?;
        }

        Ok(hwpx)
    }

    // fn write_answer_sheet_to_hwpx(&self, mut hwpx: HwpxWriter) -> Result<HwpxWriter, String>
    /// Writes the answer sheet for the question bank to a HWPX document.
    ///
    /// This private helper function takes a mutable HWPX `HwpxWriter` object and
    /// appends the answer sheet content, applying HWPX-specific formatting such as font sizes.
    /// 
    /// # Arguments
    /// * `hwpx` - A mutable reference to an `HwpxWriter` object
    ///
    /// # Returns
    /// `Result<HwpxWriter, String>` - Returns `Ok(HwpxWriter)` on success, or
    ///                                 an `Err` with a
    ///                                `String` describing the error on failure.
    /// 
    /// # Cautions
    /// This function is unstable at the moment! Don;t use this.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let mut hwpx = exporter.write_questions_to_hwpx()?;
    /// let result = exporter.write_answer_sheet_to_hwpx(hwpx)?;
    /// ```
    /// This example demonstrates how the `write_questions_to_hwpx` function
    /// can be called to write qbank content for a student into a HWPX document.
    /// The actual content of the questions and formatting will depend on the
    /// properties of the `Exporter` instance and the data in the `QBank`.
    /// Note that since `write_questions_to_hwpx` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the HWPX document.
    fn write_answer_sheet_to_hwpx(&self, mut hwpx: HwpxWriter) -> Result<HwpxWriter, String>
    {
        // Add answer sheet
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
            text: self.answer_sheet_title.clone(),
            style: title_style,
        }]).map_err(|e| e.to_string())?;

        hwpx.add_paragraph("").map_err(|e| e.to_string())?;

        let mut answer_sheet_style = HwpxTextStyle::new().size(self.answer_sheet_font_size.round() as u32);
        if self.is_answer_sheet_bold()
            { answer_sheet_style = answer_sheet_style.bold(); }
        if self.is_answer_sheet_italic()
            { answer_sheet_style = answer_sheet_style.italic(); }
        if self.is_answer_sheet_underline()
            { answer_sheet_style = answer_sheet_style.underline(); }
        if self.is_answer_sheet_strike()
            { answer_sheet_style = answer_sheet_style.strikethrough(); }

        let mut answers_text = String::new();
        for (i, question) in self.qbank.get_questions().iter().enumerate()
        {
            let category_id = question.get_category();
            let answer_string = match category_id
            {
                3 => {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                },
                4 => { "(---)".to_string() },
                _ => {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                },
            };
            answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
        }
        hwpx.add_mixed_styled_paragraph(vec![StyledText {
            text: answers_text,
            style: answer_sheet_style.clone(),
        }]).map_err(|e| e.to_string())?;
        hwpx.add_paragraph("").map_err(|e| e.to_string())?;

        Ok(hwpx)
    }

    // pub fn save_qbank_in_hwp(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    /// Saves the question bank to a HWP file.
    ///
    /// This function generates a HWP document containing the question bank,
    /// with specified page margins and a footer with page numbers.
    /// 
    /// # Arguments
    /// * `path` - The file path where the HWP document will be saved.
    /// * `is_with_answer_sheet` indicates whether it is printed
    ///   with an answer sheet.
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
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let result = exporter.save_qbank_in_hwp(Path::new("question_bank.hwp"), true);
    /// assert!(result.is_ok());
    /// std::fs::remove_file("question_bank.hwp").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_qbank_in_hwp(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    {
        self.build_qbank_in_hwp(is_with_answer_sheet)?
            .save_to_file(path)
            .map_err(|e| e.to_string())
    }

    // pub fn export_qbank_in_hwp(&self, is_with_answer_sheet: bool) -> Result<Vec<u8>, String>
    /// Exports the question bank to a Vec<u8> object.
    ///
    /// This function generates a HWP document containing the question bank,
    /// with specified page margins and a footer with page numbers,
    /// and returns the document as a Vec<u8> object.
    /// 
    /// # Arguments
    /// * `is_with_answer_sheet` indicates whether it is printed
    ///   with an answer sheet.
    /// 
    /// # Output
    /// `Result<Vec<u8>, String>` - Returns `Ok(Vec<u8>)` containing the HWP
    ///                             document on success, or an `Err` with a
    ///                             `String` describing the error on failure.
    /// 
    /// # Cautions
    /// This function is unstable at the moment! Don;t use this.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let result = exporter.export_qbank_in_hwp(true);
    /// assert!(result.is_ok());
    /// std::fs::write("question_bank.hwp", result.unwrap()).unwrap();
    /// std::fs::remove_file("question_bank.hwp").unwrap();
    /// ```
    pub fn export_qbank_in_hwp(&self, is_with_answer_sheet: bool) -> Result<Vec<u8>, String>
    {
        self.build_qbank_in_hwp(is_with_answer_sheet)?
            .to_bytes()
            .map_err(|e| e.to_string())
    }

    fn build_qbank_in_hwp(&self, is_with_answer_sheet: bool) -> Result<HwpWriter, String>
    {
        let mut hwp = self.write_questions_to_hwp()?;
        if is_with_answer_sheet
            { hwp = self.write_answer_sheet_to_hwp(hwp)?; }

        Ok(hwp)
    }

    // fn write_questions_to_hwp(&self) -> Result<HwpWriter, String>
    /// Writes the question bank content to a HWP document.
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
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let mut hwp = hwpers::HwpWriter::new();
    /// let result = exporter.write_questions_to_hwp();
    /// ```
    /// This example demonstrates how the `write_questions_to_hwp` function
    /// can be called to write question bank content into a HWP document.
    /// The actual content of the questions and formatting will depend on the
    /// properties of the `Exporter` instance and the data in the `QBank`.
    /// Note that since `write_questions_to_hwp` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the HWP document.
    fn write_questions_to_hwp(&self) -> Result<HwpWriter, String>
    {
        let mut hwp = HwpWriter::new();
        hwp.set_page_margins_mm(
            self.margin_left_in_mm,
            self.margin_right_in_mm,
            self.margin_top_in_mm,
            self.margin_buttom_in_mm,
        );
        hwp.add_footer_with_page_number(" ", hwpers::model::PageNumberFormat::Numeric);
        let header = self.qbank.get_header();
        let mut title_style = TextStyle::new().size(self.title_font_size.round() as u32);
        if self.is_title_bold()
            { title_style = title_style.bold(); }
        if self.is_title_italic()
            { title_style = title_style.italic(); }
        if self.is_title_underline()
            { title_style = title_style.underline(); }
        if self.is_title_strike()
            { title_style = title_style.strikethrough(); }

        let title_text = header.get_title();
        let mut title_styled = hwpers::writer::style::StyledText::new(title_text.clone());
        title_styled = title_styled.add_range(0, title_text.len(), title_style);
        hwp.add_styled_paragraph(&title_styled).map_err(|e| e.to_string())?;

        let mut body_style = TextStyle::new().size(self.body_font_size.round() as u32);
        if self.is_title_bold()
            { body_style = body_style.bold(); }
        if self.is_title_italic()
            { body_style = body_style.italic(); }
        if self.is_title_underline()
            { body_style = body_style.underline(); }
        if self.is_title_strike()
            { body_style = body_style.strikethrough(); }

        let student_info = format!("{}: {}\t\t{}: {}", header.get_name(), Self::DUMMY_STRING, header.get_id(), Self::DUMMY_STRING);
        let mut student_info_styled = hwpers::writer::style::StyledText::new(student_info.clone());
        student_info_styled = student_info_styled.add_range(0, student_info.len(), body_style.clone());
        hwp.add_styled_paragraph(&student_info_styled).map_err(|e| e.to_string())?;

        hwp.add_paragraph("").map_err(|e| e.to_string())?;

        // Instructions (Handle multi-line notice)
        for line in header.get_notice().lines()
        {
            let mut line_styled = hwpers::writer::style::StyledText::new(line.to_string());
            line_styled = line_styled.add_range(0, line.len(), body_style.clone());
            hwp.add_styled_paragraph(&line_styled).map_err(|e| e.to_string())?;
        }
        hwp.add_paragraph("").map_err(|e| e.to_string())?;

        for (question_index, question) in self.qbank.get_questions().iter().enumerate()
        {
            let question_id = question_index + 1;
            let group_id = question.get_group();
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s| s.as_str()).unwrap_or("");
            let q_text = if question_id == group_id as usize
                { format!("{}. [{}]   {}", question_id, category_text, question.get_question()) }
            else
                { format!("{}{{={}}}. [{}]   {}", question_id, group_id, category_text, question.get_question()) };
            let mut q_styled = hwpers::writer::style::StyledText::new(q_text.clone());
            q_styled = q_styled.add_range(0, q_text.len(), body_style.clone());
            hwp.add_styled_paragraph(&q_styled).map_err(|e| e.to_string())?;

            match category_id
            {
                3 => {
                    // Short answer: ( space * 3 * max_choice_len )
                    let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                    let spaces = " ".repeat(max_len * 3);
                    let c_text = format!("    ({})", spaces);
                    let mut c_styled = hwpers::writer::style::StyledText::new(c_text.clone());
                    c_styled = c_styled.add_range(0, c_text.len(), body_style.clone());
                    hwp.add_styled_paragraph(&c_styled).map_err(|e| e.to_string())?;
                },
                4 => {
                    // Essay: 15 blank lines
                    for _ in 0..15
                        { hwp.add_paragraph("").map_err(|e| e.to_string())?; }
                },
                _ => {
                    // Category 1, 2: Standard choices
                    for (choice_index, (choice_text, _)) in question.get_choices().iter().enumerate()
                    {
                        let c_text = format!("    ({}) {}", choice_index + 1, choice_text);
                        let mut c_styled = hwpers::writer::style::StyledText::new(c_text.clone());
                        c_styled = c_styled.add_range(0, c_text.len(), body_style.clone());
                        hwp.add_styled_paragraph(&c_styled).map_err(|e| e.to_string())?;
                    }
                },
            }
            hwp.add_paragraph("").map_err(|e| e.to_string())?;
        }
        Ok(hwp)
    }

    fn write_answer_sheet_to_hwp(&self, mut hwp: HwpWriter) -> Result<HwpWriter, String>
    {
        // Add answer sheet
        let mut title_style = TextStyle::new().size(self.title_font_size.round() as u32);
        if self.is_title_bold()
            { title_style = title_style.bold(); }
        if self.is_title_italic()
            { title_style = title_style.italic(); }
        if self.is_title_underline()
            { title_style = title_style.underline(); }
        if self.is_title_strike()
            { title_style = title_style.strikethrough(); }

        let mut title_styled = hwpers::writer::style::StyledText::new(self.answer_sheet_title.clone());
        title_styled = title_styled.add_range(0, self.answer_sheet_title.len(), title_style);
        hwp.add_styled_paragraph(&title_styled).map_err(|e| e.to_string())?;
        hwp.add_paragraph("").map_err(|e| e.to_string())?;

        let mut answer_sheet_style = TextStyle::new().size(self.answer_sheet_font_size.round() as u32);
        if self.is_answer_sheet_bold()
            { answer_sheet_style = answer_sheet_style.bold(); }
        if self.is_answer_sheet_italic()
            { answer_sheet_style = answer_sheet_style.italic(); }
        if self.is_answer_sheet_underline()
            { answer_sheet_style = answer_sheet_style.underline(); }
        if self.is_answer_sheet_strike()
            { answer_sheet_style = answer_sheet_style.strikethrough(); }

        let mut answers_text = String::new();
        for (i, question) in self.qbank.get_questions().iter().enumerate()
        {
            let category_id = question.get_category();
            let answer_string = match category_id
            {
                3 => {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                },
                4 => { "(---)".to_string() },
                _ => {
                    let correct_choices: Vec<String> = question.get_choices()
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, is_correct))| *is_correct)
                    .map(|(j, _)| (j + 1).to_string())
                    .collect();
                    format!("({})", correct_choices.join(", "))
                },
            };
            answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
        }
        let mut answers_styled = hwpers::writer::style::StyledText::new(answers_text.clone());
        answers_styled = answers_styled.add_range(0, answers_text.len(), answer_sheet_style.clone());
        hwp.add_styled_paragraph(&answers_styled).map_err(|e| e.to_string())?;
        hwp.add_paragraph("").map_err(|e| e.to_string())?;

        Ok(hwp)
    }

    // pub fn save_qbank_in_pdf(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    /// Saves the question bank to a PDF file.
    ///
    /// This function generates a PDF document containing the question bank,
    /// with a footer showing page numbers.
    ///
    /// # Arguments
    /// * `path` - The file path where the PDF document will be saved.
    /// * `is_with_answer_sheet` indicates whether it is printed
    ///   with an answer sheet.
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
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    ///
    /// let result = exporter.save_shuffled_exams_in_pdf(Path::new("question_bank.pdf"), true);
    /// assert!(result.is_ok());
    /// std::fs::remove_file("question_bank.pdf").unwrap();
    /// ```
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn save_qbank_in_pdf(&self, path: &Path, is_with_answer_sheet: bool) -> Result<(), String>
    {
        let mut doc = self.save_questions_to_pdf()?;
        if is_with_answer_sheet
            { doc = self.save_answer_sheet_to_pdf(doc); }
        doc.render_to_file(path).map_err(|e: Error| e.to_string())?;

        Ok(())
    }

    // fn save_questions_to_pdf(&self, mut doc: Document) -> Result<Document, String>
    /// Writes the question bank content for a single student to a PDF document.
    ///
    /// This private helper function takes a mutable PDF `Document` object
    /// and appends the question bank content, applying PDF-specific formatting
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
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let mut doc = genpdfi::Document::new(genpdfi::fonts::from_files("./fonts", "font", None).unwrap());
    /// let result = exporter.write_exam_content_to_pdf(&mut doc, &student1, &qbank);
    /// ```
    /// This example demonstrates how the `save_questions_to_pdf` function
    /// can be called to write question bank content for a student into a PDF
    /// document.
    /// The actual content of the questions and formatting will depend on the
    /// properties of the `Exporter` instance and the data in the `QBank`.
    /// Note that since `save_questions_to_pdf` is a private helper
    /// function, it is not intended to be called directly in typical usage.
    /// Instead, it is used internally by the public methods that generate
    /// the PDF document.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn save_questions_to_pdf(&self) -> Result<Document, String>
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
        let header = self.qbank.get_header();

        let mut title_style = style::Style::new();
        title_style.set_font_size(title_font_size);
        if self.is_title_bold()
            { title_style.set_bold(); }
        if self.is_title_italic()
            { title_style.set_italic(); }

        // Exam Title
        let mut title_paragraph = elements::Paragraph::new(format!("{}", header.get_title()));
        title_paragraph.set_alignment(Alignment::Center);
        doc.push(title_paragraph.styled(title_style));

        let mut body_style = style::Style::new();
        body_style.set_font_size(normal_font_size);
        if self.is_body_bold()
            { body_style.set_bold(); }
        if self.is_body_italic()
            { body_style.set_italic(); }
        
        // Student Information
        doc.push(elements::Paragraph::new(format!("{}: {}    {}: {}", header.get_name(), Self::DUMMY_STRING, header.get_id(), Self::DUMMY_STRING)).styled(body_style));
        doc.push(elements::Paragraph::new("")); // Blank line

        // Instructions (Handle multi-line notice)
        for line in header.get_notice().lines()
            { doc.push(elements::Paragraph::new(format!("{}", line)).styled(body_style)); }
        doc.push(elements::Paragraph::new("")); // Blank line after notice

        for (question_index, question) in self.qbank.get_questions().iter().enumerate()
        {
            let question_id = question_index + 1;
            let group_id = question.get_group();
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s| s.as_str()).unwrap_or("");
            let txt = if question_id == group_id as usize
                { format!("{}. [{}]   {}", question_id, category_text, question.get_question()) }
            else
                { format!("{}{{={}}}. [{}]   {}", question_id, group_id, category_text, question.get_question()) };
            doc.push(elements::Paragraph::new(txt).styled(body_style));
            
            match category_id
            {
                3 => {
                    // Short answer: ( space * 3 * max_choice_len )
                    let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                    let spaces = " ".repeat(max_len * 3);
                    doc.push(elements::Paragraph::new(format!("    ({})", spaces)).styled(body_style));
                },
                4 => {
                    // Essay: 15 blank lines
                    for _ in 0..15
                        { doc.push(elements::Paragraph::new("")); }
                },
                _ => {
                    // Category 1, 2: Standard choices
                    for (choice_index, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
                    {
                        let choice_char = (choice_index + 1).to_string();
                        doc.push(elements::Paragraph::new(format!("    ({}) {}", choice_char, choice_text)).styled(body_style));
                    }
                },
            }
            doc.push(elements::Paragraph::new("")); // Blank line after each question
        }
        Ok(doc)
    }

    fn save_answer_sheet_to_pdf(&self, mut doc: Document) -> Document
    {
        // Add answer sheet
        doc.push(elements::PageBreak::new());
        let mut answer_style = style::Style::new();
        answer_style.set_font_size(self.answer_sheet_font_size as u8);
        if self.is_answer_sheet_bold()
            { answer_style.set_bold(); }
        if self.is_answer_sheet_italic()
            { answer_style.set_italic(); }
        let mut answer_title_style = style::Style::new();
        answer_title_style.set_font_size(self.title_font_size as u8);
        if self.is_title_bold()
            { answer_title_style.set_bold(); }
        if self.is_title_italic()
            { answer_title_style.set_italic(); }
        
        let mut title_paragraph = elements::Paragraph::new(self.answer_sheet_title.clone());
        title_paragraph.set_alignment(Alignment::Center);
        doc.push(title_paragraph.styled(answer_title_style));
        doc.push(elements::Paragraph::new("")); // Blank line

        let header = self.qbank.get_header();

        // Student Info
        let student_info_text = format!("{}: {}    {}: {}", header.get_name(), Self::DUMMY_STRING, header.get_id(), Self::DUMMY_STRING);
        doc.push(elements::Paragraph::new(student_info_text).styled(answer_style));

        // Answers
        let mut answers_text = String::new();
        for (i, question) in self.qbank.get_questions().iter().enumerate()
        {
            let category_id = question.get_category();
            let answer_string = match category_id
            {
                3 => {
                    let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                    format!("({})", answers.join(", "))
                },
                4 => { "(---)".to_string() },
                _ => {
                    let correct_choices: Vec<String> = question.get_choices()
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, is_correct))| *is_correct)
                        .map(|(j, _)| (j + 1).to_string())
                        .collect();
                    format!("({})", correct_choices.join(", "))
                },
            };
            answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
        }
        doc.push(elements::Paragraph::new(answers_text).styled(answer_style));
        doc.push(elements::Paragraph::new("")); // Blank line

        doc
    }

    // pub fn export_qbank_in_pdf(&self, is_with_answer_sheet: bool) -> Result<Vec<u8>, String>
    /// Exports the question bank as a PDF-compatible JSON byte vector.
    ///
    /// This function generates a JSON structure that can be used with the `pdfmake`
    /// library ot javascript to create a PDF document.
    /// It includes the question bankfor all students and an answer sheet.
    ///
    /// # Returns
    /// `Result<Vec<u8>, String>` - Returns a byte vector containing the JSON string
    ///                            on success, or an `Err` with a `String`
    ///                            describing the error on failure.
    /// 
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Exporter, Question };
    /// use std::fs;
    /// use std::path::Path;
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    /// let exporter = Exporter::new(&qbank, "Answer Sheet".to_string());
    /// let result = exporter.export_shuffled_exams_in_pdf();
    /// assert!(result.is_ok());
    /// std::fs::write("exam_exported.json", result.unwrap()).unwrap();
    /// std::fs::remove_file("exam_exported.json").unwrap();
    /// ```
    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    pub fn export_qbank_in_pdf(&self, is_with_answer_sheet: bool) -> Result<Vec<u8>, String>
    {
        let mut content = self.export_questions_to_pdf();
        if is_with_answer_sheet
            { content = self.export_answer_sheet_to_pdf(content); }

        Ok(content)
    }

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    fn export_questions_to_pdf(&self) -> Vec<u8>
    {
        use serde_json::json;
        let mut content = Vec::new();

        let title_font_size = self.title_font_size;
        let body_font_size = self.body_font_size;
        let answer_sheet_font_size = self.answer_sheet_font_size;

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

        let answer_sheet_style = json!({
            "fontSize": answer_sheet_font_size,
            "bold": self.is_answer_sheet_bold(),
            "italics": self.is_answer_sheet_italic(),
            "decoration": if self.is_answer_sheet_underline() { Some("underline") } else if self.is_answer_sheet_strike() { Some("lineThrough") } else { None },
            "margin": [0, 2, 0, 2]
        });


        let student: &Student = student;
        let qbank: &QBank = qbank;
        if idx > 0
            { content.push(json!({"text": "", "pageBreak": "before"})); }

        let header = qbank.get_header();
        
        // Title
        content.push(json!({"text": header.get_title(), "style": "title"}));

        // Student Info
        content.push(json!({
            "text": format!("{}: {}        {}: {}", header.get_name(), student.get_name(), header.get_id(), student.get_id()),
            "style": "body"
        }));
        content.push(json!({"text": "\n"}));

        // Notice
        for line in header.get_notice().lines()
            { content.push(json!({"text": line, "style": "body"})); }
        content.push(json!({"text": "\n"}));

        // Questions
        for (question_index, question) in self.qbank.get_questions().iter().enumerate()
        {
            let question_id = question_index + 1;
            let group_id = question.get_group();
            let category_id = question.get_category();
            let category_text = header.get_category(category_id).map(|s: &String| s.as_str()).unwrap_or("");
            if question_id == group_id as usize
            {
                content.push(json!({
                                "text": format!("{}. [{}]   {}", question_id, category_text, question.get_question()),
                                "style": "body"
                }));
            }
            else
            {
                content.push(json!({
                                "text": format!("{}{{={}}}. [{}]   {}", question_id, group_id, category_text, question.get_question()),
                                "style": "body"
                }));
            }

            match category_id
            {
                3 => {
                    // Short answer: ( space * 3 * max_choice_len )
                    let max_len = question.get_choices().iter().map(|(t, _)| t.len()).max().unwrap_or(0);
                    let spaces = " ".repeat(max_len * 3);
                    content.push(json!({
                        "text": format!("    ({})", spaces),
                        "style": "body"
                    }));
                },
                4 => {
                    // Essay: 15 blank lines
                    for _ in 0..15
                        { content.push(json!({"text": "\n"})); }
                },
                _ => {
                    // Category 1, 2: Standard choices
                    for (choice_index, (choice_text, _)) in question.get_choices().iter().enumerate()
                    {
                        content.push(json!({
                            "text": format!("    ({}) {}", choice_index + 1, choice_text),
                            "style": "body"
                        }));
                    }
                },
            }
            content.push(json!({"text": "\n"}));
        }
        content
    }

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    fn export_answer_sheet_to_pdf(&self, mut content: Vec<u8>) -> Vec<u8>
    {
        // Answer Sheet
        content.push(json!({"text": "", "pageBreak": "before"}));
        content.push(json!({"text": self.answer_sheet_title.as_str(), "style": "title"}));

        let header = self.qbank.get_header();
        for (student, qbank) in &shuffled_qsets
        {
            let mut answers_text = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate()
            {
                let category_id = question.get_category();
                let answer_string = match category_id
                {
                    3 => {
                        let answers: Vec<String> = question.get_choices().iter().map(|(t, _)| t.clone()).collect();
                        format!("({})", answers.join(", "))
                    },
                    4 => { "(---)".to_string() },
                    _ => {
                        let correct_choices: Vec<String> = question.get_choices()
                            .iter()
                            .enumerate()
                            .filter(|(_, (_, is_correct)): &(usize, &(String, bool))| *is_correct)
                            .map(|(j, _): (usize, &(String, bool))| (j + 1).to_string())
                            .collect();
                        format!("({})", correct_choices.join(", "))
                    },
                };
                answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
            }
            content.push(json!({"text": answers_text, "style": "answer_sheet"}));
            content.push(json!({"text": "\n"}));
        }

        let doc_definition = json!({
            "content": content,
            "styles": {
                "title": title_style,
                "body": body_style,
                "answer_sheet": answer_sheet_style
            }
        });

        serde_json::to_vec(&doc_definition).map_err(|e| e.to_string())
    }
}