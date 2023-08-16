use std::{collections::HashMap, hash::Hash};

use super::{ASType, ASVariable, KeyVar};

pub trait IsASVar
where
    Self: Sized,
{
    fn into_adventure_var(self) -> ASVariable;

    fn from_adventure_var(var: &ASVariable) -> Option<Self>;

    fn into_adventure_type(self) -> ASType {
        self.into_adventure_var().get_type()
    }
}

pub trait ASVarByRef: IsASVar {
    fn adventure_var(&self) -> ASVariable;

    fn adventure_type(&self) -> ASType;
}

impl<T> ASVarByRef for T
where
    T: Clone + IsASVar,
{
    fn adventure_var(&self) -> ASVariable {
        self.clone().into_adventure_var()
    }

    fn adventure_type(&self) -> ASType {
        self.clone().into_adventure_type()
    }
}

pub trait ASKeyVar: IsASVar + ASVarByRef + Hash + Eq {
    fn from_key_var(var: &KeyVar) -> Option<Self> {
        Self::from_adventure_var(&var.get())
    }

    fn into_key_var(self) -> KeyVar {
        KeyVar::new(self.into_adventure_var()).expect("This type should not implement ASKeyVar")

    }

    fn key_var(&self) -> KeyVar {
        KeyVar::new(self.adventure_var()).expect("This type should not implement ASKeyVar")
        
    }
}

// ---------------------------

impl IsASVar for ASVariable {
    fn from_adventure_var(var: &ASVariable) -> Option<Self> {
        Some(var.clone())
    }
    fn into_adventure_var(self) -> ASVariable {
        self
    }
}

// ---------------------------

impl<T> IsASVar for Option<T>
where
    T: IsASVar,
{
    fn into_adventure_var(self) -> ASVariable {
        match self {
            Some(c) => c.into_adventure_var(),
            None => ASVariable::None,
        }
    }

    fn from_adventure_var(var: &ASVariable) -> Option<Self> {
        match var {
            ASVariable::None => Some(None),
            _ => T::from_adventure_var(var).map(|c| Some(c)),
        }
    }
}

// ---------------------------

impl IsASVar for i64 {
    fn into_adventure_var(self) -> ASVariable {
        ASVariable::Int(self)
    }

    fn from_adventure_var(var: &ASVariable) -> Option<Self> {
        match var {
            ASVariable::Int(c) => Some(*c),
            _ => None,
        }
    }
}

impl ASKeyVar for i64 {}

// ---------------------------

impl IsASVar for String {
    fn into_adventure_var(self) -> ASVariable {
        ASVariable::String(self)
    }

    fn from_adventure_var(var: &ASVariable) -> Option<Self> {
        match var {
            ASVariable::String(c) => Some(c.clone()),
            _ => None,
        }
    }
}

impl ASKeyVar for String {}

// ---------------------------

impl IsASVar for bool {
    fn into_adventure_var(self) -> ASVariable {
        ASVariable::Bool(self)
    }

    fn from_adventure_var(var: &ASVariable) -> Option<Self> {
        match var {
            ASVariable::Bool(c) => Some(*c),
            _ => None,
        }
    }
}

impl ASKeyVar for bool {}

// ---------------------------

impl<T> IsASVar for Vec<T> where T: IsASVar {
    fn into_adventure_var(self) -> ASVariable {
        let mut out = vec![];
        for val in self {
            out.push(val.into_adventure_var())
        }
        ASVariable::List(out)
    }
    fn from_adventure_var(var: &ASVariable) -> Option<Self> {
        match var {
            ASVariable::List(c) => {
                let mut out = vec![];
                for val in c {
                    out.push(T::from_adventure_var(val)?)
                }
                Some(out)
            },
            _ => None,
        }
    }
}

// ---------------------------

impl<K, V> IsASVar for HashMap<K, V> where K: ASKeyVar, V: IsASVar {
    fn into_adventure_var(self) -> ASVariable {
        let mut out = HashMap::new();
        for (k, v) in self {
            out.insert(k.into_key_var(), v.into_adventure_var());
        }
        ASVariable::Map(out)
    }

    fn from_adventure_var(var: &ASVariable) -> Option<Self> {
        match var {
            ASVariable::Map(c) => {
                let mut out = HashMap::new();
                for (k, v) in c {
                    out.insert(K::from_key_var(k)?, V::from_adventure_var(v)?);
                }
                Some(out)
            },
            _ => None,
        }
    }
}

// ---------------------------


// TODO: Label, VarRef, Object