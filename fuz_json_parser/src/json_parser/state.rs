use std::{iter::Peekable, str::Chars};

use crate::error::Result;

/// state of parseing function,
/// holds cursor and string to be parsed
#[derive(Debug)]
pub struct ParserState<'a> {
    pos: usize,
    char_itter: Peekable<Chars<'a>>,
}

impl<'a> ParserState<'a> {
    pub fn new(char_itter: Chars<'a>) -> ParserState<'a> {

        ParserState::<'a> {
            pos: 0,
            char_itter: char_itter.peekable(),
        }
    }

    /// returns the `pos` value from the parser state
    pub fn get_pos(&self) -> usize {

        self.pos
    }

    /// look at the current charater
    pub fn peek(&mut self) -> Option<char> {

        self.char_itter.peek().copied()
    }

    /// return the current character and cursor to the next position
    pub fn advance(&mut self) -> Option<char> {

        self.pos += 1;
        self.char_itter.next()
    }

    /// assert that the current character is the expected character `c`
    ///
    /// if `ignore_case` is `true` the check will be preformed without considering the case of the character
    pub fn assert_char(&mut self, mut c: char, ignore_case: bool) -> Result<()> {

        if ignore_case {
            c = c.to_ascii_lowercase();
        }

        if let Some(mut advance_value) = self.advance() {
            if ignore_case {
                advance_value = advance_value.to_ascii_lowercase();
                advance_value = advance_value.to_ascii_lowercase();
            }
            if advance_value != c {
                Err(format!(
                    "`{advance_value}` is not equal to `{c}` at position {}",
                    self.get_pos()
                )
                .into())
            } else {
                Ok(())
            }
        } else {
            Err("No char returned".into())
        }
    }

    /// check if the current character matches a character `c`
    ///
    /// similar to [assert_char] but only consumes the character if it does match, useful for control flow
    ///
    /// if `ignore_case` is `true` the check will be preformed without considering the case of the character
    pub fn check_char(&mut self, check_against_char: char) -> bool {
        //! # Example
        //! instead of using [peek] then [advance] if the returned character was equal, you can
        //! use [check_char] then branch based on the returned value
        //! ```
        //! use fuz_json_parser::{json_parser::state::{ParserState}};
        //!
        //! let mut state = ParserState::new("-127".into());
        //! let is_negative = matches!(state.peek(), Some('-'));
        //!
        //! if is_negative {
        //!     state.advance();
        //! }
        //! ```
        //! vs
        //! ```
        //! use fuz_json_parser::{json_parser::state::ParserState};
        //!
        //! let mut state = ParserState::new("-127".into());
        //! let is_negative = state.check_char('-');
        //! ```


        match self.peek() {
            Some(c) if c == check_against_char => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    /// uses [assert_char] to assert that the next characters are equal to the provided string
    pub fn assert_string<S: AsRef<str>>(&mut self, string: S, ignore_case: bool) -> Result<()> {

        for c in string.as_ref().chars() {
            if let Err(e) = self.assert_char(c, ignore_case) {
                return Err(format!("failed assert expected \"{}\"\n{e}", string.as_ref()).into());
            }
        }
        Ok(())
    }

    /// create an integer from digits in [state](ParserState)
    ///
    /// consumes characters in the [state](ParserState) until a
    /// character that is not a digit is found
    ///
    /// the returned value is a [String] containing all the digits
    pub fn consume_number(&mut self) -> String {

        let char_itter = &mut self.char_itter;

        let ret_string = char_itter
            .take(
                char_itter
                    .clone()
                    .take_while(|c| is_number_part(*c))
                    .count(),
            )
            .collect::<String>();

        self.pos += ret_string.len();

        ret_string
    }

    /// move cursor t next character that is not whitespace
    pub fn consume_whitespace(&mut self) {

        while self.is_whitespace() {
            self.advance();
        }
    }

    /// check if the character at the cursor is white space used by [consume_whitespace]
    pub fn is_whitespace(&mut self) -> bool {
        
        matches!(self.peek(), Some(' ' | '\t' | '\n'))
    }
}

/// check if the character at the cursor is a digit used by [consume_number]
fn is_number_part(character: char) -> bool {

    match character {
        '-' | '.' => true,
        c => c >= (48 as char) && c <= (57 as char),
    }
}
