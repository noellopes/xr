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

use std::{slice::Iter, str::CharIndices};

#[derive(Copy, Clone, PartialEq)]
enum LevelOneToken {
    BackSlash,
    ForwardSlash,
    Asterisc,
    CharDelimiter,
    StrDelimiter,
    NewLine,
    Digit,
    UnderscoreLetter,
    Other,
}

impl From<char> for LevelOneToken {
    fn from(value: char) -> Self {
        match value {
            '\\' => Self::BackSlash,
            '/' => Self::ForwardSlash,
            '*' => Self::Asterisc,
            '\'' => Self::CharDelimiter,
            '"' => Self::StrDelimiter,
            '\r' | '\n' => Self::NewLine,
            '_' | 'a'..='z' | 'A'..='Z' => Self::UnderscoreLetter,
            '0'..='9' => Self::Digit,
            _ => Self::Other,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum LevelTwoToken {
    BackSlash,
    BeginSingleLineComment,
    BeginMultiLineComment,
    EndMultiLineComment,
    CharDelimiter,
    StrDelimiter,
    Word,
    NewLine(usize),
    Other,
}

impl From<LevelOneToken> for LevelTwoToken {
    fn from(value: LevelOneToken) -> Self {
        match value {
            LevelOneToken::BackSlash => Self::BackSlash,
            LevelOneToken::CharDelimiter => Self::CharDelimiter,
            LevelOneToken::StrDelimiter => Self::StrDelimiter,
            _ => Self::Other,
        }
    }
}

#[derive(PartialEq)]
pub enum Token {
    SingleLineComment,
    MultiLineComment,
    NewLine(usize),
    StrLiteral,
    CharLiteral,
    LifetimeElision,
    Invalid(String),
    Other,
}

impl From<LevelTwoToken> for Token {
    fn from(token: LevelTwoToken) -> Self {
        match token {
            LevelTwoToken::NewLine(line_number) => Self::NewLine(line_number),
            _ => Self::Other,
        }
    }
}

impl Token {
    fn unclosed_char_literal() -> Self {
        Self::Invalid(String::from("Unclosed char or lifetime elision"))
    }

    fn invalid_char_literal() -> Token {
        Self::Invalid(String::from("Invalid char literal"))
    }

    fn unclosed_multi_line_comment(unclosed_levels: usize) -> Self {
        Self::Invalid(format!(
            "Multiline comment not closed ({unclosed_levels} level(s) unclosed)."
        ))
    }

    fn multi_line_comment_without_beggining() -> Self {
        Self::Invalid(String::from(
            "Multiline end comment detected without a beginning.",
        ))
    }
}

pub struct Sequence<'a, T: PartialEq> {
    pub token: T,
    pub text: &'a str,
}

trait Parser {
    fn next(&mut self);

    fn parse_while<P: Fn(&mut Self) -> bool>(&mut self, predicate: P) {
        while predicate(self) {
            self.next();
        }
    }

    fn parse_until<P: Fn(&mut Self) -> bool>(&mut self, predicate: P) {
        while !predicate(self) {
            self.next();
        }
    }

    fn advance_and_parse_until<P: Fn(&mut Self) -> bool>(&mut self, predicate: P) {
        self.next();
        self.parse_until(predicate)
    }
}

struct StrParser<'a> {
    iterator: CharIndices<'a>,
    current_item: Option<(usize, char)>,
    start_index: usize,
    text: &'a str,
}

impl<'a> StrParser<'a> {
    fn new(text: &'a str) -> StrParser {
        let mut iterator = text.char_indices();
        let current_item = iterator.next();

        StrParser {
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

    fn parsed_str(&self) -> &'a str {
        match self.current_item {
            Some((end_index, _)) => &self.text[self.start_index..end_index],
            None => &self.text[self.start_index..],
        }
    }
}

impl<'a> Parser for StrParser<'a> {
    fn next(&mut self) {
        self.current_item = self.iterator.next();
    }
}

struct VecParser<'a, 'b, T: Copy + PartialEq> {
    text: &'a str,
    iterator: Iter<'b, Sequence<'b, T>>,
    current_item: Option<&'b Sequence<'b, T>>,
    next_item: Option<&'b Sequence<'b, T>>,
    start_index: usize,
    end_index: usize,
}

impl<'a, 'b, T: Copy + PartialEq> VecParser<'a, 'b, T> {
    fn new(text: &'a str, vector: &'b Vec<Sequence<'b, T>>) -> VecParser<'a, 'b, T> {
        let mut iterator = vector.iter();
        let next_item = iterator.next();

        VecParser {
            text,
            iterator,
            current_item: None,
            next_item,
            start_index: 0,
            end_index: 0,
        }
    }

