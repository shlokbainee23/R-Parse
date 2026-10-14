//! Minimal, self-contained JSON parser and serializer with zero external dependencies.

use std::collections::BTreeMap;
use std::fmt;

/// A JSON value representation.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    /// Returns the string slice if this is a JSON string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the boolean if this is a JSON boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the number as an f64 if this is a JSON number.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the number as a u64 if this is a non-negative integer number.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(n) if *n >= 0.0 && n.fract() == 0.0 => Some(*n as u64),
            _ => None,
        }
    }

    /// Returns the number as an i64 if this is an integer number.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(n) if n.fract() == 0.0 => Some(*n as i64),
            _ => None,
        }
    }

    /// Returns the array slice if this is a JSON array.
    pub fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns the object map if this is a JSON object.
    pub fn as_object(&self) -> Option<&BTreeMap<String, JsonValue>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Looks up a key in a JSON object.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            Self::Object(o) => o.get(key),
            _ => None,
        }
    }

    /// Serializes this JSON value into a compact JSON string.
    pub fn to_json_string(&self) -> String {
        let mut out = String::new();
        self.serialize_into(&mut out);
        out
    }

    fn serialize_into(&self, out: &mut String) {
        match self {
            Self::Null => out.push_str("null"),
            Self::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Self::Number(n) => {
                if n.fract() == 0.0
                    && !n.is_infinite()
                    && !n.is_nan()
                    && *n >= (i64::MIN as f64)
                    && *n <= (i64::MAX as f64)
                {
                    out.push_str(&format!("{}", *n as i64));
                } else {
                    out.push_str(&format!("{}", n));
                }
            }
            Self::String(s) => {
                out.push('"');
                for c in s.chars() {
                    match c {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        '\r' => out.push_str("\\r"),
                        '\t' => out.push_str("\\t"),
                        other => out.push(other),
                    }
                }
                out.push('"');
            }
            Self::Array(arr) => {
                out.push('[');
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    val.serialize_into(out);
                }
                out.push(']');
            }
            Self::Object(obj) => {
                out.push('{');
                for (i, (k, val)) in obj.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    JsonValue::String(k.clone()).serialize_into(out);
                    out.push(':');
                    val.serialize_into(out);
                }
                out.push('}');
            }
        }
    }

    /// Parses a JSON string into a `JsonValue`.
    pub fn parse(input: &str) -> Result<Self, String> {
        let chars: Vec<char> = input.chars().collect();
        let mut idx = 0;
        skip_ws(&chars, &mut idx);
        let val = parse_value(&chars, &mut idx)?;
        skip_ws(&chars, &mut idx);
        if idx < chars.len() {
            return Err(format!(
                "Unexpected trailing characters at position {}: '{}'",
                idx,
                &input[idx..]
            ));
        }
        Ok(val)
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_json_string())
    }
}

fn skip_ws(chars: &[char], idx: &mut usize) {
    while *idx < chars.len() && chars[*idx].is_whitespace() {
        *idx += 1;
    }
}

fn parse_value(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    skip_ws(chars, idx);
    if *idx >= chars.len() {
        return Err("Unexpected end of input while parsing JSON value".to_string());
    }

    match chars[*idx] {
        'n' => parse_null(chars, idx),
        't' | 'f' => parse_bool(chars, idx),
        '"' => parse_string(chars, idx).map(JsonValue::String),
        '[' => parse_array(chars, idx),
        '{' => parse_object(chars, idx),
        '-' | '0'..='9' => parse_number(chars, idx),
        ch => Err(format!("Unexpected character '{}' at index {}", ch, idx)),
    }
}

fn parse_null(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    if *idx + 4 <= chars.len() && chars[*idx..*idx + 4] == ['n', 'u', 'l', 'l'] {
        *idx += 4;
        Ok(JsonValue::Null)
    } else {
        Err(format!("Expected 'null' at index {}", idx))
    }
}

fn parse_bool(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    if *idx + 4 <= chars.len() && chars[*idx..*idx + 4] == ['t', 'r', 'u', 'e'] {
        *idx += 4;
        Ok(JsonValue::Bool(true))
    } else if *idx + 5 <= chars.len() && chars[*idx..*idx + 5] == ['f', 'a', 'l', 's', 'e'] {
        *idx += 5;
        Ok(JsonValue::Bool(false))
    } else {
        Err(format!("Expected boolean at index {}", idx))
    }
}

