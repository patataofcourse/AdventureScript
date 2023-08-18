use core::mem::MaybeUninit;

// autoderef "specialization" magic
// courtesy of aleok (https://github.com/aleokdev), with some modifications for my specific thing

/// Implemented by only Option<T>.
trait Optional {}
impl<T> Optional for Option<T> {}

pub trait OptionInfo {
    fn is_optional(&self) -> bool;

    type AsOptional;
    type Unwrapped;
    fn unwrap_if_optional(&self, in_: Self::AsOptional) -> Self::Unwrapped;
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

impl<T: Optional> OptionInfo for Wrap<Wrap<&MaybeUninit<&T>>> {
    fn is_optional(&self) -> bool {
        true
    }

    type AsOptional = T;
    type Unwrapped = T;
    fn unwrap_if_optional(&self, in_: Self::AsOptional) -> Self::Unwrapped {
        in_
    }
}

impl<T> OptionInfo for Wrap<&MaybeUninit<&T>> {
    fn is_optional(&self) -> bool {
        false
    }

    type AsOptional = Option<T>;
    type Unwrapped = T;
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
