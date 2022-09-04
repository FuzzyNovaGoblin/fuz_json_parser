#![feature(test)]
#![feature(type_alias_impl_trait)]

pub mod json_parser;
pub mod error;
pub mod values;

pub use json_parser::parse as json_parse;

#[cfg(test)]
mod tests;