//! Contains functionality regarding conditions based on `CfgValue`s.
//! 
//! **TODO: FILL THIS IN**

use std::ops::{BitAnd, BitOr};

/// Trait for the `check_that` function, that allows it to run a condition on a struct.
pub trait Checkable {
    /// Checks whether the object satisfies the condition passed as `c`.
    /// 
    /// Note that the `condition` can be chained using `.and` (&) and `.or` (|).
    fn check_that(&self, condition: Condition) -> bool;
}

/// Different possible conditions.
/// 
/// Many conditions are self explanatory, such as `Is_Int` and `Exists`.
/// Complex conditions can be created easily using the `|` and `&` operators.
/// So, for example, if you want to check whether an enum is an integer, *or* a float,
/// you can do the following:
/// 
/// ```
/// # use cmap::{CfgValue::*, Condition::*, Checkable};
/// # let value = Int(5);
/// value.check_that(Is_Int | Is_Float);
/// ```
/// 
/// If you'd rather use methods, the following is equivalent:
/// ```
/// # use cmap::{CfgValue::*, Condition::*, Checkable};
/// # let value = Int(5);
/// value.check_that(Is_Int.or(Is_Float));
/// ```
/// 
/// Both of the above examples expand to the following:
/// ```
/// # use cmap::{CfgValue::*, Condition::*, Checkable};
/// # let value = Int(5);
/// value.check_that(Or(Box::new(Is_Int), Box::new(Is_Float)));
/// ```
pub enum Condition {
    Is_Int,
    Is_Float,
    Is_Str,
    Is_IRange,
    Is_FRange,
    Is_List,

    /// Equivalent to doing `Is_List | Is_IRange | Is_FRange`.
    Is_Listlike,

    Is_Map,
    /// A combination of two conditions.
    /// 
    /// If both evaluate to `TRUE`, the result is `TRUE`, otherwise it is `FALSE`.
    And(Box<Condition>, Box<Condition>),

    /// A combination of two conditions.
    /// 
    /// If one evaluates to `TRUE`, the result is `TRUE`, otherwise it is `FALSE`.
    Or(Box<Condition>, Box<Condition>),

    /// A result condition. When executed this will always return TRUE.
    TRUE,

    /// A result condition. When executed this will always return FALSE.
    FALSE
}

impl Condition {

    /// Helper function to generate an `AND` condition.
    pub fn and(self, other: Condition) -> Condition {
        Condition::And(Box::new(self), Box::new(other))
    }

    /// Helper function to generate an `OR` condition.
    pub fn or(self, other: Condition) -> Condition {
        Condition::Or(Box::new(self), Box::new(other))
    }

    /// Executes the condition. For all conditions, this function
    /// will return one of the result conditions - `TRUE` or `FALSE`.
    /// All conditions are executed on the input that is passed - including 
    /// conditions within `AND` and `OR` combinations.
    /// 
    /// ## Examples
    /// 
    /// ```ignore
    /// use cmap::{Condition::*, CfgValue::*};
    /// assert!(Is_Int.execute(Int(5)).to_bool()); 
    /// assert!(!Is_Int.execute(Float(1.0)).to_bool());
    /// assert!((Is_Int | Is_Float).execute(Float(1.0)).to_bool());
    /// ```
    pub fn execute(&self, input: &super::CfgValue) -> Condition {
        use Condition::*;

        match self {
            Is_Int => Condition::from_bool(input.is_int()),
            Is_Float => Condition::from_bool(input.is_float()),
            Is_Str => Condition::from_bool(input.is_str()),
            Is_IRange => Condition::from_bool(input.is_irange()),
            Is_FRange => Condition::from_bool(input.is_frange()),
            Is_List => Condition::from_bool(input.is_list()),
            Is_Map => Condition::from_bool(input.is_map()),
            TRUE => TRUE,
            FALSE => FALSE,
            And(x,y) => {
                let res1 = x.execute(input);
                let res2 = y.execute(input);

                if res1.to_bool() && res2.to_bool() {
                    TRUE
                } else {FALSE}
            },
            Or(x,y) => {
                let res1 = x.execute(input);
                let res2 = y.execute(input);

                if res1.to_bool() || res2.to_bool() {
                    TRUE
                } else {FALSE}
            },
            Is_Listlike => (Is_List | Is_IRange | Is_FRange).execute(input),
        }
    }

    /// Converts a bool into one of the result conditions.
    fn from_bool(b: bool) -> Condition {
        if b {Condition::TRUE} else {Condition::FALSE}
    }

    /// Converts from result condition to boolean.
    /// 
    /// All non-TRUE values are interpreted as FALSE.
    /// Reasoning behind this is that all other values are either incomplete conditions,
    /// or FALSE.
    pub fn to_bool(&self) -> bool {
        if let Condition::TRUE = self { true } else { false }
    }
}

/// Syntactical sugar for `a.and(b)`.
impl BitAnd for Condition {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

/// Syntactical sugar for `a.or(b)`.
impl BitOr for Condition {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}