fn parse_string(chars: &[char], idx: &mut usize) -> Result<String, String> {
    if *idx >= chars.len() || chars[*idx] != '"' {
        return Err(format!("Expected '\"' at index {}", idx));
    }
    *idx += 1;
    let mut s = String::new();
    while *idx < chars.len() {
        let ch = chars[*idx];
        *idx += 1;
        match ch {
            '"' => return Ok(s),
            '\\' => {
                if *idx >= chars.len() {
                    return Err("Unterminated escape sequence in string".to_string());
                }
                let esc = chars[*idx];
                *idx += 1;
                match esc {
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    '/' => s.push('/'),
                    'b' => s.push('\x08'),
                    'f' => s.push('\x0C'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    'u' => {
                        if *idx + 4 > chars.len() {
                            return Err("Incomplete \\u hex escape in string".to_string());
                        }
                        let hex_str: String = chars[*idx..*idx + 4].iter().collect();
                        *idx += 4;
                        let code = u32::from_str_radix(&hex_str, 16)
                            .map_err(|_| format!("Invalid \\u hex: {}", hex_str))?;
                        let ch = char::from_u32(code)
                            .ok_or_else(|| format!("Invalid unicode codepoint: {:x}", code))?;
                        s.push(ch);
                    }
                    other => s.push(other),
                }
            }
            other => s.push(other),
        }
    }
    Err("Unterminated string literal".to_string())
}

fn parse_number(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    let start = *idx;
    if *idx < chars.len() && chars[*idx] == '-' {
        *idx += 1;
    }
    while *idx < chars.len() && chars[*idx].is_ascii_digit() {
        *idx += 1;
    }
    if *idx < chars.len() && chars[*idx] == '.' {
        *idx += 1;
        while *idx < chars.len() && chars[*idx].is_ascii_digit() {
            *idx += 1;
        }
    }
    if *idx < chars.len() && (chars[*idx] == 'e' || chars[*idx] == 'E') {
        *idx += 1;
        if *idx < chars.len() && (chars[*idx] == '+' || chars[*idx] == '-') {
            *idx += 1;
        }
        while *idx < chars.len() && chars[*idx].is_ascii_digit() {
            *idx += 1;
        }
    }

    let num_str: String = chars[start..*idx].iter().collect();
    let num: f64 = num_str
        .parse()
        .map_err(|_| format!("Invalid number literal: '{}'", num_str))?;
    Ok(JsonValue::Number(num))
}

fn parse_array(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    if *idx >= chars.len() || chars[*idx] != '[' {
        return Err(format!("Expected '[' at index {}", idx));
    }
    *idx += 1;
    skip_ws(chars, idx);

    let mut arr = Vec::new();
    if *idx < chars.len() && chars[*idx] == ']' {
        *idx += 1;
        return Ok(JsonValue::Array(arr));
    }

    loop {
        let val = parse_value(chars, idx)?;
        arr.push(val);
        skip_ws(chars, idx);
        if *idx >= chars.len() {
            return Err("Unterminated array literal".to_string());
        }
        if chars[*idx] == ']' {
            *idx += 1;
            break;
        } else if chars[*idx] == ',' {
            *idx += 1;
            skip_ws(chars, idx);
        } else {
            return Err(format!("Expected ',' or ']' in array at index {}", idx));
        }
    }

    Ok(JsonValue::Array(arr))
}

fn parse_object(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    if *idx >= chars.len() || chars[*idx] != '{' {
        return Err(format!("Expected '{{' at index {}", idx));
    }
    *idx += 1;
    skip_ws(chars, idx);

    let mut map = BTreeMap::new();
    if *idx < chars.len() && chars[*idx] == '}' {
        *idx += 1;
        return Ok(JsonValue::Object(map));
    }

    loop {
        skip_ws(chars, idx);
        if *idx >= chars.len() || chars[*idx] != '"' {
            return Err(format!("Expected string key in object at index {}", idx));
        }
        let key = parse_string(chars, idx)?;
        skip_ws(chars, idx);
        if *idx >= chars.len() || chars[*idx] != ':' {
            return Err(format!("Expected ':' after key in object at index {}", idx));
        }
        *idx += 1;
        let val = parse_value(chars, idx)?;
        map.insert(key, val);

        skip_ws(chars, idx);
        if *idx >= chars.len() {
            return Err("Unterminated object literal".to_string());
        }
        if chars[*idx] == '}' {
            *idx += 1;
            break;
        } else if chars[*idx] == ',' {
            *idx += 1;
            skip_ws(chars, idx);
        } else {
            return Err(format!("Expected ',' or '}}' in object at index {}", idx));
        }
    }

    Ok(JsonValue::Object(map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_primitives() {
        assert_eq!(JsonValue::parse("null").unwrap(), JsonValue::Null);
        assert_eq!(JsonValue::parse("true").unwrap(), JsonValue::Bool(true));
        assert_eq!(JsonValue::parse("false").unwrap(), JsonValue::Bool(false));
        assert_eq!(JsonValue::parse("42").unwrap(), JsonValue::Number(42.0));
        assert_eq!(JsonValue::parse("-3.5").unwrap(), JsonValue::Number(-3.5));
        assert_eq!(
            JsonValue::parse(r#""hello world""#).unwrap(),
            JsonValue::String("hello world".to_string())
        );
    }

    #[test]
    fn test_parse_complex_structure() {
        let json = r#"
        {
            "jsonrpc": "2.0",
            "id": 42,
            "method": "inspect_dataframe",
            "params": {
                "columns": ["price", "quantity"],
                "active": true,
                "count": 100
            }
        }
        "#;
        let parsed = JsonValue::parse(json).unwrap();
        assert_eq!(parsed.get("jsonrpc").unwrap().as_str(), Some("2.0"));
        assert_eq!(parsed.get("id").unwrap().as_u64(), Some(42));
        assert_eq!(
            parsed.get("method").unwrap().as_str(),
            Some("inspect_dataframe")
        );
        let params = parsed.get("params").unwrap();
        assert_eq!(params.get("active").unwrap().as_bool(), Some(true));
        assert_eq!(params.get("count").unwrap().as_u64(), Some(100));
        let cols = params.get("columns").unwrap().as_array().unwrap();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0].as_str(), Some("price"));
        assert_eq!(cols[1].as_str(), Some("quantity"));
    }

    #[test]
    fn test_roundtrip_serialization() {
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), JsonValue::String("test".to_string()));
        map.insert("val".to_string(), JsonValue::Number(123.0));
        let val = JsonValue::Object(map);
        let s = val.to_json_string();
        let reparsed = JsonValue::parse(&s).unwrap();
        assert_eq!(val, reparsed);
    }
}
