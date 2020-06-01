// PUBLIC MACROS

#[macro_export]
/// Creates a `CfgValue` value using the passed variable.
///
/// ## Examples:
/// ```
/// # use cfgmap::{CfgMap, Condition::*, Checkable, value};
/// let s = value!(4);
/// let x = value!(3.2);
/// let y = value!("hello there");
/// let a = value!(vec![value!(3), value!(9.4), value!("amazing")]);
/// let m = value!(CfgMap::new());
///
/// assert!(s.check_that(IsInt));
/// assert!(x.check_that(IsFloat));
/// assert!(y.check_that(IsStr));
/// assert!(a.check_that(IsListWith(Box::new(IsInt | IsFloat | IsStr))));
/// assert!(m.check_that(IsMap));
/// ```
macro_rules! value {
    ($($tt:tt)*) => {
        {
            let __value: $crate::CfgValue = ($($tt)*).into();
            __value
        }
    };
}

#[macro_export]
/// Creates a `CfgValue::List` from the values passed.
/// Works very similarly to the `vec!` macro.
///
/// ## Examples:
/// ```
/// # use cfgmap::{Condition::*, Checkable, value, list};
/// let arr1 = list![2, 3.2, "hello there"];
/// let arr2 = value!(vec![value!(2), value!(3.2), value!("hello there")]);
///
/// assert_eq!(arr1, arr2);
/// ```
macro_rules! list {
    ($($tt:tt),*) => {
        value!(vec![$(value!($tt)),*])
    };
}

// MACROS for implementing FROM trait.

macro_rules! from_int {
    ($($type:ty),*) => {
        $(
        impl From<$type> for CfgValue {
            fn from(i: $type) -> Self {
                CfgValue::Int(i.into())
            }
        }
        )*
    };
}

macro_rules! from_float {
    ($($type:ty),*) => {
        $(
        impl From<$type> for CfgValue {
            fn from(f: $type) -> Self {
                CfgValue::Float(f.into())
            }
        }
        )*
    };
}

macro_rules! from_str {
    ($($type:ty),*) => {
        $(
        impl From<$type> for CfgValue {
            fn from(s: $type) -> Self {
                CfgValue::Str(s.into())
            }
        }
        )*
    };
}

// MACROS for documenting, and implementing "is", "as" and "as_mut".

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! is_type {
    ($fn_name:ident, $enum_type:path) => {
        doc_comment! {
            concat!("Checks whether the enum is a `", stringify!($enum_type), "`."),
            pub fn $fn_name (&self) -> bool {
                if let $enum_type(..) = self {
                    true
                } else { false }
            }
        }
    };

    ($fn_name:ident [0], $enum_type:path) => {
        doc_comment! {
            concat!("Checks whether the enum is a `", stringify!($enum_type), "`."),
            pub fn $fn_name (&self) -> bool {
                if let $enum_type = self {
                    true
                } else { false }
            }
        }
    }
}

macro_rules! as_type {
    ($fn_name:ident, $type:ty, $enum_type:path) => {
        doc_comment! {
            concat!("Returns a reference to the `", stringify!($type),
                    "`. Result is `None` if contents aren't a `", stringify!($enum_type), "`."),
            pub fn $fn_name (&self) -> Option<&$type> {
                if let $enum_type(x) = self {
                    Some(x)
                } else { None }
            }
        }
    };
}

macro_rules! as_mut_type {
    ($fn_name:ident, $type:ty, $enum_type:path) => {
        doc_comment! {
            concat!("Returns a reference to the `", stringify!($type),
                    "`. Result is `None` if contents aren't a `", stringify!($enum_type), "`."),
            pub fn $fn_name (&mut self) -> Option<&mut $type> {
                if let $enum_type(x) = self {
                    Some(x)
                } else { None }
            }
        }
    };
}