    fn begin_parsing(&mut self) -> Option<&Sequence<T>> {
        self.start_index = self.end_index;
        self.next();

        self.current_item
    }

    fn next_if<P: Fn(&mut Self) -> bool>(&mut self, predicate: P) -> bool {
        if predicate(self) {
            self.next();
            true
        } else {
            false
        }
    }

    fn current_token(&self) -> Option<T> {
        Some(self.current_item?.token)
    }

    fn next_token(&self) -> Option<T> {
        Some(self.next_item?.token)
    }

    fn parsed_str(&self) -> &'a str {
        &self.text[self.start_index..self.end_index]
    }
}

impl<'a, 'b, T: Copy + PartialEq> Parser for VecParser<'a, 'b, T> {
    fn next(&mut self) {
        self.current_item = self.next_item;
        self.next_item = self.iterator.next();

        if let Some(i) = self.current_item {
            self.end_index += i.text.len();
        }
    }
}

pub fn parse(text: &str) -> Vec<Sequence<Token>> {
    let result: Vec<Sequence<LevelOneToken>> = parse_level_one_tokens(text);
    let result: Vec<Sequence<LevelTwoToken>> = parse_level_two_tokens(text, result);
    parse_level_three_tokens(text, result)
}

fn parse_level_one_tokens(text: &str) -> Vec<Sequence<LevelOneToken>> {
    let mut result = Vec::<Sequence<LevelOneToken>>::new();

    let mut parser = StrParser::new(&text);
    while let Some(c) = parser.begin_parsing() {
        let token = LevelOneToken::from(c);

        if let LevelOneToken::NewLine | LevelOneToken::Other = token {
            parser.parse_while(
                |p| matches!(p.current_item, Some((_, c)) if LevelOneToken::from(c) == token),
            );
        }

        let text = parser.parsed_str();
        result.push(Sequence { token, text });
    }

    result
}

fn parse_level_two_tokens<'a>(
    text: &'a str,
    sequences: Vec<Sequence<LevelOneToken>>,
) -> Vec<Sequence<'a, LevelTwoToken>> {
    let mut result = Vec::<Sequence<LevelTwoToken>>::new();

    let mut line_number: usize = 1;

    let mut parser = VecParser::new(&text, &sequences);
    while let Some(s) = parser.begin_parsing() {
        let token = match s.token {
            LevelOneToken::ForwardSlash => parse_possible_comment_token(&mut parser),
            LevelOneToken::Asterisc => parse_possible_end_multi_line_comment(&mut parser),
            LevelOneToken::UnderscoreLetter => parse_word(&mut parser),
            LevelOneToken::NewLine => {
                line_number += cout_new_lines(s.text);
                LevelTwoToken::NewLine(line_number)
            }
            other => LevelTwoToken::from(other),
        };

        let text = parser.parsed_str();
        result.push(Sequence { token, text });
    }

    result
}

fn cout_new_lines(text: &str) -> usize {
    let mut new_lines = 0;

    for c in text.chars() {
        if c == '\n' {
            new_lines += 1;
        }
    }

    new_lines
}

fn parse_word(parser: &mut VecParser<LevelOneToken>) -> LevelTwoToken {
    parser.parse_while(|p| {
        matches!(
            p.next_token(),
            Some(LevelOneToken::UnderscoreLetter | LevelOneToken::Digit)
        )
    });
    LevelTwoToken::Word
}

