//! Tests related to integer/float variable kinds

use super::*;

/// If we know that the type is an integer, we can narrow down the possible
/// types. This test is based on the following example:
/// ```ignore
/// let x: &[u32];
/// let i = 1;
/// x[i]
/// ```
/// `i` must be `usize` because that is the only integer type used in `Index`
/// impls for slices.
#[test]
fn integer_index() {
    test! {
        program {
            trait Index<T> {}
            struct Slice {}
            struct Foo {}

            impl Index<usize> for Slice {}
            impl Index<Foo> for Slice {}
        }

        goal {
            exists<int N> {
                Slice: Index<N>
            }
        } yields {
            "Unique; substitution [?0 := Uint(Usize)]"
        }
    }
}

/// A more straightforward version of the `integer_index` test where the
/// variable is on the impl side of the trait ref.
#[test]
fn integer_kind_trait() {
    test! {
        program {
            trait Foo {}
            struct Bar {}

            impl Foo for usize {}
            impl Foo for Bar {}
        }

        goal {
            exists<int N> {
                N: Foo
            }
        } yields {
            "Unique; substitution [?0 := Uint(Usize)]"
        }
    }
}

/// The `integer_kind_trait` test, but for floats
#[test]
fn float_kind_trait() {
    test! {
        program {
            trait Foo {}
            struct Bar {}

            impl Foo for f32 {}
            impl Foo for Bar {}
        }

        goal {
            exists<float N> {
                N: Foo
            }
        } yields {
            "Unique; substitution [?0 := Float(F32)]"
        }
    }
}

/// You can still get ambiguous results with integer variables
#[test]
fn integer_ambiguity() {
    test! {
        program {
            trait Foo {}

            impl Foo for usize {}
            impl Foo for isize {}
        }

        goal {
            exists<int N> {
                N: Foo
            }
        } yields {
            "Ambiguous; no inference guidance"
        }
    }
}

/// You can still get ambiguous results with float variables
#[test]
fn float_ambiguity() {
    test! {
        program {
            trait Foo {}

            impl Foo for f32 {}
            impl Foo for f64 {}
        }

        goal {
            exists<float N> {
                N: Foo
            }
        } yields {
            "Ambiguous; no inference guidance"
        }
    }
}

/// Integer/float type kinds are just specialized type kinds, so they can unify
/// with general type kinds.
#[test]
fn integer_and_float_are_specialized_ty_kinds() {
    test! {
        program {}

        goal {
            exists<T, int N> {
                T = N, N = usize
            }
        } yields {
            "Unique; substitution [?0 := Uint(Usize), ?1 := Uint(Usize)], lifetime constraints []"
        }

        goal {
            exists<T, float N> {
                T = N, N = f32
            }
        } yields {
            "Unique; substitution [?0 := Float(F32), ?1 := Float(F32)], lifetime constraints []"
        }
    }
}

/// Once a general type kind is unified with a specific type kind, it cannot be
/// unified with an incompatible type (ex. integer type kind with char)
#[test]
fn general_ty_kind_becomes_specific() {
    test! {
        program {}

        goal {
            exists<T, int N> {
                T = N, T = char
            }
        } yields {
            "No possible solution"
        }

        goal {
            exists<T, float N> {
                T = N, T = char
            }
        } yields {
            "No possible solution"
        }
    }
}

/// Integer and float type kinds can not be equated
#[test]
fn integers_are_not_floats() {
    test! {
        program {}

        goal {
            exists<int I, float F> {
                I = F
            }
        } yields {
            "No possible solution"
        }
    }
}
