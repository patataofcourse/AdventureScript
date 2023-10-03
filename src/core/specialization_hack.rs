use core::mem::MaybeUninit;

use super::{ASVariable, IsASVar};

// autoderef "specialization" magic
// courtesy of aleok (https://github.com/aleokdev), with some modifications for my specific thing

/// Implemented by only Option<T>.
pub trait Optional {
    type Inner;
}
impl<T> Optional for Option<T> {
    type Inner = T;
}

pub trait OptionInfo {
    fn is_optional(&self) -> bool;

    type AsOptional;
    type Unwrapped;
    type InnerType: IsASVar;
    fn unwrap_if_optional(&self, in_: Self::AsOptional) -> Self::Unwrapped;
    fn from_adventure_var(&self, t: &ASVariable) -> Option<Self::InnerType> {
        Self::InnerType::from_adventure_var(t)
    }
}

pub struct Wrap<T>(T);
impl<T> std::ops::Deref for Wrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Wrap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Optional> OptionInfo for Wrap<Wrap<&MaybeUninit<&T>>>
where
    T::Inner: IsASVar,
{
    fn is_optional(&self) -> bool {
        true
    }

    type AsOptional = T;
    type Unwrapped = T;
    type InnerType = T::Inner;
    fn unwrap_if_optional(&self, in_: Self::AsOptional) -> Self::Unwrapped {
        in_
    }
}

impl<T: IsASVar> OptionInfo for Wrap<&MaybeUninit<&T>> {
    fn is_optional(&self) -> bool {
        false
    }

    type AsOptional = Option<T>;
    type Unwrapped = T;
    type InnerType = T;
    fn unwrap_if_optional(&self, in_: Self::AsOptional) -> Self::Unwrapped {
        in_.unwrap()
    }
}

// for convenience

impl<T> Wrap<T> {
    pub fn new() -> Wrap<Wrap<MaybeUninit<&'static T>>> {
        Wrap(Wrap(MaybeUninit::uninit()))
    }
}

impl<T> Wrap<Wrap<MaybeUninit<&'static T>>> {
    pub fn refer(&self) -> Wrap<Wrap<&MaybeUninit<&'static T>>> {
        Wrap(Wrap(&self.0 .0))
    }
}
