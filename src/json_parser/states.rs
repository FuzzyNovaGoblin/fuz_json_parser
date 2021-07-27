pub enum BlockType {
    Array,
    Object,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InQuotes {
    False,
    Single,
    Double,
}

impl InQuotes {
    pub fn is_insided(&self) -> bool {
        match self {
            InQuotes::False => false,
            InQuotes::Single | InQuotes::Double => true,
        }
    }

    pub fn _matches_corisponding(&self, quote_char: char) -> bool {
        match self {
            InQuotes::False => false,
            InQuotes::Single => quote_char == '\'',
            InQuotes::Double => quote_char == '\"',
        }
    }
}

impl Default for InQuotes {
    fn default() -> Self {
        InQuotes::False
    }
}
