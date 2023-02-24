/*
    Copyright 2023 Noel Lopes

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
*/

use std::{iter, str::CharIndices};

#[derive(Copy, Clone, PartialEq)]
pub enum AuxiliaryToken {
    BeginSingleLineComment,
    BeginMultiLineComment,
    EndMultiLineComment,
    StrDelimiter,
    CharDelimiter,
    NewLine(usize),
    Other,
}

#[derive(PartialEq)]
pub enum Token {
    SingleLineComment,
    MultiLineComment,
    NewLine(usize),
    Str,
    Char,
    Invalid(String),
    Other,
}

impl Token {
    fn from(token: AuxiliaryToken) -> Token {
        match token {
            AuxiliaryToken::NewLine(line_number) => Token::NewLine(line_number),
            AuxiliaryToken::CharDelimiter => Token::Char,
            AuxiliaryToken::StrDelimiter => Token::Str,
            _ => Token::Other,
        }
    }
}

pub struct Sequence<'a, T: PartialEq> {
    pub token: T,
    pub text: &'a str,
}

pub struct Parser<'a> {
    iterator: iter::Peekable<CharIndices<'a>>,
    next_item: Option<(usize, char)>,
    start_index: usize,
    text: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Parser {
        let mut iterator = text.char_indices().peekable();
        let next_item = iterator.next();

        Parser {
            iterator,
            next_item,
            start_index: 0,
            text,
        }
    }

    fn begin_parsing(&mut self) -> Option<char> {
        if let Some((current_index, c)) = self.next_item {
            self.start_index = current_index;
            self.next();
            Some(c)
        } else {
            None
        }
    }

    fn next(&mut self) {
        self.next_item = self.iterator.next();
    }

    fn next_if<P: Fn(&mut Self) -> bool>(&mut self, predicate: P) -> bool {
        if predicate(self) {
            self.next();
            true
        } else {
            false
        }
    }

    fn peek(&mut self) -> Option<&char> {
        if let Some((_, c)) = self.iterator.peek() {
            Some(c)
        } else {
            None
        }
    }

    fn parse_until<P: Fn(&Self) -> bool>(&mut self, predicate: P) {
        while !predicate(self) {
            self.next();
        }
    }

    fn parsed_str(&self) -> &'a str {
        match self.next_item {
            Some((end_index, _)) => &self.text[self.start_index..end_index],
            None => &self.text[self.start_index..],
        }
    }
}

pub fn parse(text: &str) -> Vec<Sequence<Token>> {
    let result = parse_newlines(text);
    let result = parse_begin_end_comments_and_strings(result);
    parse_comments(text, result)
}

fn parse_newlines(text: &str) -> Vec<Sequence<AuxiliaryToken>> {
    let mut result = Vec::<Sequence<AuxiliaryToken>>::new();

    let mut line_number: usize = 1;

    let mut parser = Parser::new(&text);
    while let Some(c) = parser.begin_parsing() {
        let token = match c {
            '\r' | '\n' => {
                if c == '\r' {
                    parser.next_if(|p| matches!(p.next_item, Some((_, '\n'))));
                }
                line_number += 1;
                AuxiliaryToken::NewLine(line_number)
            }
            _ => {
                parser.parse_until(|p| matches!(p.next_item, None | Some((_, '\r' | '\n'))));
                AuxiliaryToken::Other
            }
        };

        let text = parser.parsed_str();
        result.push(Sequence { token, text });
    }

    result
}

fn parse_begin_end_comments_and_strings(
    sequences: Vec<Sequence<AuxiliaryToken>>,
) -> Vec<Sequence<AuxiliaryToken>> {
    let mut result = Vec::<Sequence<AuxiliaryToken>>::new();

    for s in sequences {
        if let AuxiliaryToken::NewLine(_) = s.token {
            result.push(s);
        } else {
            let mut parser = Parser::new(s.text);

            while let Some(c) = parser.begin_parsing() {
                let token = match c {
                    '"' => AuxiliaryToken::StrDelimiter,
                    '\'' => AuxiliaryToken::CharDelimiter,
                    '/' => {
                        let next_item = parser.next_item;
                        parser.next();

                        match next_item {
                            Some((_, '/')) => AuxiliaryToken::BeginSingleLineComment,
                            Some((_, '*')) => AuxiliaryToken::BeginMultiLineComment,
                            _ => AuxiliaryToken::Other,
                        }
                    }
                    '*' => {
                        let next_item = parser.next_item;
                        parser.next();

                        match next_item {
                            Some((_, '/')) => AuxiliaryToken::EndMultiLineComment,
                            _ => AuxiliaryToken::Other,
                        }
                    }
                    _ => AuxiliaryToken::Other,
                };

                if let AuxiliaryToken::Other = token {
                    loop {
                        parser.parse_until(|p| {
                            matches!(p.next_item, None | Some((_, '/' | '*' | '"' | '\'')))
                        });

                        match parser.next_item {
                            Some((_, '/')) => {
                                if !parser.next_if(|p| !(matches!(p.peek(), Some('/' | '*')))) {
                                    break;
                                }
                            }
                            Some((_, '*')) => {
                                if !parser.next_if(|p| !(matches!(p.peek(), Some('/')))) {
                                    break;
                                }
                            }
                            None | Some((_, '"' | '\'')) => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }

                let text = parser.parsed_str();
                result.push(Sequence { token, text });
            }
        }
    }

    result
}

fn parse_comments<'a>(
    text: &'a str,
    sequences: Vec<Sequence<AuxiliaryToken>>,
) -> Vec<Sequence<'a, Token>> {
    let mut result = Vec::<Sequence<Token>>::new();

    let n = sequences.len();

    let mut start_index: usize = 0;
    let mut i: usize = 0;
    while i < n {
        let s = &sequences[i];
        let mut end_index: usize = start_index + s.text.len();

        i += 1;

        let token = match s.token {
            AuxiliaryToken::BeginSingleLineComment => {
                while i < n {
                    let s = &sequences[i];

                    if let AuxiliaryToken::NewLine(_) = s.token {
                        break;
                    }

                    end_index += s.text.len();
                    i += 1;
                }

                Token::SingleLineComment
            }
            AuxiliaryToken::BeginMultiLineComment => {
                let mut level: usize = 1;

                while i < n {
                    let s = &sequences[i];

                    end_index += s.text.len();
                    i += 1;

                    match s.token {
                        AuxiliaryToken::BeginMultiLineComment => level += 1,
                        AuxiliaryToken::EndMultiLineComment => {
                            level -= 1;
                            if level == 0 {
                                break;
                            }
                        }
                        _ => (),
                    }
                }

                if level > 0 {
                    Token::Invalid(format!(
                        "Multiline comment not closed ({level} level(s) unclosed)."
                    ))
                } else {
                    Token::MultiLineComment
                }
            }
            AuxiliaryToken::EndMultiLineComment => Token::Invalid(String::from(
                "Multiline end comment detected without a beginning.",
            )),
            _ => Token::from(s.token),
        };

        result.push(Sequence {
            token,
            text: &text[start_index..end_index],
        });

        start_index = end_index;
    }

    result
}
