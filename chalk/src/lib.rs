#![allow(dead_code)] // temporary
#![feature(question_mark)]
#![feature(static_in_const)]

extern crate chalk_parse;
extern crate lalrpop_intern;
extern crate ena;

/// Create a deref impl. We do this a lot.
macro_rules! deref_to {
    ($source:ident<$($param:ident),*>.$field:ident => $target:ty) => {
        impl<$($param),*> ::std::ops::Deref for $source<$($param),*> {
            type Target = $target;

            fn deref(&self) -> &$target {
                &self.$field
            }
        }
    };

    ($source:ident.$field:ident => $target:ty) => {
        impl ::std::ops::Deref for $source {
            type Target = $target;

            fn deref(&self) -> &$target {
                &self.$field
            }
        }
    };
}

mod formula;
mod program;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}