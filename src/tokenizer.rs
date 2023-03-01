struct Tokenizer {
    string: String,
    cursor: Option<T>,
}

impl Tokenizer {
    init(&self, string: &String) {
        self.string = string;
        self.cursor = 0;
    }

    has_more_tokens(&self) -> Bool {
        return self.cursor < self.string.len();
    }

    get_next_token(&self) {
        if self.has_more_tokens() {
            pass
        }
       let mut token_str = self.string[0..cursor];
    }
}