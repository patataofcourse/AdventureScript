use super::ASVariable;
use crate::core::error::ASSyntaxError;
use std::{
    cmp::{Ordering, PartialOrd},
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

impl Add for ASVariable {
    type Output = anyhow::Result<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        match &self {
            Self::Int(c) => {
                if let ASVariable::Int(c2) = rhs {
                    Ok(ASVariable::Int(c + c2))
                } else {
                    op_err("add".to_string(), self, rhs)
                }
            }
            Self::String(c) => {
                if let ASVariable::String(c2) = rhs {
                    Ok(ASVariable::String(c.to_string() + &c2))
                } else {
                    op_err("add".to_string(), self, rhs)
                }
            }
            //TODO: lists and maps
            _ => op_err("add".to_string(), self, rhs),
        }
    }
}

impl Sub for ASVariable {
    type Output = anyhow::Result<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = rhs {
                return Ok(ASVariable::Int(c - c2));
            }
        };
        op_err("substract".to_string(), self, rhs)
    }
}

impl Mul for ASVariable {
    type Output = anyhow::Result<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        match &self {
            Self::Int(c) => {
                if let ASVariable::Int(c2) = rhs {
                    Ok(ASVariable::Int(c * c2))
                } else {
                    op_err("multiply".to_string(), self, rhs)
                }
            }
            Self::String(c) => {
                if let ASVariable::Int(c2) = rhs {
                    let mut result = String::new();
                    for _ in 0..c2 {
                        result += c;
                    }
                    Ok(ASVariable::String(result))
                } else {
                    op_err("multiply".to_string(), self, rhs)
                }
            }
            Self::List(c) => {
                if let ASVariable::Int(c2) = rhs {
                    let mut result = Vec::new();
                    for _ in 0..c2 {
                        result.extend(c.clone());
                    }
                    Ok(ASVariable::List(result))
                } else {
                    op_err("multiply".to_string(), self, rhs)
                }
            }
            _ => op_err("multiply".to_string(), self, rhs),
        }
    }
}

impl Div for ASVariable {
    type Output = anyhow::Result<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = rhs {
                return Ok(ASVariable::Int(c / c2));
            }
        };
        op_err("divide".to_string(), self, rhs)
    }
}

impl Neg for ASVariable {
    type Output = anyhow::Result<Self>;
    fn neg(self) -> Self::Output {
        match self {
            Self::Int(c) => Ok(Self::Int(-c)),
            _ => unary_op_err("negate".to_string(), self),
        }
    }
}

impl Not for ASVariable {
    type Output = anyhow::Result<Self>;
    fn not(self) -> Self::Output {
        match self {
            Self::Bool(c) => Ok(Self::Bool(!c)),
            _ => unary_op_err("bool-negate".to_string(), self),
        }
    }
}

impl ASVariable {
    /// Meant for use with `Int`-type variables, exponent/power function
    pub fn pow(self, exponent: Self) -> anyhow::Result<Self> {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = exponent {
                return Ok(ASVariable::Int(c.pow(c2 as u32)));
            }
        };
        op_err("calculate the power of".to_string(), self, exponent)
    }
}

impl PartialOrd for ASVariable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            ASVariable::Int(c) => {
                if let ASVariable::Int(d) = other {
                    c.partial_cmp(d)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[inline]
fn op_err(op: String, v1: ASVariable, v2: ASVariable) -> anyhow::Result<ASVariable> {
    Err(ASSyntaxError::OperationNotDefined {
        op,
        type1: v1.get_type(),
        type2: v2.get_type(),
    })?
}

#[inline]
fn unary_op_err(op: String, v: ASVariable) -> anyhow::Result<ASVariable> {
    Err(ASSyntaxError::UnaryOperationNotDefined {
        op,
        type1: v.get_type(),
    })?
}
