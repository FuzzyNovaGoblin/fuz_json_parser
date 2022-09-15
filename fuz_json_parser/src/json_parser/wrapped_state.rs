use crate::error::Result;
use std::{iter::ArrayChunks, str::Chars};

/// state of parseing function,
/// holds cursor and string to be parsed
pub struct ParserState<'a, const N: usize> {
    chars: Vec<char>,
    last_chars: Vec<char>,
    pos: usize,
    offset: usize,
    chunks_iter: Option<ArrayChunks<Chars<'a>, N>>,
}

impl<'a, const N: usize> ParserState<'a, N> {
    pub fn new(chunks_iter: ArrayChunks<Chars<'a>, N>) -> ParserState<'a, N> {
        let mut ret = ParserState {
            pos: 0,
            chunks_iter: Some(chunks_iter),
            chars: Vec::new(),
            last_chars: Vec::new(),
            offset: 0,
        };
        ret.next_array_chunck();
        ret
    }

    pub fn total_pos(&self) -> usize {
        self.offset + self.pos
    }

    /// look at the current charater
    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    /// return the current character and cursor to the next position
    pub fn advance(&mut self) -> Option<char> {

        let ret_char = self.chars.get(self.pos).copied();

        self.pos += 1;
        self.next_array_chunck();

        ret_char
    }

    fn next_array_chunck(&mut self) {
        if self.pos >= self.chars.len() {
            if let Some(mut chunks_iter) = self.chunks_iter.take() {
                self.offset += self.pos;
                self.pos = 0;
                if let Some(next_chunk) = chunks_iter.next() {
                    self.last_chars = std::mem::replace(&mut self.chars, next_chunk.into());
                    self.chunks_iter.replace(chunks_iter);
                } else if let Some(remaining) = chunks_iter.into_remainder() {
                    self.last_chars = std::mem::replace(&mut self.chars, remaining.collect());
                }
            }
        }
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
                Err(format!("`{advance_value}` is not equal to `{c}`").into())
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
        //! use fuz_json_parser::{create_state, json_parser::state::{ParserState, advance, peek}};
        //!
        //! let mut state:ParserState = create_state!("-127");
        //! let is_negative = matches!(peek(&mut state), Some('-'));
        //!
        //! if is_negative {
        //!     advance(&mut state);
        //! }
        //! ```
        //! vs
        //! ```
        //! use fuz_json_parser::{create_state, json_parser::state::{check_char, ParserState}};
        //!
        //! let mut state:ParserState = create_state!("-127");
        //! let is_negative = check_char(&mut state, '-');
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
        let start = self.pos;
        let start_offset = self.offset;
        while self.peek().map_or(false, |c| is_number_part(c)) {
            self.advance();
        }

        if start_offset != self.offset {
            self.last_chars[start..]
                .iter()
                .chain(self.chars[..self.pos].iter())
                .collect()
        } else {
            self.chars[start..self.pos].iter().collect()
        }
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