fn parse_possible_end_multi_line_comment(parser: &mut VecParser<LevelOneToken>) -> LevelTwoToken {
    if parser.next_if(|p| matches!(p.next_token(), Some(LevelOneToken::ForwardSlash))) {
        LevelTwoToken::EndMultiLineComment
    } else {
        LevelTwoToken::Other
    }
}

fn parse_possible_comment_token(parser: &mut VecParser<LevelOneToken>) -> LevelTwoToken {
    if parser.next_if(|p| matches!(p.next_token(), Some(LevelOneToken::ForwardSlash))) {
        LevelTwoToken::BeginSingleLineComment
    } else if parser.next_if(|p| matches!(p.next_token(), Some(LevelOneToken::Asterisc))) {
        LevelTwoToken::BeginMultiLineComment
    } else {
        LevelTwoToken::Other
    }
}

fn parse_level_three_tokens<'a>(
    text: &'a str,
    sequences: Vec<Sequence<LevelTwoToken>>,
) -> Vec<Sequence<'a, Token>> {
    let mut result = Vec::<Sequence<Token>>::new();

    let mut parser = VecParser::new(&text, &sequences);
    while let Some(s) = parser.begin_parsing() {
        let token = match s.token {
            LevelTwoToken::BeginSingleLineComment => parse_single_line_comment(&mut parser),
            LevelTwoToken::BeginMultiLineComment => parse_multi_line_comment(&mut parser),
            LevelTwoToken::EndMultiLineComment => Token::multi_line_comment_without_beggining(),
            LevelTwoToken::CharDelimiter => parse_char_literal_or_elison(&mut parser),
            // LevelTwoToken::StrDelimiter => todo!(),
            other => Token::from(other),
        };

        let text = parser.parsed_str();
        result.push(Sequence { token, text });
    }

    result
}

fn parse_char_literal_or_elison(parser: &mut VecParser<LevelTwoToken>) -> Token {
    parser.next();

    match parser.current_token() {
        Some(token) => {
            match token {
                LevelTwoToken::BackSlash => {
                    parser.next(); // ignore at least the next token that might be a CharDelimiter
                    parse_until_close_char_literal(parser)
                }
                LevelTwoToken::Word => {
                    if parser
                        .next_if(|p| matches!(p.next_token(), Some(LevelTwoToken::CharDelimiter)))
                    {
                        Token::CharLiteral
                    } else {
                        Token::LifetimeElision
                    }
                }
                LevelTwoToken::StrDelimiter | LevelTwoToken::Other => {
                    parse_until_close_char_literal(parser)
                }
                _ => Token::invalid_char_literal(),
            }
        }
        None => Token::unclosed_char_literal(),
    }
}

fn parse_until_close_char_literal(parser: &mut VecParser<LevelTwoToken>) -> Token {
    parser.advance_and_parse_until(|p| {
        matches!(p.current_token(), None | Some(LevelTwoToken::CharDelimiter))
    });

    match parser.current_item {
        None => Token::unclosed_char_literal(),
        _ => Token::CharLiteral,
    }
}

fn parse_single_line_comment(parser: &mut VecParser<LevelTwoToken>) -> Token {
    parser.parse_until(|p| matches!(p.next_token(), None | Some(LevelTwoToken::NewLine(_))));
    Token::SingleLineComment
}

fn parse_multi_line_comment(parser: &mut VecParser<LevelTwoToken>) -> Token {
    let mut level: usize = 1;

    loop {
        parser.advance_and_parse_until(|p| {
            matches!(
                p.current_token(),
                None | Some(
                    LevelTwoToken::BeginMultiLineComment | LevelTwoToken::EndMultiLineComment
                )
            )
        });

        match parser.current_token() {
            Some(LevelTwoToken::BeginMultiLineComment) => level += 1,
            Some(LevelTwoToken::EndMultiLineComment) => {
                level -= 1;
                if level == 0 {
                    return Token::MultiLineComment;
                }
            }
            _ => {
                return Token::unclosed_multi_line_comment(level);
            }
        };
    }
}
