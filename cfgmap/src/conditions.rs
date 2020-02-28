use std::ops::{BitAnd, BitOr, Not};

/// Trait for the `check_that` function, that allows it to run a condition on a struct.
pub trait Checkable {
    /// Checks whether the object satisfies the condition passed as `c`.
    /// 
    /// Note that the `condition` can be chained using `.and` (&) and `.or` (|).
    fn check_that(&self, condition: Condition) -> bool;
}

/// Different possible conditions.
/// 
/// Many conditions are self explanatory, such as `IsInt` and `IsList`.
/// Complex conditions can be created easily using the `|` and `&` operators.
/// So, for example, if you want to check whether an enum is an integer, *or* a float,
/// you can do the following:
/// 
/// ```
/// # use cfgmap::{CfgValue::*, Condition::*, Checkable};
/// # let value = Int(5);
/// value.check_that(IsInt | IsFloat);
/// ```
/// 
/// If you'd rather use methods, the following is equivalent:
/// ```
/// # use cfgmap::{CfgValue::*, Condition::*, Checkable};
/// # let value = Int(5);
/// value.check_that(IsInt.or(IsFloat));
/// ```
/// 
/// Both of the above examples expand to the following:
/// ```
/// # use cfgmap::{CfgValue::*, Condition::*, Checkable};
/// # let value = Int(5);
/// value.check_that(Or(Box::new(IsInt), Box::new(IsFloat)));
/// ```
pub enum Condition {
    IsInt,
    IsFloat,
    IsStr,
    IsList,

    IsMap,
    /// A combination of two conditions.
    /// 
    /// If both evaluate to `TRUE`, the result is `TRUE`, otherwise it is `FALSE`.
    And(Box<Condition>, Box<Condition>),

    /// A combination of two conditions.
    /// 
    /// If one evaluates to `TRUE`, the result is `TRUE`, otherwise it is `FALSE`.
    Or(Box<Condition>, Box<Condition>),

    /// Represents a negation.
    Not(Box<Condition>),

    /// A result condition. When executed this will always return `true`.
    TRUE,

    /// A result condition. When executed this will always return `false`.
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

    /// Helper function to generate a `NOT` condition.
    pub fn not(self) -> Condition {
        Condition::Not(Box::new(self))
    }

    /// Executes the condition. For all conditions, this function
    /// will return one of the result conditions - `TRUE` or `FALSE`.
    /// All conditions are executed on the input that is passed - including 
    /// conditions within `AND` and `OR` combinations.
    /// 
    /// ## Examples
    /// 
    /// ```ignore
    /// use cfgmap::{Condition::*, CfgValue::*};
    /// assert!(IsInt.execute(Int(5)).to_bool()); 
    /// assert!(!IsInt.execute(Float(1.0)).to_bool());
    /// assert!((IsInt | IsFloat).execute(Float(1.0)).to_bool());
    /// ```
    pub fn execute(&self, input: &super::CfgValue) -> Condition {
        use Condition::*;

        match self {
            IsInt => Condition::from_bool(input.is_int()),
            IsFloat => Condition::from_bool(input.is_float()),
            IsStr => Condition::from_bool(input.is_str()),
            IsList => Condition::from_bool(input.is_list()),
            IsMap => Condition::from_bool(input.is_map()),
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
            Not(x) => {
                let res = x.execute(input);

                if res.to_bool() { FALSE } else { TRUE }
            },
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

/// Syntactical sugar for `a.not()`
impl Not for Condition {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}