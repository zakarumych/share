
use std::{borrow::Borrow, ops::Deref};

/// Simple trait that can be implemented for any type via `shareable!` trait.
pub trait Shareable<'a> {
    /// Either `&'a Self` or `Self` when `Self` is `Copy`.
    #[doc(hidden)]
    type Shared: Copy + Borrow<Self> + 'a;

    /// Share reference.
    #[doc(hidden)]
    fn share(&'a self) -> Self::Shared;
}

/// Shared value reference.
/// It can store either reference or copy of the value.
pub struct Share<'a, T: ?Sized + Shareable<'a>> {
    shared: T::Shared,
}

impl<'a, T> Share<'a, T>
where
    T: ?Sized + Shareable<'a>,
{
    /// Construct new `Share` from reference.
    pub fn new(reference: &'a T) -> Self {
        Share {
            shared: reference.share(),
        }
    }
}

impl<'a, T> Deref for Share<'a, T>
where
    T: ?Sized + Shareable<'a>,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.shared.borrow()
    }
}

/// References are share-by-copy.
impl<'a, T> Shareable<'a> for &'a T
where
    T: 'a,
{
    type Shared = &'a T;
    fn share(&'a self) -> &'a T {
        *self
    }
}

#[macro_export]
macro_rules! shareable {
    ($t:ty) => {
        impl<'a> Shareable<'a> for $t {
            type Shared = &'a $t;
            fn share(&'a self) -> &'a $t {
                self
            }
        }
    };
    ($t:ty: Copy) => {
        impl<'a> Shareable<'a> for $t {
            type Shared = $t;
            fn share(&self) -> $t {
                *self
            }
        }
    };
}



#[test]
fn copy() {
    #[derive(Copy, Clone)]
    struct Foo;
    shareable!(Foo: Copy);

    // Copy.
    let _: Foo = Share::new(&Foo).shared;
}


#[test]
fn reference() {
    #[derive(Clone)]
    struct Foo;
    shareable!(Foo);

    // Ref.
    let _: &Foo = Share::new(&Foo).shared;
}
