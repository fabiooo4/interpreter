use std::{
    fmt::Display,
    ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Rem, Sub},
    str::FromStr,
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Char,

    #[default]
    Void,
}
impl Type {
    pub fn pow(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Type::Int, Type::Int) => self,
            (Type::Float, Type::Float) => self,

            _ => panic!(
                "Type mismatch: cannot apply 'pow' operator on '{}' and '{}'",
                self, rhs
            ),
        }
    }

    pub fn cmp_type(&self, rhs: Self) -> Self {
        match (self, rhs) {
            (Type::Int, Type::Int) => *self,
            (Type::Float, Type::Float) => *self,

            _ => panic!("Type mismatch: cannot compare '{}' and '{}'", self, rhs),
        }
    }

    pub fn concat(&self, rhs: Type) -> Self {
        match (self, rhs) {
            (Type::String, Type::String)
            | (Type::String, Type::Char)
            | (Type::Char, Type::String)
            | (Type::Char, Type::Char) => Type::String,

            _ => panic!("Type mismatch: cannot concatenate '{}' and '{}'", self, rhs),
        }
    }

    pub fn cast(self, cast_typ: Type) -> Self {
        match (cast_typ, &self) {
            (Type::Int, Type::Float) => Type::Int,
            (Type::Float, Type::Int) => Type::Float,

            _ => panic!("Cannot convert '{self}' to '{cast_typ}'"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Char => write!(f, "char"),
            Type::Void => write!(f, "void"),
        }
    }
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "string" => Ok(Type::String),
            "char" => Ok(Type::Char),
            "bool" => Ok(Type::Bool),
            _ => panic!("Failed to parse token: Undefined type for '{s}'"),
        }
    }
}

impl Not for Type {
    type Output = Type;

    fn not(self) -> Self::Output {
        if self != Type::Bool {
            panic!("Type mismatch: cannot apply 'not' operator on '{}'", self,)
        }

        self
    }
}

impl Mul for Type {
    type Output = Type;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Type::Int, Type::Int) => self,
            (Type::Float, Type::Float) => self,

            // Implicitly convert int to float
            (Type::Float, Type::Int) | (Type::Int, Type::Float) => Type::Float,

            _ => panic!(
                "Type mismatch: cannot apply 'mul' operator on '{}' and '{}'",
                self, rhs,
            ),
        }
    }
}

impl Div for Type {
    type Output = Type;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Type::Int, Type::Int) => self,
            (Type::Float, Type::Float) => self,

            // Implicitly convert int to float
            (Type::Float, Type::Int) | (Type::Int, Type::Float) => Type::Float,

            _ => panic!(
                "Type mismatch: cannot apply 'div' operator on '{}' and '{}'",
                self, rhs
            ),
        }
    }
}

impl Rem for Type {
    type Output = Type;

    fn rem(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Type::Int, Type::Int) => self,
            (Type::Float, Type::Float) => self,

            // Implicitly convert int to float
            (Type::Float, Type::Int) | (Type::Int, Type::Float) => Type::Float,

            _ => panic!(
                "Type mismatch: cannot apply 'rem' operator on '{}' and '{}'",
                self, rhs,
            ),
        }
    }
}

impl Add for Type {
    type Output = Type;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Type::Int, Type::Int) => self,
            (Type::Float, Type::Float) => self,

            // Implicitly convert int to float
            (Type::Float, Type::Int) | (Type::Int, Type::Float) => Type::Float,

            _ => panic!(
                "Type mismatch: cannot apply 'add' operator on '{}' and '{}'",
                self, rhs,
            ),
        }
    }
}

impl Sub for Type {
    type Output = Type;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Type::Int, Type::Int) => self,
            (Type::Float, Type::Float) => self,

            // Implicitly convert int to float
            (Type::Float, Type::Int) | (Type::Int, Type::Float) => Type::Float,

            _ => panic!(
                "Type mismatch: cannot apply 'sub' operator on '{}' and '{}'",
                self, rhs
            ),
        }
    }
}

impl Neg for Type {
    type Output = Type;

    fn neg(self) -> Self::Output {
        match self {
            Type::Int => self,
            Type::Float => self,

            _ => panic!("Type mismatch: cannot apply 'neg' operator on '{}'", self,),
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Type::Int, Type::Int) => self.partial_cmp(other),
            (Type::Float, Type::Float) => self.partial_cmp(other),

            _ => panic!("Type mismatch: cannot compare '{}' and '{}'", self, other),
        }
    }
}

impl BitAnd for Type {
    type Output = Type;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Type::Bool, Type::Bool) => self,

            _ => panic!(
                "Type mismatch: cannot apply 'bitand' operator on '{}' and '{}'",
                self, rhs
            ),
        }
    }
}

impl BitOr for Type {
    type Output = Type;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Type::Bool, Type::Bool) => self,

            _ => panic!(
                "Type mismatch: cannot apply 'bitor' operator on '{}' and '{}'",
                self, rhs
            ),
        }
    }
}
