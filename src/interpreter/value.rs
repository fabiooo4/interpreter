use std::{
    fmt::Display,
    ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Rem, Sub},
    str::FromStr,
};

type IntType = i32;
type FloatType = f32;
type StringType = String;
type CharType = char;
type BoolType = bool;

#[derive(Default, Debug, Clone)]
pub enum Value {
    Int(IntType),
    Float(FloatType),
    Bool(BoolType),
    String(StringType),
    Char(CharType),

    #[default]
    Void,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(val) => write!(f, "{}", val),
            Value::Float(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
            Value::String(val) => write!(f, "{}", val),
            Value::Char(val) => write!(f, "{}", val),
            Value::Void => write!(f, "()"),
        }
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Int
        if let Ok(int_val) = s.parse::<IntType>() {
            return Ok(Value::Int(int_val));
        }

        // Float
        if let Ok(float_val) = s.parse::<FloatType>() {
            return Ok(Value::Float(float_val));
        }

        // Bool
        if let Ok(bool_val) = s.parse::<BoolType>() {
            return Ok(Value::Bool(bool_val));
        }

        // String
        if s.starts_with('"') && s.ends_with('"') {
            let str_without_quotes = unescaper::unescape(&s[1..s.len() - 1])
                .unwrap_or_else(|_| panic!("Failed to unescape string: '{}'", s));
            return Ok(Value::String(str_without_quotes));
        }

        // Char
        if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 3 {
            let char_without_quotes = unescaper::unescape(s)
                .unwrap_or_else(|_| panic!("Failed to unescape char: '{}'", s))
                .chars()
                .nth(1)
                .unwrap();
            return Ok(Value::Char(char_without_quotes));
        }

        panic!(
            "Failed to parse token: Undefined type for '{s}', an implementation of from_str is needed"
        )
    }
}

impl Value {
    fn type_name(&self) -> String {
        match self {
            Value::Int(_) => String::from("int"),
            Value::Float(_) => String::from("float"),
            Value::Bool(_) => String::from("bool"),
            Value::String(_) => String::from("string"),
            Value::Char(_) => String::from("char"),
            Value::Void => String::from("void"),
        }
    }

    pub fn pow(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.pow((*rhs).try_into().expect(
                "Type mismatch: cannot apply 'pow' operator, exponent must be a positive integer",
            ))),
            (Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.powf(*rhs)),

            _ => panic!(
                "Type mismatch: cannot apply 'pow' operator on '{}' and '{}'",
                self.type_name(),
                rhs.type_name()
            ),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(val) => Value::Bool(!val),

            _ => panic!(
                "Type mismatch: cannot apply 'not' operator on '{}'",
                self.type_name(),
            ),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.mul(*rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.mul(*rhs)),

            _ => panic!(
                "Type mismatch: cannot apply 'mul' operator on '{}' and '{}'",
                self.type_name(),
                rhs.type_name()
            ),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.div(*rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.div(*rhs)),

            _ => panic!(
                "Type mismatch: cannot apply 'div' operator on '{}' and '{}'",
                self.type_name(),
                rhs.type_name()
            ),
        }
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.rem(*rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.rem(*rhs)),

            _ => panic!(
                "Type mismatch: cannot apply 'rem' operator on '{}' and '{}'",
                self.type_name(),
                rhs.type_name()
            ),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.add(*rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.add(*rhs)),

            _ => panic!(
                "Type mismatch: cannot apply 'add' operator on '{}' and '{}'",
                self.type_name(),
                rhs.type_name()
            ),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.sub(*rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.sub(*rhs)),

            _ => panic!(
                "Type mismatch: cannot apply 'sub' operator on '{}' and '{}'",
                self.type_name(),
                rhs.type_name()
            ),
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Int(val) => Value::Int(-val),
            Value::Float(val) => Value::Float(-val),

            _ => panic!(
                "Type mismatch: cannot apply 'neg' operator on '{}'",
                self.type_name(),
            ),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(lhs), Value::Int(rhs)) => lhs.partial_cmp(rhs),
            (Value::Float(lhs), Value::Float(rhs)) => lhs.partial_cmp(rhs),

            _ => panic!(
                "Type mismatch: cannot compare '{}' and '{}'",
                self.type_name(),
                other.type_name()
            ),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::Float(lhs), Self::Float(rhs)) => lhs == rhs,
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::Char(lhs), Self::Char(rhs)) => lhs == rhs,
            (Self::Void, Self::Void) => true,

            _ => panic!(
                "Type mismatch: cannot compare '{}' and '{}'",
                self.type_name(),
                other.type_name()
            ),
        }
    }
}

impl BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Value::Bool(lhs.bitand(*rhs)),

            _ => panic!(
                "Type mismatch: cannot apply 'bitand' operator on '{}' and '{}'",
                self.type_name(),
                rhs.type_name()
            ),
        }
    }
}

impl BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Value::Bool(lhs.bitor(*rhs)),

            _ => panic!(
                "Type mismatch: cannot apply 'bitor' operator on '{}' and '{}'",
                self.type_name(),
                rhs.type_name()
            ),
        }
    }
}
