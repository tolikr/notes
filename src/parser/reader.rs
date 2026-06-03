use std::collections::BTreeMap;

use crate::parser::json::Json;

pub struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Reader<'a> {
        Reader { bytes, pos: 0 }
    }

    pub fn read(&mut self) -> Result<Json, ReaderError> {
        if !self.has_next() {
            return Err(ReaderError::EmptyInput);
        }

        let b = self.peek();
        if self.is_digit_begin(b) {
            self.read_number()
        } else if self.is_bool_begin(b) {
            self.read_bool()
        } else if self.is_string_begin(b) {
            self.read_string()
        } else if self.is_object_begin(b) {
            self.read_object()
        } else if self.is_array_begin(b) {
            self.read_array()
        } else {
            return Err(ReaderError::InvalidSyntax("Unsupported yet".to_string()));
        }
    }

    fn read_array(&mut self) -> Result<Json, ReaderError> {
        // Пропускаем начало массива
        self.next();

        let mut content: Vec<Json> = Vec::new();

        loop {
            if !self.has_next() {
                return Err(ReaderError::UnexpectedEof);
            }

            let next_byte = self.peek();

            match next_byte {
                // Встретили закрытие объекта — парсинг окончен
                b']' => {
                    self.next();
                    break;
                }
                // Встретили запятую — переходим к следующему элементу
                b',' => {
                    self.next();
                }

                _ => {
                    let value = self.read()?;

                    content.push(value);
                }
            }
        }

        Ok(Json::Array(content))
    }

    fn is_array_begin(&self, b: u8) -> bool {
        b == b'['
    }

    fn read_object(&mut self) -> Result<Json, ReaderError> {
        // Пропускаем начало объекта
        self.next();

        let mut content: BTreeMap<String, Json> = BTreeMap::new();

        loop {
            if !self.has_next() {
                return Err(ReaderError::UnexpectedEof);
            }

            let next_byte = self.peek();

            match next_byte {
                // Встретили закрытие объекта — парсинг окончен
                b'}' => {
                    self.next();
                    break;
                }
                // Встретили запятую — переходим к следующему элементу
                b',' => {
                    self.next();
                }

                _ => {
                    let key = self.read_raw_string()?;
                    if self.next() != b':' {
                        return Err(ReaderError::InvalidSyntax(
                            "Invalid key-value pair".to_string(),
                        ));
                    }
                    let value = self.read()?;

                    content.insert(key, value);
                }
            }
        }

        Ok(Json::Object(content))
    }

    fn is_object_begin(&self, b: u8) -> bool {
        b == b'{'
    }

    fn read_string(&mut self) -> Result<Json, ReaderError> {
        let string_result = self.read_raw_string()?;

        Ok(Json::String(string_result))
    }

    fn read_raw_string(&mut self) -> Result<String, ReaderError> {
        // Пропускаем открывающую кавычку
        self.next();

        let mut byte_buffer = Vec::new();

        loop {
            if !self.has_next() {
                return Err(ReaderError::UnexpectedEof);
            }

            let b = self.next();

            match b {
                // Встретили закрывающую кавычку — парсинг окончен
                b'"' => break,

                // Встретили экранирование — заменяем ДВА байта на ОДИН специальный
                b'\\' => {
                    if !self.has_next() {
                        return Err(ReaderError::UnexpectedEof);
                    }
                    let escaped = self.next();
                    match escaped {
                        b'"' => byte_buffer.push(b'"'),
                        b'\\' => byte_buffer.push(b'\\'),
                        b'/' => byte_buffer.push(b'/'),
                        b'b' => byte_buffer.push(8),     // Backspace
                        b'f' => byte_buffer.push(12),    // Form feed
                        b'n' => byte_buffer.push(b'\n'), // Перевод строки (10)
                        b'r' => byte_buffer.push(b'\r'), // Возврат каретки (13)
                        b't' => byte_buffer.push(b'\t'), // Табуляция (9)
                        _ => {
                            return Err(ReaderError::InvalidSyntax(
                                "Invalid escape sequence".to_string(),
                            ));
                        }
                    }
                }

                // Любой другой байт (включая части многобайтных символов UTF-8 вроде кириллицы)
                // просто копируем как есть
                _ => byte_buffer.push(b),
            }
        }

        // В самом конце превращаем накопленные байты в валидную Rust-строку
        String::from_utf8(byte_buffer)
            .map_err(|_| ReaderError::InvalidSyntax("Invalid UTF-8 sequence".to_string()))
    }

    fn is_string_begin(&self, b: u8) -> bool {
        b == b'"'
    }

    fn read_bool(&mut self) -> Result<Json, ReaderError> {
        let mut value = false;
        let mut at_begining = true;
        let mut true_bytes = b"true";
        let mut false_bytes = b"false";
        let mut assert_pos = 1;

        loop {
            if self.has_next() {
                let next_char = self.next();
                if next_char == b't' && at_begining {
                    value = true;
                    at_begining = false;
                } else if next_char == b'f' && at_begining {
                    value = false;
                    at_begining = false;
                } else {
                    if at_begining {
                        return Err(ReaderError::InvalidSyntax("Invalid bool".to_string()));
                    } else {
                        if (value) {
                            if true_bytes[assert_pos] == next_char {
                                if !assert_pos < true_bytes.len() {
                                    break;
                                }
                                assert_pos += 1;
                            } else {
                                return Err(ReaderError::InvalidSyntax(
                                    "Invalid true bool".to_string(),
                                ));
                            }
                        } else {
                            if false_bytes[assert_pos] == next_char {
                                if !assert_pos < false_bytes.len() {
                                    break;
                                }
                                assert_pos += 1;
                            } else {
                                return Err(ReaderError::InvalidSyntax(
                                    "Invalid false bool".to_string(),
                                ));
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(Json::Bool(value))
    }

    fn is_bool_begin(&self, b: u8) -> bool {
        b == b't' || b == b'f'
    }

    fn read_number(&mut self) -> Result<Json, ReaderError> {
        let mut at_begining = true;
        let mut is_negative = false;
        let mut is_fraqtion_part = false;
        let mut fraction_part = 0; // Сюда копим цифры дроби как целое число
        let mut fraction_digits = 0; // Считаем количество знаков после точки
        let mut number = 0.0;

        loop {
            if self.has_next() {
                let next_digit = self.next();
                if self.is_digit(next_digit) {
                    if is_fraqtion_part {
                        fraction_part = fraction_part * 10 + (next_digit - b'0') as i64;
                        fraction_digits += 1;
                    } else {
                        number = number * 10.0 + (next_digit - b'0') as f64;
                    }
                    at_begining = false;
                } else if self.is_negative(next_digit) {
                    if at_begining {
                        is_negative = true;
                        at_begining = false;
                    } else {
                        return Err(ReaderError::InvalidSyntax("Invalid digit".to_string()));
                    }
                } else if next_digit == b'.' {
                    if is_fraqtion_part {
                        return Err(ReaderError::InvalidSyntax("Invalid fraqtional".to_string()));
                    } else {
                        is_fraqtion_part = true;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if fraction_digits > 0 {
            // Делаем ОДНО деление в самом конце. Это минимизирует погрешность.
            number += fraction_part as f64 / 10.0_f64.powi(fraction_digits);
        }

        if is_negative {
            Ok(Json::Number(-number))
        } else {
            Ok(Json::Number(number))
        }
    }

    fn is_digit_begin(&self, b: u8) -> bool {
        self.is_digit(b) || self.is_negative(b)
    }

    fn is_digit(&self, b: u8) -> bool {
        b.is_ascii_digit()
    }

    fn is_negative(&self, b: u8) -> bool {
        b == b'-'
    }

    fn has_next(&self) -> bool {
        self.pos < self.bytes.len()
    }

    fn next(&mut self) -> u8 {
        let byte = self.peek();
        self.pos += 1;
        byte
    }

    fn skip_spaces(&mut self) {
        loop {
            let byte = self.peek();
            match byte {
                b' ' => if self.has_next() { self.pos += 1 } else { break },
                _ => break
            }
        }
    }

    fn peek(&self) -> u8 {
        self.bytes[self.pos]
    }
}

// errors.rs или прямо в модуле
#[derive(Debug, PartialEq)]
pub enum ReaderError {
    EmptyInput,
    InvalidSyntax(String),
    UnexpectedEof,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Вспомогательная функция для удобства
    fn parse(s: &str) -> Result<Json, ReaderError> {
        let mut reader = Reader::new(s.as_bytes());
        reader.read()
    }

    #[test]
    fn test_parse_simple_number() {
        assert_eq!(parse("123"), Ok(Json::Number(123.into())));
    }

    #[test]
    fn test_parse_single_digit() {
        assert_eq!(parse("5"), Ok(Json::Number(5.into())));
    }

    #[test]
    fn test_parse_single_negative_digit() {
        assert_eq!(parse("-5"), Ok(Json::Number((-5).into())));
    }

    #[test]
    fn test_invalid_negative_digit() {
        assert_eq!(
            parse("-5-5"),
            Err(ReaderError::InvalidSyntax("Invalid digit".to_string()))
        );
    }

    #[test]
    fn test_parse_single_float_number() {
        assert_eq!(parse("1.2"), Ok(Json::Number(1.2)));
    }

    #[test]
    fn test_parse_simple_float_number() {
        assert_eq!(parse("123.234"), Ok(Json::Number(123.234)));
    }

    #[test]
    fn test_invalid_float_number() {
        assert_eq!(
            parse("123.23.4"),
            Err(ReaderError::InvalidSyntax("Invalid fraqtional".to_string()))
        );
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(parse(""), Err(ReaderError::EmptyInput));
    }

    #[test]
    fn test_parse_true() {
        assert_eq!(parse("true"), Ok(Json::Bool(true)));
    }

    #[test]
    fn test_invalid_true() {
        assert_eq!(
            parse("tree"),
            Err(ReaderError::InvalidSyntax("Invalid true bool".to_string()))
        );
    }

    #[test]
    fn test_parse_false() {
        assert_eq!(parse("false"), Ok(Json::Bool(false)));
    }

    #[test]
    fn test_invalid_false() {
        assert_eq!(
            parse("falye"),
            Err(ReaderError::InvalidSyntax("Invalid false bool".to_string()))
        );
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse("\"abc\""), Ok(Json::String("abc".to_string())));
    }

    #[test]
    fn test_parse_string_new_line() {
        assert_eq!(
            parse("\"abc\nfgd\""),
            Ok(Json::String("abc\nfgd".to_string()))
        );
    }

    #[test]
    fn test_parse_string_escaped() {
        assert_eq!(
            parse(r#""abc\"fgd""#),
            Ok(Json::String(r#"abc"fgd"#.to_string()))
        );
    }

    #[test]
    fn test_parse_object_empty() {
        assert_eq!(parse("{}"), Ok(Json::Object(BTreeMap::new())));
    }

    #[test]
    fn test_parse_object() {
        let expected = Json::Object(BTreeMap::from([(
            "abc".to_string(),
            Json::String("xyz".to_string()),
        )]));

        assert_eq!(parse(r#"{"abc":"xyz"}"#), Ok(expected));
    }
    
    #[test]
    fn test_parse_array_empty() {
        assert_eq!(parse("[]"), Ok(Json::Array(Vec::new())));
    }

    #[test]
    fn test_parse_array() {
        let expected = Json::Array(vec![Json::String("abc".to_string())]);

        assert_eq!(parse(r#"["abc"]"#), Ok(expected));
    }

    // #[test]
    // fn test_null() {
    //     assert_eq!(parse("null"), Ok(Json::Null));
    // }
}
