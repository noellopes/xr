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
pub enum Token {
    Space,
    Other,
}

pub struct Sequence<'a> {
    pub token: Token,
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
            Some(c)
        } else {
            None
        }
    }

    fn parse_while<P>(&mut self, predicate: P)
    where
        P: Fn(&Self) -> bool,
    {
        while self.current_item.is_some() {
            self.current_item = self.iterator.next();

            if !predicate(&self) {
                break;
            }
        }
    }

    fn parsed_str(&self) -> &'a str {
        if let Some((end_index, _)) = self.current_item {
            &self.text[self.start_index..end_index]
        } else {
            &self.text[self.start_index..]
        }
    }
}

pub fn parse(text: &str) -> Vec<Sequence> {
    let mut parser = Parser::new(&text);
    let mut result = Vec::<Sequence>::new();

    while let Some(c) = parser.begin_parsing() {
        let sequence = if c.is_whitespace() {
            parser.parse_while(|p| match p.current_item {
                None | Some((_, '\r' | '\n')) => false,
                Some((_, c)) => c.is_whitespace(),
            });

            Sequence {
                token: Token::Space,
                text: parser.parsed_str(),
            }
        } else {
            parser.parse_while(|p| match p.current_item {
                None => false,
                Some((_, c)) => !c.is_whitespace(),
            });

            Sequence {
                token: Token::Other,
                text: parser.parsed_str(),
            }
        };

        result.push(sequence);
    }

    result
}
