use std::{collections::HashMap, hash::Hash};

use super::{ASType, ASVariable, KeyVar};

pub trait IsASVar
where
    Self: Sized,
{
    const ADVENTURE_TYPE: ASType;

    fn into_adventure_var(self) -> ASVariable;

    fn from_adventure_var(var: &ASVariable) -> Option<Self>;

    fn into_adventure_type(self) -> ASType {
        Self::ADVENTURE_TYPE
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

pub trait ASVarWrapTo: IsASVar {
    type InnerType: IsASVar;
    const INNER_AS_TYPE: ASType = Self::InnerType::ADVENTURE_TYPE;

    fn wrap(vars: Vec<Self::InnerType>) -> Option<Self>;
}

// ---------------------------

impl IsASVar for ASVariable {
    const ADVENTURE_TYPE: ASType = ASType::Any;

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
    const ADVENTURE_TYPE: ASType = T::ADVENTURE_TYPE;

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

    fn into_adventure_type(self) -> ASType {
        self.into_adventure_var().get_type()
    }
}

// ---------------------------

impl IsASVar for i64 {
    const ADVENTURE_TYPE: ASType = ASType::Int;

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
    const ADVENTURE_TYPE: ASType = ASType::String;

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
    const ADVENTURE_TYPE: ASType = ASType::Bool;

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

impl<T> IsASVar for Vec<T>
where
    T: IsASVar,
{
    const ADVENTURE_TYPE: ASType = ASType::List;

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
            }
            _ => None,
        }
    }

    fn into_adventure_type(self) -> ASType {
        let as_type = T::ADVENTURE_TYPE;

        // If it's ASVariable...
        if as_type == ASType::Any {
            // ...return regular List
            ASType::List
        } else {
            // Otherwise, use ListExplicit
            ASType::ListExplicit(Box::new(as_type))
        }
    }
}

impl<T> ASVarWrapTo for Vec<T>
where
    T: IsASVar,
{
    type InnerType = T;

    fn wrap(vars: Vec<Self::InnerType>) -> Option<Self> {
        Some(vars)
    }
}

// ---------------------------

impl<K, V> IsASVar for HashMap<K, V>
where
    K: ASKeyVar,
    V: IsASVar,
{
    const ADVENTURE_TYPE: ASType = ASType::Map;

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
            }
            _ => None,
        }
    }

    fn into_adventure_type(self) -> ASType {
        let k_type = K::ADVENTURE_TYPE;
        let v_type = V::ADVENTURE_TYPE;

        // If both are ASVariable...
        if k_type == ASType::Any && v_type == ASType::Any {
            // ...return regular Map
            ASType::Map
        } else {
            // Otherwise, use MapExplicit
            ASType::MapExplicit(Box::new(k_type), Box::new(v_type))
        }
    }
}

// ---------------------------

// TODO: Label, VarRef, Object
