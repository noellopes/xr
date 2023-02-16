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
    NewLine(usize),
    Other,
}

#[derive(PartialEq)]
pub enum Token {
    SingleLineComment,
    MultiLineComment,
    NewLine(usize),
    Invalid(String),
    Other,
}

impl Token {
    fn from(token: AuxiliaryToken) -> Token {
        match token {
            AuxiliaryToken::NewLine(line_number) => Token::NewLine(line_number),
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
    current_item: Option<(usize, char)>,
    start_index: usize,
    text: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Parser {
        let mut iterator = text.char_indices().peekable();
        let current_item = iterator.next();

        Parser {
            iterator,
            current_item,
            start_index: 0,
            text,
        }
    }

    fn begin_parsing(&mut self) -> Option<char> {
        if let Some((current_index, c)) = self.current_item {
            self.start_index = current_index;
            self.next();
            Some(c)
        } else {
            None
        }
    }

    fn next(&mut self) {
        self.current_item = self.iterator.next();
    }

    fn parse_loop<P>(&mut self, predicate: P, break_value: bool)
    where
        P: Fn(&Self) -> bool,
    {
        while self.current_item.is_some() {
            self.next();

            if predicate(self) == break_value {
                break;
            }
        }
    }

    fn parse_while<P>(&mut self, predicate: P)
    where
        P: Fn(&Self) -> bool,
    {
        self.parse_loop(predicate, false);
    }

    fn parse_until<P>(&mut self, predicate: P)
    where
        P: Fn(&Self) -> bool,
    {
        self.parse_loop(predicate, true);
    }

    fn parsed_str(&self) -> &'a str {
        match self.current_item {
            Some((end_index, _)) => &self.text[self.start_index..end_index],
            None => &self.text[self.start_index..],
        }
    }
}

pub fn parse(text: &str) -> Vec<Sequence<Token>> {
    let result = parse_newlines(text);
    let result = parse_begin_end_comments(result);
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
                    if let Some((_, '\n')) = parser.current_item {
                        parser.next();
                    }
                }
                line_number += 1;
                AuxiliaryToken::NewLine(line_number)
            }
            _ => {
                parser.parse_until(|p| matches!(p.current_item, Some((_, '\r' | '\n'))));
                AuxiliaryToken::Other
            }
        };

        let text = parser.parsed_str();
        result.push(Sequence { token, text });
    }

    result
}

fn parse_begin_end_comments(
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
                    '/' => match parser.current_item {
                        Some((_, '/')) => AuxiliaryToken::BeginSingleLineComment,
                        Some((_, '*')) => AuxiliaryToken::BeginMultiLineComment,
                        _ => AuxiliaryToken::Other,
                    },
                    '*' => match parser.current_item {
                        Some((_, '/')) => AuxiliaryToken::EndMultiLineComment,
                        _ => AuxiliaryToken::Other,
                    },
                    _ => AuxiliaryToken::Other,
                };

                match token {
                    AuxiliaryToken::Other => loop {
                        parser.parse_until(|p| matches!(p.current_item, Some((_, '/' | '*'))));

                        match parser.current_item {
                            Some((_, '/')) => {
                                if let Some((_, '/' | '*')) = parser.iterator.peek() {
                                    break;
                                }
                            }
                            Some((_, '*')) => {
                                if let Some((_, '/')) = parser.iterator.peek() {
                                    break;
                                }
                            }
                            None => {
                                break;
                            }
                            _ => {}
                        }
                    },
                    _ => {
                        parser.next();
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

                Token::MultiLineComment
            }
            AuxiliaryToken::EndMultiLineComment => {
                Token::Invalid(String::from("End comment detected without a beginning."))
            }
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
