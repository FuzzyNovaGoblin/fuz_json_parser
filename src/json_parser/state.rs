use crate::error::Result;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

/// state of parseing function,
/// holds cursor and string to be parsed
pub type ParserState<'a> = Peekable<Enumerate<Chars<'a>>>;
// pub type ParserState = Peekable<Enumerate<impl Iterator<Item = char> + Clone>>;

/// create [ParserState] from json string
#[macro_export]
macro_rules! create_state {
    ($json_str:ident) => {
        $json_str.as_ref().chars().enumerate().peekable()
    };
    ($json_str:expr) => {
        $json_str.chars().enumerate().peekable()
    };
}

/// look at the current charater
pub fn peek(state: &mut ParserState) -> Option<char> {
    state.peek().map(|(_, c)| *c)
}

/// return the current character and cursor to the next position
pub fn advance(state: &mut ParserState) -> Option<char> {
    state.next().map(|(_, c)| c)
}

/// assert that the current character is the expected character `c`
pub fn assert_char(state: &mut ParserState, mut c: char, ignore_case: bool) -> Result<()> {
    //!
    //! if `ignore_case` is `true` the check will be preformed without considering the case of the character
    if ignore_case {
        c = c.to_ascii_lowercase();
    }

    if let Some(mut advance_value) = advance(state) {
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
pub fn check_char(state: &mut ParserState, check_against_char: char) -> bool {
    //!
    //! similar to [assert_char] but only consumes the character if it does match, useful for control flow
    //!
    //! if `ignore_case` is `true` the check will be preformed without considering the case of the character
    //!
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

    match peek(state) {
        Some(c) if c == check_against_char => {
            advance(state);
            true
        }
        _ => false,
    }
}

/// uses [assert_char] to assert that the next characters are equal to the provided string
pub fn assert_string<S: AsRef<str>>(
    state: &mut ParserState,
    string: S,
    ignore_case: bool,
) -> Result<()> {
    for c in string.as_ref().chars() {
        if let Err(e) = assert_char(state, c, ignore_case) {
            return Err(format!("failed assert expected \"{}\"\n{e}", string.as_ref()).into());
        }
    }
    Ok(())
}

/// create an integer from digits in [state](ParserState)
pub fn consume_number(state: &mut ParserState) -> String {
    //!
    //! consumes characters in the [state](ParserState) until a
    //! character that is not a digit is found
    //!
    //! the returned value is a [String] containing all the digits

    state
        .take(
            state
                .clone()
                .take_while(|(_, c)| is_number_part(*c))
                .count(),
        )
        .map(|(_, c)| c)
        .collect::<String>()
}

/// check if the character at the cursor is a digit used by [consume_number]
fn is_number_part(character: char) -> bool {
    match character {
        '-' | '.' => true,
        c => c >= (48 as char) && c <= (57 as char),
    }
}

/// move cursor t next character that is not whitespace
pub fn consume_whitespace(state: &mut ParserState) {
    while is_whitespace(state) {
        advance(state);
    }
}

/// check if the character at the cursor is white space used by [consume_whitespace]
pub fn is_whitespace(state: &mut ParserState) -> bool {
    matches!(peek(state), Some(' ' | '\t' | '\n'))
}
