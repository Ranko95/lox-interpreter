use std::fmt::{self, Display};

#[derive(Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    NilImplicit,
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(v) => write!(f, "{v}"),
            Literal::String(v) => write!(f, "{v}"),
            Literal::Bool(v) => {
                if *v {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Literal::Nil => write!(f, "nil"),
            Literal::NilImplicit => write!(f, "nil_implicit"),
        }
    }
}
