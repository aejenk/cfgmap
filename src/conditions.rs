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
/// 
/// If you'd like to not only check the type of a `CfgValue`, but also the *value* its wrapped in,
/// you can use the `Exactly` conditions:
/// ```
/// # use cfgmap::{CfgValue::*, Condition::*, Checkable};
/// let value = Int(5);
/// assert!(value.check_that(IsExactlyInt(5)));
/// ```
/// 
/// These exist for all `CfgValue`s. There also exist other miscellaneous conditions, such as
/// `IsListWithLength(usize)` or `IsListWith(Box<Condition>)`, which serve other purposes.
#[derive(Clone)]
pub enum Condition {
    IsInt,
    IsFloat,
    IsStr,
    IsList,
    IsBool,

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

    /// Does an exact comparison with an integer.
    IsExactlyInt(super::_Int),

    /// Does an exact comparison with an float.
    IsExactlyFloat(super::_Float),

    /// Does an exact comparison with a string.
    IsExactlyStr(super::_Str),

    /// Does an exact comparison with a `Vec<CfgValue>`.
    IsExactlyList(Vec<super::CfgValue>),

    /// Does an exact comparison with a `CfgMap`
    IsExactlyMap(super::CfgMap),

    /// Verifies it to be a `Bool`, and checks whether it is true.
    IsTrue,

    /// Verifies it to be a `List` and applies the condition to each of its elements.
    IsListWith(Box<Condition>),

    /// Verifies it to be a `List`, while also having a specific length.
    IsListWithLength(usize),

    #[cfg(feature = "from_json")]
    /// Verifies the value to be `null`. Only availiable while using `from_json`.
    IsNull,

    #[cfg(feature = "from_toml")]
    /// Verifies the value to be a `Datetime`. Only available while using `from_toml`.
    IsDatetime,

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
    /// ```
    /// use cfgmap::{Condition::*, CfgValue::*};
    /// assert!(IsInt.execute(&Int(5)).to_bool()); 
    /// assert!(!IsInt.execute(&Float(1.0)).to_bool());
    /// assert!((IsInt | IsFloat).execute(&Float(1.0)).to_bool());
    /// ```
    pub fn execute(&self, input: &super::CfgValue) -> Condition {
        use Condition::*;

        match self {
            // Basic conditions.
            IsInt => input.is_int().into(),
            IsFloat => input.is_float().into(),
            IsStr => input.is_str().into(),
            IsList => input.is_list().into(),
            IsMap => input.is_map().into(),
            IsBool => input.is_bool().into(),
            TRUE => TRUE,
            FALSE => FALSE,

            // Combined conditions.
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

            // Exact condition.
            IsExactlyInt(s) => input.as_int().map_or(false, |i| *i == *s).into(),
            IsExactlyFloat(s) => input.as_float().map_or(false, |f| *f == *s).into(),
            IsExactlyStr(s) => input.as_str().map_or(false, |st| *st == *s).into(),
            IsExactlyList(s) => input.as_list().map_or(false, |l| *l == *s).into(),
            IsExactlyMap(s) => input.as_map().map_or(false, |l| *l == *s).into(),
            IsTrue => input.as_bool().map_or(false, |b| *b).into(),

            // Miscellaneous.
            IsListWith(s) => {
                input.as_list().map(|list| {
                    for elem in list.iter() {
                        if !elem.check_that((**s).clone()) {
                            return FALSE;
                        }
                    }
                    TRUE
                }).map_or(FALSE, |o| o.into())
            },

            IsListWithLength(l) => input.as_list().map_or(false, |li| *l == li.len()).into(),

            // Feature-dependent.

            #[cfg(feature = "from_json")]
            IsNull => input.is_null().into(),

            #[cfg(feature = "from_toml")]
            IsDatetime => input.is_datetime().into(),
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

impl From<bool> for Condition {
    fn from(b: bool) -> Self {
        Condition::from_bool(b)
    }
}

impl From<Condition> for bool {
    fn from(c: Condition) -> Self {
        c.to_bool()
    }
}

#[cfg(test)]
mod test {
    use crate::{CfgMap, CfgValue::*, Condition::*, Checkable};

    #[test]
    fn basic_and_exact() {
        let i = Int(5);
        let f = Float(2.0);
        let s = Str(String::from("hello"));
        let b = Bool(true);
        let l = List(vec![Int(2), Float(8.0)]);
        let m = Map(CfgMap::new());

        // Verifies int
        assert!(i.check_that(IsInt));
        assert!(i.check_that(IsExactlyInt(5)));
        assert!(!i.check_that(IsExactlyInt(6)));

        // Verifies float
        assert!(f.check_that(IsFloat));
        assert!(f.check_that(IsExactlyFloat(2.0)));
        assert!(!f.check_that(IsExactlyFloat(3.0)));

        // Verifies string
        assert!(s.check_that(IsStr));
        assert!(s.check_that(IsExactlyStr(String::from("hello"))));
        assert!(!s.check_that(IsExactlyStr(String::from("hella"))));

        // Verifies bool
        assert!(b.check_that(IsBool));
        assert!(b.check_that(IsTrue));

        // Verifies list
        assert!(l.check_that(IsList));
        assert!(l.check_that(IsExactlyList(vec![Int(2), Float(8.0)])));
        assert!(!l.check_that(IsExactlyList(vec![Int(2), Float(8.1)])));

        // Verifies map
        assert!(m.check_that(IsMap));
        assert!(m.check_that(IsExactlyMap(CfgMap::new())));
        let mut map = CfgMap::new();
        map.default = "default".into();
        assert!(!m.check_that(IsExactlyMap(map)));
    }

    #[test]
    fn combinations() {
        vec![Int(5), Float(9.0), Str(String::from("foobar"))]
            .iter()
            .for_each(|e| assert!(e.check_that(IsInt | IsFloat | IsStr)));

        vec![Int(5), Float(9.0), Str(String::from("foobar"))]
            .iter()
            .for_each(|e| assert!(!e.check_that(IsList | IsMap)));

        vec![Int(5), Float(9.0), Str(String::from("foobar"))]
            .iter()
            .for_each(|e| assert!(!e.check_that(IsInt & IsFloat)));
    }

    #[test]
    fn misc() {
        let listexample = List(vec![Int(5), Float(9.0)]);

        assert!(listexample.check_that(IsListWith(Box::new(IsInt | IsFloat))));
        assert!(listexample.check_that(IsListWithLength(2)));
        assert!(!listexample.check_that(IsListWithLength(3)));
    }

}