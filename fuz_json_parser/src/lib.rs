#![feature(iter_array_chunks)]
pub mod error;
pub mod json_parser;
pub mod values;

pub use json_parser::parse as json_parse;

#[cfg(test)]
mod tests;
