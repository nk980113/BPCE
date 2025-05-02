//! Provides types to handle assets (or asset links/references) in a chart, which will be stored in alternative storage.

use std::mem;

/// See https://github.com/rust-lang/rust/issues/20041#issuecomment-2316329375
pub trait TyEq
where
    Self: From<Self::Type> + Into<Self::Type>,
    Self::Type: From<Self> + Into<Self>,
{
    type Type;
}

impl<T> TyEq for T {
    type Type = T;
}

pub trait AssetReplacement<'a, A: GenericDescriptor + 'a> {
    type This<Inner> where
        Self: TyEq<Type = Self::This<AssetHolder<'a, A>>> + Into<Self::This<A>> + Into<Self::This<&'a str>>,
        Self::This<&'a str>: Into<Self>,
        Self::This<A>: Into<Self>;

    fn points(&mut self) -> impl Iterator<Item = &'a mut AssetHolder<'a, A>>;
}

pub enum AssetHolder<'a, Processed: GenericDescriptor> {
    Raw(&'a str),
    Processed(Processed),
}

impl<'a, Processed: GenericDescriptor> AssetHolder<'a, Processed> {
    pub fn collect(&mut self, store: impl FnMut(&str) -> u16) {
        let &mut Self::Raw(route) = self else { panic!("Called `AssetPlaceholder::collect()` on a `Processed` value"); };
        let _ = mem::replace(self, Self::Processed(GenericDescriptor::from_desc(route, store)));
    }
    
    pub fn expand<'inner: 'a>(&mut self, pick: impl Fn(u16) -> &'inner str) {
        let &mut Self::Processed(ref processed) = self else { panic!("Called `AssetPlaceholder::expand()` on a `Raw` value"); };
        let _ = mem::replace(self, Self::Raw(processed.describe(pick)));
    }

    pub fn into_processed(self) -> Processed {
        let Self::Processed(value) = self else { panic!("Cannot cast from `AssetHolder::Raw` value to the underlying processed type") };
        value
    }
}

impl<T: GenericDescriptor> From<T> for AssetHolder<'_, T> {
    fn from(value: T) -> Self {
        Self::Processed(value)
    }
}

impl<'a, T: GenericDescriptor> From<&'a str> for AssetHolder<'a, T> {
    fn from(value: &'a str) -> Self {
        Self::Raw(value)
    }
}

impl<'a, T: GenericDescriptor> From<AssetHolder<'a, T>> for &'a str {
    fn from(value: AssetHolder<'a, T>) -> Self {
        let AssetHolder::Raw(value) = value else { panic!("Cannot cast from `AssetHolder::Processed` value to raw value (&str)") };
        value
    }
}

pub trait GenericDescriptor {
    fn from_desc(desc: &str, store: impl FnMut(&str) -> u16) -> Self;
    fn describe<'a>(&self, pick: impl Fn(u16) -> &'a str) -> &'a str;
}

impl GenericDescriptor for u16 {
    fn from_desc(desc: &str, mut store: impl FnMut(&str) -> u16) -> Self {
        return store(desc);
    }

    fn describe<'a>(&self, pick: impl Fn(u16) -> &'a str) -> &'a str {
        return pick(*self);
    }
}
