use std::collections::HashMap;
pub use crate::conditions::{Checkable, Condition};

macro_rules! is_type {
    ($fn_name:ident, $enum_type:path) => {
        /// Checks whether the enum is a $enum_type.
        pub fn $fn_name (&self) -> bool {
            if let $enum_type(_) = self {
                true
            } else { false }
        }
    };

    ($fn_name:ident [rg], $enum_type:path) => {
        /// Checks whether the enum is a $enum_type.
        pub fn $fn_name (&self) -> bool {
            if let $enum_type(_,_) = self {
                true
            } else { false }
        }
    };

    ($fn_name:ident [0], $enum_type:path) => {
        /// Checks whether the enum is a $enum_type.
        pub fn $fn_name (&self) -> bool {
            if let $enum_type = self {
                true
            } else { false }
        }
    };
}

macro_rules! as_type {
    ($fn_name:ident, $type:ty, $enum_type:path) => {
        /// Returns a reference to the $type. Result is `None` if contents aren't a `$enum_type`.
        pub fn $fn_name (&self) -> Option<&$type> {
            if let $enum_type(x) = self {
                Some(x)
            } else { None }
        }
    };
}

macro_rules! as_mut_type {
    ($fn_name:ident, $type:ty, $enum_type:path) => {
        /// Returns a mutable reference to the $type. Result is `None` if contents aren't a `$enum_type`.
        pub fn $fn_name (&mut self) -> Option<&mut $type> {
            if let $enum_type(x) = self {
                Some(x)
            } else { None }
        }
    };
}

/// Represents a value within a `CfgMap`
/// 
/// **EXTRA STUFF HERE**
#[derive(Debug, Clone)]
pub enum CfgValue {
    /// Represents an integer value.
    Int(isize),

    /// Represents a float value.
    Float(f64),

    /// Represents a string.
    Str(String),

    /// Represents a range.
    /// A list of two integers will be inferred as an IRange.
    IRange(isize, isize),

    /// Represents a range.
    /// A list of two floats will be inferred as an FRange.
    FRange(f64, f64),

    /// Represents a nested configuration map.
    Map(CfgMap),

    /// Represents a list of values. These values can have differing types.
    List(Vec<CfgValue>),
}

impl CfgValue {
    /// Returns the contents of the enum converted into an integer, if possible.
    /// If the enum represents a float, it will be converted into an integer.
    pub fn to_int(&self) -> Option<isize> {
        if let CfgValue::Int(x) = self {
            Some(*x)
        } else if let CfgValue::Float(x) = self {
            Some(*x as isize)
        } else { None }
    }

    /// Returns the contents of the enum converted into a float, if possible.
    /// If the enum represents an integer, it will be converted into a float.
    pub fn to_float(&self) -> Option<f64> {
        if let CfgValue::Float(x) = self {
            Some(*x)
        } else if let CfgValue::Int(x) = self {
            Some(*x as f64)
        } else { None }
    }

    /// Returns a clone of the enum's contents as a list, if possible.
    /// Use this for any listlike types, like `IRange`, and `FRange`.
    /// If you know the contents to be a `List`, and you only want a 
    /// reference, use `as_list` or `as_list_mut` instead.
    pub fn to_list(&self) -> Option<Vec<CfgValue>> {
        if let CfgValue::List(x) = self {
            Some(x.clone())
        } else if let CfgValue::IRange(x, y) = self {
            Some(vec![CfgValue::Int(*x), CfgValue::Int(*y)])
        } else if let CfgValue::FRange(x, y) = self {
            Some(vec![CfgValue::Float(*x), CfgValue::Float(*y)])
        } else { None }
    }

    is_type!(is_int, CfgValue::Int);
    is_type!(is_float, CfgValue::Float);
    is_type!(is_str, CfgValue::Str);
    is_type!(is_map, CfgValue::Map);
    is_type!(is_list, CfgValue::List);
    is_type!(is_irange [rg], CfgValue::IRange);
    is_type!(is_frange [rg], CfgValue::FRange);

    as_type!(as_int, isize, CfgValue::Int);
    as_type!(as_float, f64, CfgValue::Float);
    as_type!(as_str, str, CfgValue::Str);
    as_type!(as_map, CfgMap, CfgValue::Map);
    as_type!(as_list, Vec<CfgValue>, CfgValue::List);

    as_mut_type!(as_mut_int, isize, CfgValue::Int);
    as_mut_type!(as_mut_float, f64, CfgValue::Float);
    as_mut_type!(as_mut_str, str, CfgValue::Str);
    as_mut_type!(as_mut_map, CfgMap, CfgValue::Map);
    as_mut_type!(as_mut_list, Vec<CfgValue>, CfgValue::List);
}

impl conditions::Checkable for CfgValue {
    fn check_that(&self, c: conditions::Condition) -> bool {
        return c.execute(self).is_true();
    }
}

impl conditions::Checkable for Option<&CfgValue> {
    fn check_that(&self, c: conditions::Condition) -> bool {
        self.as_ref().map_or(false, |v| v.check_that(c))
    }
}

impl conditions::Checkable for Option<&mut CfgValue> {
    fn check_that(&self, c: conditions::Condition) -> bool {
        self.as_ref().map_or(false, |v| v.check_that(c))
    }
}

fn split_once(in_string: &str, pat: char) -> (String, Option<String>) {
    let mut splitter = in_string.splitn(2, pat);
    let first = splitter.next().unwrap().to_string();
    let second = splitter.next();
    let second_mod = if let Some(s) = second {
        Some(s.to_string())
    } else {
        None
    };

    (first, second_mod)
}

