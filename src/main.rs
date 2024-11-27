fn main() {
    let code = std::fs::read_to_string(String::from("main.lpl")).unwrap();

    let mut lexical_analyzer = new_lexical_analyzer(code);
    lexical_analyzer.init();
    let lexemes = lexical_analyzer.lexemes.clone();
    let mut indentation_analyzer = new_indentation_analyzer(lexemes);
    indentation_analyzer.init();
    let lexemes = indentation_analyzer.output.clone();
    println!("{:#?}", lexemes);
}

#[derive(Debug, Clone)]
enum ASTNode {
    Binding(String, Box::<ASTNode>),
    Def(Vec::<String>, Vec::<ASTNode>),
    Copy(Box::<ASTNode>, String, Vec::<ASTNode>),
    Name(String),
}

#[derive(Debug, Clone)]
struct IndentationAnalyzer {
    input: Vec::<Lexeme>,
    output: Vec::<Lexeme>,
    i: usize,
}

impl IndentationAnalyzer {
    fn init(&mut self) {
        let mut indent_level = 0;
        while self.i < self.input.len() {
            let mut this_line_indent_level = 0;
            while self.i < self.input.len()
                && *self.input.get(self.i).unwrap() == Lexeme::Tab {
                this_line_indent_level += 1;
                self.i += 1;
            }
            while indent_level < this_line_indent_level {
                self.output.push(Lexeme::Indent);
                indent_level += 1;
            }
            while indent_level > this_line_indent_level {
                self.output.push(Lexeme::Unindent);
                indent_level -= 1;
            }
            while self.i < self.input.len()
                && *self.input.get(self.i).unwrap() != Lexeme::Newline {
                self.output.push(self.input.get(self.i).unwrap().clone());
                self.i += 1;
            }
            if self.i >= self.input.len() {
                eprintln!("Error: expected newline at the end of every line.");
                panic!("Aborting due to some errors above...");
            }
            assert!(*self.input.get(self.i).unwrap() == Lexeme::Newline);
            self.output.push(Lexeme::Newline);
            self.i += 1;
        }
    }
}

fn new_indentation_analyzer(lexemes: Vec::<Lexeme>) -> IndentationAnalyzer {
    IndentationAnalyzer{
        input: lexemes,
        output: vec![],
        i: 0,
    }
}

#[derive(Debug, Clone)]
struct LexicalAnalyzer {
    input: Vec::<char>,
    lexemes: Vec::<Lexeme>,
    i: usize,
}

fn new_lexical_analyzer(code: String) -> LexicalAnalyzer {
    LexicalAnalyzer{
        input: code.chars().collect(),
        lexemes: vec![],
        i: 0,
    }
}

impl LexicalAnalyzer {
    fn init(&mut self) {
        while self.i < self.input.len() {
            self.generate_lexeme();
        }
    }
    fn char_at(&self, k: usize) -> Option<char> {
        if k >= self.input.len() {
            return None;
        }
        return Some(*self.input.get(k).unwrap());
    }
    fn generate_lexeme(&mut self) {
        if self.i >= self.input.len() {
            return;
        }
        let ch = *self.input.get(self.i).unwrap();
        if ch.is_alphabetic() || ch == '_' {
            self.generate_name();
        } else if ch.is_digit(10) {
            self.generate_number();
        } else if ch == ':' {
            self.i += 1;
            self.lexemes.push(Lexeme::Colon);
        } else if ch == '/' {
            self.i += 1;
            self.lexemes.push(Lexeme::Slash);
        } else if ch == '\t' {
            self.i += 1;
            self.lexemes.push(Lexeme::Tab);
        } else if ch == ' ' {
            self.i += 1;
            if let Some(next_ch) = self.char_at(self.i) {
                if next_ch == ' ' {
                    self.i += 1;
                    self.lexemes.push(Lexeme::Tab);
                    return;
                }
            }
            self.lexemes.push(Lexeme::Space);
        } else if ch == '\n' {
            self.i += 1;
            self.lexemes.push(Lexeme::Newline);
        } else if ch == '\'' {
            self.generate_string();
        } else {
            println!("Unknown character `{}` somewhere in a code.", ch);
            panic!("Aborting due to some errors above...");
        }
    }
    fn generate_name(&mut self) {
        let mut chars = vec![];
        while self.i < self.input.len()
            && (*self.input.get(self.i).unwrap() == '_'
        || self.input.get(self.i).unwrap().is_alphanumeric()) {
            chars.push(*self.input.get(self.i).unwrap());
            self.i += 1;
        }
        self.lexemes.push(Lexeme::Name(chars));
    }
    fn generate_string(&mut self) {
        self.i += 1;
        let mut chars = vec![];
        while self.i < self.input.len()
            && *self.input.get(self.i).unwrap() != '\'' {
            chars.push(*self.input.get(self.i).unwrap());
            self.i += 1;
        }
        if let Some(ch) = self.char_at(self.i) {
            if ch == '\'' {
                self.i += 1;
            }
        } else {
            eprintln!("Unclosed string literal that starts with `{}`.", chars.iter().collect::<String>());
            return;
        }
        self.lexemes.push(Lexeme::String(chars));
    }
    fn generate_number(&mut self) {
        let mut digits = vec![];
        while self.i < self.input.len()
            && self.input.get(self.i).unwrap().is_digit(10) {
            digits.push(*self.input.get(self.i).unwrap());
            self.i += 1;
        }
        self.lexemes.push(Lexeme::Int(
            digits.iter().collect::<String>().parse::<i32>().unwrap()
        ));
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Lexeme {
    Name(Vec::<char>),
    Tab,
    Space,
    Newline,
    Colon,
    Slash,
    Int(i32),
    String(Vec::<char>),
    Indent,
    Unindent,
}

