use std::collections::HashMap;
mod conditions;
pub use conditions::{Checkable, Condition};

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
#[derive(Debug, Clone, PartialEq)]
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

    as_mut_type!(as_int_mut, isize, CfgValue::Int);
    as_mut_type!(as_float_mut, f64, CfgValue::Float);
    as_mut_type!(as_str_mut, str, CfgValue::Str);
    as_mut_type!(as_map_mut, CfgMap, CfgValue::Map);
    as_mut_type!(as_list_mut, Vec<CfgValue>, CfgValue::List);
}

impl conditions::Checkable for CfgValue {
    fn check_that(&self, c: conditions::Condition) -> bool {
        return c.execute(self).to_bool();
    }
}

impl conditions::Checkable for Option<&CfgValue> {
    fn check_that(&self, condition: conditions::Condition) -> bool {
        self.as_ref().map_or(false, |val| val.check_that(condition))
    }
}

impl conditions::Checkable for Option<&mut CfgValue> {
    fn check_that(&self, condition: conditions::Condition) -> bool {
        self.as_ref().map_or(false, |val| val.check_that(condition))
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
/// **TODO: FILL THIS IN**
#[derive(Debug, Clone, PartialEq)]
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
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// get the inner submap `a/b/...y/`, and add `z` onto it. This is for convenience sake,
    /// as doing this manually can prove to be verbose.
    /// 
    /// In order to add a default value to a normal submap - you would need to do this manually,
    /// as this function will always use `get_mut`.
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use cmap::{CfgMap, CfgValue::*};
    /// 
    /// let mut cmap = CfgMap::new();
    /// 
    /// // Works - a root add like this will always work.
    /// assert!(cmap.add("k1", Int(5)).is_ok());
    /// 
    /// // Doesn't work, because k1 isn't a map.
    /// assert!(cmap.add("k1/k2", Int(10)).is_err());
    /// 
    /// // Works - returns the old value.
    /// let r = cmap.add("k1", Float(8.0));
    /// assert_eq!(Ok(Some(Int(5))), r);
    /// ```
    /// 
    /// ## Return values
    /// 
    /// - `Err` if the path as specified by `key` isn't found. In the case above for example, `get_mut("a")` returns a `None`.
    /// - `Ok(Some(CfgValue))` if the path as specified by key already contained a value, and was overwritten. In this case, the old value is returned.
    /// - `Ok(None)` otherwise.
    pub fn add(&mut self, key: &str, value: CfgValue) -> Result<Option<CfgValue>, ()> {
        let (path, key) = rsplit_once(key, '/');

        if path.is_none(){
            // Return successful result, with possible overwritten key.
            Ok(self.internal_map.insert(key.to_string(), value))
        }
        else {
            let subtree = self.get_mut(&path.unwrap());
            if subtree.check_that(Condition::Is_Map) {
                subtree.unwrap().as_map_mut().unwrap().add(&key, value)
            }
            else {
                Err(())
            }
        }
    }

    /// Adds a new entry in the default configuration.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// get the inner submap `a/b/...y/`, and add `z` onto it. This is for convenience sake,
    /// as doing this manually can prove to be verbose.
    /// 
    /// In order to add a normal value to a default submap - you would need to do this manually, as 
    /// this function will always use `get_default_mut`.
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use cmap::{CfgMap, CfgValue::*};
    /// 
    /// let mut cmap = CfgMap::new();
    /// 
    /// // Works - a root add like this will always work.
    /// assert!(cmap.add_default("k1", Int(5)).is_ok());
    /// 
    /// // Doesn't work, because k1 isn't a map.
    /// assert!(cmap.add_default("k1/k2", Int(10)).is_err());
    /// 
    /// // Works - returns the old value.
    /// let r = cmap.add_default("k1", Float(8.0));
    /// assert_eq!(Ok(Some(Int(5))), r);
    /// ```
    /// 
    /// ## Return values
    /// 
    /// - `Err` if the path as specified by `key` isn't found. In the case above for example, `get_default_mut("a")` returns a `None`.
    /// - `Ok(Some(CfgValue))` if the path as specified by key already contained a value, and was overwritten. In this case, the old value is returned.
    /// - `Ok(None)` otherwise.
    pub fn add_default(&mut self, key: &str, value: CfgValue) -> Result<Option<CfgValue>, ()> {
        let (path, key) = rsplit_once(key, '/');

        if path.is_none(){
            Ok(self.defaults.insert(key.to_string(), value))
        }
        else {
            let subtree = self.get_default_mut(&path.unwrap());
            if subtree.check_that(Condition::Is_Map) {
                subtree.unwrap().as_map_mut().unwrap().add_default(&key, value)
            }
            else {
                Err(())
            }
        }
    }

    /// Gets a reference to a value from within the configuration.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// get the inner submap `a/b/...y/`, and add `z` onto it. This is for convenience sake,
    /// as doing this manually can prove to be verbose.
    /// 
    /// Returns `None` if the key doesn't exist.
    /// 
    /// ## Examples
    /// ```
    /// use cmap::{CfgMap, CfgValue::*, Condition::*, Checkable};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    /// 
    /// submap.add("key", Int(5));
    ///
    /// cmap.add("sub", Map(submap));
    /// 
    /// assert!(cmap.get("sub").check_that(Is_Map));
    /// assert!(cmap.get("sub/key").check_that(Is_Int));
    /// ```
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

    /// Gets a mutable reference to a value from within the configuration.
    /// 
    /// Returns `None` if the key doesn't exist.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// get the inner submap `a/b/...y/`, and add `z` onto it. This is for convenience sake,
    /// as doing this manually can prove to be verbose.
    /// 
    /// ## Examples
    /// ```
    /// use cmap::{CfgMap, CfgValue::*, Condition::*, Checkable};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    ///
    /// cmap.add("sub", Map(submap));
    /// 
    /// let mut submap = cmap.get_mut("sub");
    /// assert!(submap.check_that(Is_Map));
    /// 
    /// submap.unwrap().as_map_mut().unwrap().add("key", Int(5));
    /// assert!(cmap.get_mut("sub/key").check_that(Is_Int));
    /// ```
    pub fn get_mut(&mut self, key: &str) -> Option<&mut CfgValue> {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.internal_map.get_mut(key)
        }
        else {
            self.internal_map.get_mut(&h).and_then(|op| {
                op.as_map_mut()
            }).and_then(|map| {
                map.get_mut(&t.unwrap())
            })
        }
    }

