#[allow(unused_variables)]
#[allow(dead_code)]

#[derive(Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SettingsBuilder {
    pub fix_yo: bool,
    pub fix_ye: bool,
    pub prefer_yo: bool,
    pub word_a: Option<String>,
    pub word_b: Option<String>,
    pub char_yo: Option<String>,
    pub char_ye: Option<String>,
    pub char_YO: Option<String>,
    pub char_YE: Option<String>,
}

#[derive(Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Settings {
    pub fix_yo: bool,
    pub fix_ye: bool,
    pub prefer_yo: bool,
    pub replace_yo: String,
    pub replace_ye: String,
    pub replace_YO: String,
    pub replace_YE: String,
}

impl SettingsBuilder {
    pub fn new() -> Self {
        Self {
            fix_yo: true,
            fix_ye: true,
            prefer_yo: false,
            word_a: None,
            word_b: None,
            char_yo: None,
            char_ye: None,
            char_YO: None,
            char_YE: None,
        }
    }

    pub fn fix_yo(self, fix_yo: bool) -> Self {
        Self {
            fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo: self.prefer_yo,
            word_a: self.word_a,
            word_b: self.word_b,
            char_yo: self.char_yo,
            char_ye: self.char_ye,
            char_YO: self.char_YO,
            char_YE: self.char_YE,
        }
    }

    pub fn fix_ye(self, fix_ye: bool) -> Self {
        Self {
            fix_yo: self.fix_yo,
            fix_ye,
            prefer_yo: self.prefer_yo,
            word_a: self.word_a,
            word_b: self.word_b,
            char_yo: self.char_yo,
            char_ye: self.char_ye,
            char_YO: self.char_YO,
            char_YE: self.char_YE,
        }
    }

    pub fn prefer_yo(self, prefer_yo: bool) -> Self {
        Self {
            fix_yo: self.fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo,
            word_a: self.word_a,
            word_b: self.word_b,
            char_yo: self.char_yo,
            char_ye: self.char_ye,
            char_YO: self.char_YO,
            char_YE: self.char_YE,
        }
    }

    pub fn word_a(self, word_a: &str) -> Self {
        Self {
            fix_yo: self.fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo: self.prefer_yo,
            word_a: Some(word_a.to_string()),
            word_b: self.word_b,
            char_yo: self.char_yo,
            char_ye: self.char_ye,
            char_YO: self.char_YO,
            char_YE: self.char_YE,
        }
    }

    pub fn word_b(self, word_b: &str) -> Self {
        Self {
            fix_yo: self.fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo: self.prefer_yo,
            word_a: self.word_a,
            word_b: Some(word_b.to_string()),
            char_yo: self.char_yo,
            char_ye: self.char_ye,
            char_YO: self.char_YO,
            char_YE: self.char_YE,
        }
    }

    pub fn char_yo(self, char_yo: &str) -> Self {
        Self {
            fix_yo: self.fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo: self.prefer_yo,
            word_a: self.word_a,
            word_b: self.word_b,
            char_yo: Some(char_yo.to_string()),
            char_ye: self.char_ye,
            char_YO: self.char_YO,
            char_YE: self.char_YE,
        }
    }

    pub fn char_ye(self, char_ye: &str) -> Self {
        Self {
            fix_yo: self.fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo: self.prefer_yo,
            word_a: self.word_a,
            word_b: self.word_b,
            char_yo: self.char_yo,
            char_ye: Some(char_ye.to_string()),
            char_YO: self.char_YO,
            char_YE: self.char_YE,
        }
    }

    #[allow(non_snake_case)]
    pub fn char_YO(self, char_YO: &str) -> Self {
        Self {
            fix_yo: self.fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo: self.prefer_yo,
            word_a: self.word_a,
            word_b: self.word_b,
            char_yo: self.char_yo,
            char_ye: self.char_ye,
            char_YO: Some(char_YO.to_string()),
            char_YE: self.char_YE,
        }
    }

    #[allow(non_snake_case)]
    pub fn char_YE(self, char_YE: &str) -> Self {
        Self {
            fix_yo: self.fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo: self.prefer_yo,
            word_a: self.word_a,
            word_b: self.word_b,
            char_yo: self.char_yo,
            char_ye: self.char_ye,
            char_YO: self.char_YO,
            char_YE: Some(char_YE.to_string()),
        }
    }

    #[allow(non_snake_case)]
    pub fn build(self) -> Settings {
        let char_yo = self.char_yo.unwrap_or("ё".to_string());
        let char_ye = self.char_ye.unwrap_or("е".to_string());
        let char_YO = self.char_YO.unwrap_or("Ё".to_string());
        let char_YE = self.char_YE.unwrap_or("Е".to_string());
        let word_a = self.word_a.unwrap_or(String::new());
        let word_b = self.word_b.unwrap_or(String::new());
        let replace_yo = format!("{}{}{}", &word_a, char_yo, &word_b);
        let replace_ye = format!("{}{}{}", &word_a, char_ye, &word_b);
        let replace_YO = format!("{}{}{}", &word_a, char_YO, &word_b);
        let replace_YE = format!("{}{}{}", &word_a, char_YE, &word_b);
        Settings {
            fix_yo: self.fix_yo,
            fix_ye: self.fix_ye,
            prefer_yo: self.prefer_yo,
            replace_yo,
            replace_ye,
            replace_YO,
            replace_YE,
        }
    }
}