fn rsplit_once(in_string: &str, pat: char) -> (Option<String>, String) {
    let mut splitter = in_string.rsplitn(2, pat);
    let first = splitter.next().unwrap().to_string();
    let second = splitter.next();
    let second_mod = if let Some(s) = second {
        Some(s.to_string())
    } else {
        None
    };

    (second_mod, first)
}

/// A configuration map, containing helper functions and effectively being a wrapper
/// around two `HashMap`s.
/// 
/// **EXTRA STUFF HERE**
#[derive(Debug, Clone)]
pub struct CfgMap {
    /// An internal map representing the configuration.
    internal_map: HashMap<String, CfgValue>,
    /// A map containing default values.
    defaults: HashMap<String, CfgValue>
}

impl CfgMap {

    /// Creates a new empty CfgMap.
    pub fn new() -> CfgMap {
        CfgMap { internal_map: HashMap::new(), defaults: HashMap::new() }
    }

    /// Adds a new entry in the configuration.
    pub fn add(&mut self, key: String, value: CfgValue){
        let (path, key) = rsplit_once(&key, '/');

        if path.is_none(){
            self.internal_map.insert(key, value);
            // Return successful result.
            unimplemented!()
        }
        else {
            let subtree = self.get_mut(&path.unwrap());
            if subtree.check_that(Condition::Is_Map) {
                subtree.unwrap().as_mut_map().unwrap().add(key, value);
                // Return successful result.
                unimplemented!()
            }
            else {
                // The path doesn't exist, so the addition fails.
                unimplemented!()
            }
        }
    }

    /// Adds a new entry as a default value in the configuration.
    pub fn add_default(&mut self, key: String, value: CfgValue){
        self.defaults.insert(key, value);
    }

    /// Gets a value from within the configuration
    /// Supports using directory notation. For example
    /// 
    /// ```
    /// cmap.get("e1/e2")
    /// ```
    /// 
    /// would attempt to retrieve `e2` from within `e1`.
    /// 
    /// If at any point an element doesn't exist, or isn't a valid map,
    /// the function returns `CfgValue::Empty`.
    pub fn get(&self, key: &str) -> Option<&CfgValue> {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.internal_map.get(key)
        }
        else {
            self.internal_map.get(&h).and_then(|op| {
                op.as_map()
            }).and_then(|map| {
                map.get(&t.unwrap())
            })
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut CfgValue> {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.internal_map.get_mut(key)
        }
        else {
            if let Some(CfgValue::Map(map)) = self.internal_map.get_mut(&h) {
                map.get_mut(&t.unwrap())
            }
            else{
                None
            }
        }
    }

    /// Retrieves a default value from within the configuration.
    /// Same as `get`, but for *default* values.
    pub fn get_default(&self, key: &str) -> Option<&CfgValue> {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.defaults.get(key)
        }
        else {
            if let Some(CfgValue::Map(map)) = self.defaults.get(&h) {
                map.get(&t.unwrap())
            }
            else{
                None
            }
        }
    }

    /// Retrieves an option value from within the configuration.
    /// Similar to using `get("category/option")`, except that if not found it 
    /// attempts to use the default value instead. In terms of priority, it returns:
    /// 
    /// - `get(category/option)`
    /// - `get_default(option)`
    pub fn get_option(&self, category: &str, option: &str) -> Option<&CfgValue> {
        let opt = self.get(&format!("{}/{}", category, option));
        if !opt.is_none() { return opt; }
        let default = self.get_default(option);
        if !default.is_none() { return default; }

        None
    }
}

/// Contains functionality regarding conditions based on `CfgValue`s.
/// 
/// **INSERT MORE HERE**
pub mod conditions {
    use std::ops::{BitAnd, BitOr};

    /// Trait for the `check_that` function, that allows it to run a condition on a struct.
    pub trait Checkable {
        /// A method to check the enum for different conditions.
        /// This is preferred over the `is_x` methods, as it uses them
        /// with insignificant overhead. This method also includes 
        /// features presented by `Condition`, which allows for 
        /// chaining of conditions.
        fn check_that(&self, c: Condition) -> bool;
    }

    /// Different possible conditions.
    /// 
    /// Many conditions are self explanatory, such as `Is_Int` and `Exists`.
    /// Complex conditions can be created easily using the `|` and `&` operators.
    /// So, for example, if you want to check whether an enum is an integer, *or* a float,
    /// you can do the following:
    /// ``` 
    /// value.check_that(Is_Int | Is_Float)
    /// ```
    /// 
    /// If you'd rather use methods, the following is equivalent:
    /// ```
    /// value.check_that(Is_Int.or(Is_Float))
    /// ```
    /// 
    /// Both of the above examples expand to the following:
    /// ```
    /// value.check_that(OR(Box::new(Is_Int), Box::new(Is_Float)))
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
        /// For example:
        /// 
        /// ```
        /// Condition::Is_Int.execute(&CfgValue::Int(5))
        /// ```
        /// 
        /// will return `Condition::TRUE`.
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

                    if res1.is_true() && res2.is_true() {
                        TRUE
                    } else {FALSE}
                },
                Or(x,y) => {
                    let res1 = x.execute(input);
                    let res2 = y.execute(input);

                    if res1.is_true() || res2.is_true() {
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

        /// Specifies whether the result is true.
        pub fn is_true(&self) -> bool {
            if let Condition::TRUE = self {
                true
            } else { false }
        }

        /// Specifies whether the result is false.
        pub fn is_false(&self) -> bool {
            if let Condition::FALSE = self {
                true
            } else { false }
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
}