    /// Gets a reference to a default value from within the configuration.
    /// 
    /// Returns `None` if the key doesn't exist within the default values.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// get the inner submap `a/b/...y/`, and add `z` onto it. This is for convenience sake,
    /// as doing this manually can prove to be verbose.
    /// 
    /// ## Examples
    /// ```
    /// use cmap::{CfgMap, CfgValue::*, Condition::*, Checkable};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    /// 
    /// submap.add_default("key", Int(5));
    ///
    /// cmap.add_default("sub", Map(submap));
    /// 
    /// assert!(cmap.get_default("sub").check_that(Is_Map));
    /// assert!(cmap.get_default("sub/key").check_that(Is_Int));
    /// ```
    pub fn get_default(&self, key: &str) -> Option<&CfgValue> {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.defaults.get(key)
        }
        else {
            self.defaults.get(&h).and_then(|op| {
                op.as_map()
            }).and_then(|map| {
                map.get_default(&t.unwrap())
            })
        }
    }

    /// Gets a mutable reference to a default value from within the configuration.
    /// 
    /// Returns `None` if the key doesn't exist within the default values.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// get the inner submap `a/b/...y/`, and add `z` onto it. This is for convenience sake,
    /// as doing this manually can prove to be verbose.
    /// 
    /// ## Examples
    /// ```
    /// use cmap::{CfgMap, CfgValue::*, Condition::*, Checkable};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    ///
    /// cmap.add_default("sub", Map(submap));
    /// 
    /// let mut submap = cmap.get_default_mut("sub");
    /// assert!(submap.check_that(Is_Map));
    /// 
    /// submap.unwrap().as_map_mut().unwrap().add_default("key", Int(5));
    /// assert!(cmap.get_default_mut("sub/key").check_that(Is_Int));
    /// ```
    pub fn get_default_mut(&mut self, key: &str) -> Option<&mut CfgValue> {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.defaults.get_mut(key)
        }
        else {
            self.defaults.get_mut(&h).and_then(|op| {
                op.as_map_mut()
            }).and_then(|map| {
                map.get_default_mut(&t.unwrap())
            })
        }
    }

    /// Gets a reference to an option within the configuration. It first tries to get 
    /// `category/option` within the normal values. If this doesn't exist, it will then 
    /// try to retrieve `option` from the default values.
    /// 
    /// Returns `None` if the key doesn't exist in either map.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// get the inner submap `a/b/...y/`, and add `z` onto it. This is for convenience sake,
    /// as doing this manually can prove to be verbose.
    /// 
    /// ## Examples
    /// ```
    /// use cmap::{CfgMap, CfgValue::*, Condition::*, Checkable};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    /// 
    /// submap.add("OP1", Int(5));
    /// cmap.add_default("OP1", Int(8));
    /// cmap.add_default("OP2", Int(10));
    /// 
    /// cmap.add("sub", Map(submap));
    /// 
    /// assert_eq!(cmap.get_option("sub", "OP1"), Some(&Int(5)));
    /// assert_eq!(cmap.get_option("foo", "OP1"), Some(&Int(8)));
    /// assert_eq!(cmap.get_option("sub", "OP2"), Some(&Int(10)));
    /// assert_eq!(cmap.get_option("sub", "OP3"), None);
    /// ```
    pub fn get_option(&self, category: &str, option: &str) -> Option<&CfgValue> {
        self.get(&format!("{}/{}", category, option)).or_else(|| self.get_default(option))
    }
}

#[cfg(test)]
pub mod test {
    use crate::{CfgMap, CfgValue, Condition};

    #[test]
    fn testing_out() {
        let cmap = CfgMap::new();
    }

}