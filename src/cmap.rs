use std::collections::HashMap;

macro_rules! as_type {
    ($name:ident, $type:ty, $enum_type:path) => {
        pub fn $name(&self) -> Option<&$type> {
            if let $enum_type(x) = self {
                Some(x)
            } else { None }
        }
    };
}

/// Commonalities with "AS"
/// 
/// pub fn as_NAME(&self) -> Option<&TYPE> {
///     if let ENUM_TYPE(x) = self {
///         Some(x)
///     } else { None }
/// }

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

    /// An empty value.
    Empty
}

impl CfgValue {
    /// Checks whether the enum is Empty.
    fn is_empty(&self) -> bool {
        if let CfgValue::Empty = self {
            true
        } else { false }
    }

    /// Checks whether the enum is an Int.
    fn is_int(&self) -> bool {
        if let CfgValue::Int(_) = self {
            true
        } else { false }
    }


    /// Returns a reference to the int within the enum.
    /// Returns `None` if the enum isn't `CfgValue::Int`.
    // pub fn as_int(&self) -> Option<&isize> {
    //     if let CfgValue::Int(x) = self {
    //         Some(x)
    //     } else { None }
    // }

    as_type!(as_int, isize, CfgValue::Int);

    /// Returns the contents of the enum converted into an integer, if possible.
    /// If the enum represents a float, it will be converted into an integer.
    pub fn to_int(&self) -> Option<isize> {
        if let CfgValue::Int(x) = self {
            Some(*x)
        } else if let CfgValue::Float(x) = self {
            Some(*x as isize)
        } else { None }
    }

    /// Checks whether the enum is a Float.
    fn is_float(&self) -> bool {
        if let CfgValue::Float(_) = self {
            true
        } else { false }
    }

    /// Returns a reference to the float within the enum.
    /// Returns `None` if the enum isn't `CfgValue::Float`.
    pub fn as_float(&self) -> Option<&f64> {
        if let CfgValue::Float(x) = self {
            Some(x)
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

    /// Checks whether the enum is a String.
    fn is_str(&self) -> bool {
        if let CfgValue::Str(_) = self {
            true
        } else { false }
    }

    /// Returns a reference to the internal string.
    /// Returns `None` if the enum isn't `CfgValue::Str`.
    pub fn as_str(&self) -> Option<&String> {
        if let CfgValue::Str(x) = self {
            Some(x)
        } else { None }
    }

    /// Checks whether the enum is an IRange.
    fn is_irange(&self) -> bool {
        if let CfgValue::IRange(_,_) = self {
            true
        } else { false }
    }

    /// Checks whether the enum is an FRange.
    fn is_frange(&self) -> bool {
        if let CfgValue::FRange(_,_) = self {
            true
        } else { false }
    }

    /// Checks whether the enum is a Map.
    fn is_map(&self) -> bool {
        if let CfgValue::Map(_) = self {
            true
        } else { false }
    }

    /// Converts the enum into a `CfgMap`, if possible.
    pub fn as_map(&self) -> Option<CfgMap> {
        if let CfgValue::Map(x) = self {
            Some(x.clone())
        } else { None }
    }

    /// Checks whether the enum is a List.
    fn is_list(&self) -> bool {
        if let CfgValue::List(_) = self {
            true
        } else { false }
    }

    /// Returns a reference to the list. Result is `None` if contents aren't a list.
    pub fn as_list(&self) -> Option<&Vec<CfgValue>> {
        if let CfgValue::List(x) = self {
            Some(x)
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

    /// A method to check the enum for different conditions.
    /// This is preferred over the `is_x` methods, as it uses them
    /// with insignificant overhead. This method also includes 
    /// features presented by `Condition`, which allows for 
    /// chaining of conditions.
    pub fn check_that(&self, c: conditions::Condition) -> bool {
        return conditions::PrimedCondition {input: self}.that(c);
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

/// check
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
            let subtree = self.get(&path.unwrap());
            if subtree.check_that(conditions::Condition::Is_Map) {
                
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
    pub fn get(&self, key: &str) -> &CfgValue {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.internal_map.get(key).unwrap_or(&CfgValue::Empty)
        }
        else {
            if let Some(CfgValue::Map(map)) = self.internal_map.get(&h) {
                map.get(&t.unwrap())
            }
            else{
                &CfgValue::Empty
            }
        }
    }

    /// Retrieves a default value from within the configuration.
    /// Same as `get`, but for *default* values.
    pub fn get_default(&self, key: &str) -> &CfgValue {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.defaults.get(key).unwrap_or(&CfgValue::Empty)
        }
        else {
            if let Some(CfgValue::Map(map)) = self.defaults.get(&h) {
                map.get(&t.unwrap())
            }
            else{
                &CfgValue::Empty
            }
        }
    }

    /// Retrieves an option value from within the configuration.
    /// Similar to using `get("category/option")`, except that if not found it 
    /// attempts to use the default value instead. In terms of priority, it returns:
    /// 
    /// - `get(category/option)`
    /// - `get_default(option)`
    pub fn get_option(&self, category: &str, option: &str) -> &CfgValue {
        let opt = self.get(&format!("{}/{}", category, option));
        if !opt.is_empty() { return opt; }
        let default = self.get_default(option);
        if !default.is_empty() { return default; }

        &CfgValue::Empty
    }
}

/// Contains functionality regarding conditions based on `CfgValue`s.
/// Primarily used for the `check_that` functions in CfgMap.
pub mod conditions {
    use std::ops::{BitAnd, BitOr};

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
        Exists,
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
        fn execute(&self, input: &super::CfgValue) -> Condition {
            use Condition::*;

            match self {
                Is_Int => Condition::from_bool(input.is_int()),
                Is_Float => Condition::from_bool(input.is_float()),
                Is_Str => Condition::from_bool(input.is_str()),
                Is_IRange => Condition::from_bool(input.is_irange()),
                Is_FRange => Condition::from_bool(input.is_frange()),
                Is_List => Condition::from_bool(input.is_list()),
                Is_Map => Condition::from_bool(input.is_map()),
                Exists => Condition::from_bool(!input.is_empty()),
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
        fn is_true(&self) -> bool {
            if let Condition::TRUE = self {
                true
            } else { false }
        }

        /// Specifies whether the result is false.
        fn is_false(&self) -> bool {
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

    /// A wrapper around an input for a condition.
    /// Used in order to execute a condition on a specific input.
    /// 
    /// This is abstracted by using the `check_that` method of a given enum.
    /// So, for example:
    /// 
    /// ```
    /// CfgValue::Int(5).check_that(Is_Int)
    /// ```
    /// 
    /// is the same as doing:
    /// 
    /// ```
    /// PrimedCondition {input: &CfgValue::Int(5)}.that(Is_Int)
    /// ```
    pub struct PrimedCondition<'a> {
        /// Represents the input that the condition will be executed on.
        pub input: &'a super::CfgValue
    }

    impl<'a> PrimedCondition<'a> {
        /// Executes the condition passed on the input that this wrapper was primed with.
        /// If the input is `CfgValue::Empty`, it avoids executing the condition and always returns `false`.
        pub fn that(&self, condition: Condition) -> bool {
            if self.input.is_empty() {
                return false;
            }

            let result = condition.execute(&self.input);
            result.is_true()
        }
    }
}

// #[cfg(test)]
// mod tests {

//     #[test]
    
// }