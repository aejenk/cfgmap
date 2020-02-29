//! This crate contains a new data structure that acts as a wrapper around a `HashMap`.
//! It provides its own data enum for values `(CfgValue)`, and contains multiple helper functions
//! that let you navigate the hashmap easily.
//! 
//! Its primary purpose is for configuration, allowing for validation as well. In essence, a `CfgMap`
//! would represent a configuration for an application. So far, alternatives for configuration would be 
//! to use a data format library directly, or utilise a struct that a 
//! configuration file, like JSON or TOML, would serialise into.
//! 
//! This can be more than satisfactory, especially for basic configurations, however in certain situations
//! it can prove to be more than a bit cumbersome. For example, if you plan on using default options in the case
//! that certain options aren't set, having multiple nested objects to validate and go through, etc.
//! 
//! It is very easy to make a new `CfgMap`, there are multiple methods:
//! 
//! ```
//! use cfgmap::CfgMap;
//! 
//! let map1 = CfgMap::new();
//! let mut map2 = CfgMap::new();
//! map2.default = "default".into();
//! ```
//! 
//! `CfgMap` allows for some functionality with regards to default values. For `map1` above, `default` was never set, so 
//! the values would be retrieved from the root. For `map2` however, it's assumed that all default values are located in
//! `default`.
//! 
//! You can also create a `CfgMap` using different methods:
//! 
//! - `with_hashmap(HashMap<String, CfgValue>)`: Useful for when you want to craft a hashmap manually and convert it into 
//!     a `CfgMap`.
//! - `from_toml(toml::value::Value)`: Generates a `CfgMap` representation of the `toml` value passed. This value has to be
//!     a `Table`, otherwise the function will panic.
//! - `from_json(json::Value)`: Same as `from_toml`, but for `json`.
//! 
//! The last two methods are optional. In order to use them, you would need to add the `from_toml` and `from_json` features.
//! 
//! `CfgMap` also comes with support for a certain `path` syntax with its keys:
//! 
//! ```
//! # use cfgmap::CfgMap;
//! # let cfgmap = CfgMap::new();
//! cfgmap.get("hello/there/pal");
//! ```
//! 
//! This helps to make access to nested items easy. The line above is essentially equal to:
//! 
//! ```
//! # use cfgmap::CfgMap;
//! # let map = CfgMap::new();
//! map.get("hello")
//!     .and_then(|a| a.as_map())
//!     .and_then(|a| a.get("there"))
//!     .and_then(|a| a.as_map())
//!     .and_then(|a| a.get("pal"));
//! ```
//! 
//! Note that if `hello` or `there` weren't `CfgMap`s as well, the whole expression would evaluate to `None`.
//! This key can also contain array indexes. For example, with `a/0/c`, it will check whether `a` is a `Map` or 
//! a `List`. If its the former, it will try to find a key with the value `0`. If its the latter, it will instead
//! try to index into the list.
//! 
//! Now, what if you want to check what a certain value evaluates to? This is something that you'll encounter 
//! very quickly if you'd like to use any value. This crate comes with an extensive support for `Conditions`!
//! 
//! ```
//! # use cfgmap::CfgMap;
//! use cfgmap::{Condition::*, Checkable};
//! # let cfgmap = CfgMap::new();
//! let is_number = cfgmap.get("hello/there/pal").check_that(IsInt | IsFloat);
//! ```
//! 
//! The above line will check whether the value at `hello/there/pal` is a `CfgValue::Int` or a `CfgValue::Float`.
//! There are more conditions listed [*here*](./enum.Condition.html). If there are more conditions that you'd like added,
//! feel free to open up an issue or open a PR! All of these serve as utilities to help validate a certain value.
//! 
//! Defaults can also be used quite easily:+
//! 
//! ```
//! # use cfgmap::CfgMap;
//! # let map = CfgMap::new();
//! map.get_option("http_settings", "ip_address");
//! ```
//! 
//! Let's say that `map` was initialised with its default at `default`. The above line will be equivalent to the following:
//! 
//! ```
//! # use cfgmap::CfgMap;
//! # let map = CfgMap::new();
//! map.get("http_settings/ip_address").or(map.get("default/ip_address"));
//! ```
//! 
//! You can also update an option like this, using `update_option`. This works similar to using `add`, except that it doesn't 
//! add a new option if it isn't found, only updating an existing one.
//! 
//! All `HashMap` methods are also available, since `CfgMap` implements `Deref` and `DerefMut` for `HashMap<String, CfgValue>`.
//! For example, you can call `.iter()` on it, even though that is not directly implemented.
//! 
//! ## Example:
//! ```
//!let toml = toml::toml! {
//!    [package]
//!    name = "cfgmap"
//!    version = "0.1.0"
//!    authors = ["Andrea Jenkins <mctech26@gmail.com>"]
//!
//!    [lib]
//!    name = "cfgmap"
//!    path = "src/cfgmap.rs"
//!
//!    [dependencies]
//!    serde_json = { version = "1.0.48", optional = true }
//!    toml = { version = "0.5.6", optional = true }
//!
//!    [other]
//!    date = 2020-02-29
//!    float = 1.2
//!    int = 3
//!    internal.more = "hello"
//!
//!    [[person]]
//!    name = "a"
//!
//!    [[person]]
//!    name = "b"
//!};
//!
//!let cmap = CfgMap::from_toml(toml);
//!
//!assert!(cmap.get("package/name").check_that(IsExactlyStr("cfgmap".into())));
//!assert!(cmap.get("package/version").check_that(IsExactlyStr("0.1.0".into())));
//!assert!(cmap.get("package/authors").check_that(IsExactlyList(vec![Str("Andrea Jenkins <mctech26@gmail.com>".into())])));
//!
//!assert!(cmap.get("lib/name").check_that(IsExactlyStr("cfgmap".into())));
//!assert!(cmap.get("lib/path").check_that(IsExactlyStr("src/cfgmap.rs".into())));
//!
//!assert!(cmap.get("dependencies/serde_json/version").check_that(IsExactlyStr("1.0.48".into())));
//!assert!(cmap.get("dependencies/serde_json/optional").check_that(IsTrue));
//!assert!(cmap.get("dependencies/toml/version").check_that(IsExactlyStr("0.5.6".into())));
//!assert!(cmap.get("dependencies/toml/optional").check_that(IsTrue));
//!
//!assert!(cmap.get("other/date").check_that(IsDatetime));
//!assert!(cmap.get("other/float").check_that(IsExactlyFloat(1.2)));
//!assert!(cmap.get("other/int").check_that(IsExactlyInt(3)));
//!assert!(cmap.get("other/internal/more").check_that(IsExactlyStr("hello".into())));
//!
//!assert!(cmap.get("person").check_that(IsListWith(Box::new(IsMap))));
//!assert!(cmap.get("person/0/name").check_that(IsExactlyStr("a".into())));
//!assert!(cmap.get("person/1/name").check_that(IsExactlyStr("b".into())));
//! ```

use std::collections::HashMap;
mod conditions;
pub use conditions::{Checkable, Condition};
use std::concat;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;

#[cfg(feature = "from_json")]
use serde_json::Value as JsonValue;

#[cfg(feature = "from_json")]
use toml::value::Value as TomlValue;

#[cfg(feature = "from_json")]
mod from_json;

#[cfg(feature = "from_toml")]
mod from_toml;

#[cfg(feature = "from_toml")]
use toml::value::Datetime;

// The type contained within `CfgValue::Int`
pub(crate) type _Int = i64;

// The type contained within `CfgValue::Float`
pub(crate) type _Float = f64;

// The type contained within `CfgValue::Str`
pub(crate) type _Str = String;

/// The type contained within `CfgValue::Bool`
pub(crate) type _Bool = bool;

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

/// Represents a value within a `CfgMap`
/// 
/// **EXTRA STUFF HERE**
#[derive(Debug, Clone, PartialEq)]
pub enum CfgValue {
    /// Represents an integer value.
    Int(_Int),

    /// Represents a float value.
    Float(_Float),

    /// Represents a string.
    Str(_Str),

    /// Represents a bool.
    Bool(_Bool),

    /// Represents a nested configuration map.
    Map(CfgMap),

    /// Represents a list of values. These values can have differing types.
    List(Vec<CfgValue>),

    
    /// Represents a `Datetime`. Only available if using `from_toml`.
    #[cfg(feature = "from_toml")]
    Datetime(Datetime),

    /// Represents a null value. Only available if using `from_json`.
    #[cfg(feature = "from_json")]
    Null,
}

impl CfgValue {
    /// Returns the contents of the enum converted into an integer, if possible.
    /// 
    /// If the enum represents a float, it will be converted into an integer.
    pub fn to_int(&self) -> Option<_Int> {
        if let CfgValue::Int(x) = self {
            Some(*x)
        } else if let CfgValue::Float(x) = self {
            Some(*x as _Int)
        } else { None }
    }

    /// Returns the contents of the enum converted into a float, if possible.
    /// 
    /// If the enum represents an integer, it will be converted into a float.
    pub fn to_float(&self) -> Option<_Float> {
        if let CfgValue::Float(x) = self {
            Some(*x)
        } else if let CfgValue::Int(x) = self {
            Some(*x as _Float)
        } else { None }
    }

    is_type!(is_int, CfgValue::Int);
    is_type!(is_float, CfgValue::Float);
    is_type!(is_str, CfgValue::Str);
    is_type!(is_bool, CfgValue::Bool);
    is_type!(is_map, CfgValue::Map);
    is_type!(is_list, CfgValue::List);

    #[cfg(feature = "from_json")]
    is_type!(is_null [0], CfgValue::Null);

    #[cfg(feature = "from_toml")]
    is_type!(is_datetime, CfgValue::Datetime);

    as_type!(as_int, _Int, CfgValue::Int);
    as_type!(as_float, _Float, CfgValue::Float);
    as_type!(as_str, _Str, CfgValue::Str);
    as_type!(as_bool, _Bool, CfgValue::Bool);
    as_type!(as_map, CfgMap, CfgValue::Map);
    as_type!(as_list, Vec<CfgValue>, CfgValue::List);

    as_mut_type!(as_int_mut, _Int, CfgValue::Int);
    as_mut_type!(as_float_mut, _Float, CfgValue::Float);
    as_mut_type!(as_str_mut, _Str, CfgValue::Str);
    as_mut_type!(as_bool_mut, _Bool, CfgValue::Bool);
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
    if in_string.find(pat).is_none() {
        return (in_string.into(), None);
    }

    let mut splitter = in_string.splitn(2, pat);
    let first = splitter.next().unwrap().to_string();
    let second = splitter.next().unwrap().to_string();

    (first, Some(second))
}

fn rsplit_once(in_string: &str, pat: char) -> (Option<String>, String) {
    if in_string.find(pat).is_none() {
        return (None, in_string.into());
    }

    let mut splitter = in_string.rsplitn(2, pat);
    let first = splitter.next().unwrap().to_string();
    let second = splitter.next().unwrap().to_string();

    (Some(second), first)
}

impl Deref for CfgMap {
    type Target = HashMap<String, CfgValue>;

    fn deref(&self) -> &Self::Target {
        &self.internal_map
    }
}

impl DerefMut for CfgMap {
    fn deref_mut (&mut self) -> &mut Self::Target {
        &mut self.internal_map
    }
}

impl From<Option<CfgValue>> for CfgValue {
    fn from(opt: Option<CfgValue>) -> Self {
        opt.unwrap_or(CfgValue::Null)
    }
}


/// A configuration map, containing helper functions and effectively being a wrapper
/// around a `HashMap`s.
/// 
/// **TODO: FILL THIS IN**
#[derive(Debug, Clone, PartialEq)]
pub struct CfgMap {
    /// An internal map representing the configuration.
    internal_map: HashMap<String, CfgValue>,

    /// A path to the default subobject.
    pub default: String
}

impl CfgMap {

    /// Creates a new empty CfgMap.
    pub fn new() -> CfgMap {
        CfgMap { internal_map: HashMap::new(), default: String::new() }
    }

    /// Initialises a `CfgMap` using the `map` that's passed in.
    pub fn with_hashmap(map: HashMap<String, CfgValue>) -> CfgMap {
        CfgMap { internal_map: map, default: String::new() }
    }

    #[cfg(feature = "from_json")]
    /// Initialises a `CfgMap` from a json `Value`.
    pub fn from_json(value: JsonValue) -> CfgMap {
        from_json::json_to_cfg(value)
    }

    #[cfg(feature = "from_toml")]
    /// Initialises a `CfgMap` from a toml `Value`.
    pub fn from_toml(value: TomlValue) -> CfgMap {
        from_toml::toml_to_cfg(value)
    }

    /// Adds a new entry in the configuration.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// get the inner submap `a/b/...y/`, and add `z` onto it. This is for convenience sake,
    /// as doing this manually can prove to be verbose.
    /// 
    /// This key can also index into lists. So, for example `a/0/b` would try checking if `"a"`
    /// is a list, and index into it. Otherwise it will try to find an internal map with the key `0`.
    /// 
    /// In order to add a default value to a normal submap - you would need to do this manually,
    /// as this function will always use `get_mut`.
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use cfgmap::{CfgMap, CfgValue::*};
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
            Ok(self.internal_map.insert(key.to_string(), value))
        }
        else {
            let subtree = self.get_mut(&path.unwrap());

            if subtree.check_that(Condition::IsMap) {
                subtree.unwrap().as_map_mut().unwrap().add(&key, value)
            }
            else {
                Err(())
            }
        }
    }

    /// Gets a reference to a value from within the configuration.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// go through the inner submaps `"a/b/..."` until a submap isn't found, or the end is reached.
    /// This is for convenience sake, as doing this manually can prove to be verbose.
    /// 
    /// This key can also index into lists. So, for example `a/0/b` would try checking if `"a"`
    /// is a list, and index into it. Otherwise it will try to find an internal map with the key `0`.
    /// 
    /// Returns `None` if the key doesn't exist.
    /// 
    /// ## Examples
    /// ```
    /// use cfgmap::{CfgMap, CfgValue::*, Condition::*, Checkable};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    /// 
    /// submap.add("key", Int(5));
    ///
    /// cmap.add("sub", Map(submap));
    /// 
    /// assert!(cmap.get("sub").check_that(IsMap));
    /// assert!(cmap.get("sub/key").check_that(IsExactlyInt(5)));
    /// ```
    pub fn get(&self, key: &str) -> Option<&CfgValue> {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.internal_map.get(key)
        }
        else {
            let next = self.internal_map.get(&h);

            if let Some(CfgValue::Map(map)) = next {
                map.get(&t.unwrap())
            } else if let Some(CfgValue::List(list)) = next {
                // Get the next segment of the path, and parse as a list index.
                let (index,new_t) = split_once(&t.unwrap(), '/');
                let index = index.parse::<usize>();

                // If it's an invalid usize, then the whole path is invalid.
                if index.is_err() {
                    None
                }
                else if new_t.is_none() {
                    list.get(index.unwrap())
                } else {
                    list.get(index.unwrap()).and_then(|op| {
                        op.as_map()
                    }).and_then(|map| {
                        map.get(&new_t.unwrap())
                    })
                }
            } else {
                None
            }
        }
    }

    /// Gets a mutable reference to a value from within the configuration.
    /// 
    /// Returns `None` if the key doesn't exist.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// go through the inner submaps `"a/b/..."` until a submap isn't found, or the end is reached.
    /// This is for convenience sake, as doing this manually can prove to be verbose.
    /// 
    /// This key can also index into lists. So, for example `a/0/b` would try checking if `"a"`
    /// is a list, and index into it. Otherwise it will try to find an internal map with the key `0`.
    /// 
    /// ## Examples
    /// ```
    /// use cfgmap::{CfgMap, CfgValue::*, Condition::*, Checkable};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    ///
    /// cmap.add("sub", Map(submap));
    /// 
    /// let mut submap = cmap.get_mut("sub");
    /// assert!(submap.check_that(IsMap));
    /// 
    /// submap.unwrap().as_map_mut().unwrap().add("key", Int(5));
    /// assert!(cmap.get_mut("sub/key").check_that(IsExactlyInt(5)));
    /// ```
    pub fn get_mut(&mut self, key: &str) -> Option<&mut CfgValue> {
        let (h, t) = split_once(key, '/');

        if t.is_none() {
            self.internal_map.get_mut(key)
        }
        else {
            let next = self.internal_map.get_mut(&h);

            if let Some(CfgValue::Map(map)) = next {
                map.get_mut(&t.unwrap())
            } else if let Some(CfgValue::List(list)) = next {
                // Get the next segment of the path, and parse as a list index.
                let (index,new_t) = split_once(&t.unwrap(), '/');
                let index = index.parse::<usize>();

                // If it's an invalid usize, then the whole path is invalid.
                if index.is_err() {
                    None
                }
                else if new_t.is_none() {
                    list.get_mut(index.unwrap())
                } else {
                    list.get_mut(index.unwrap()).and_then(|op| {
                        op.as_map_mut()
                    }).and_then(|map| {
                        map.get_mut(&new_t.unwrap())
                    })
                }
            } else {
                None
            }
        }
    }

    /// Checks whether a certain path exists.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// go through the inner submaps `"a/b/..."` until a submap isn't found, or the end is reached.
    /// This is for convenience sake, as doing this manually can prove to be verbose.
    /// 
    /// This key can also index into lists. So, for example `a/0/b` would try checking if `"a"`
    /// is a list, and index into it. Otherwise it will try to find an internal map with the key `0`.
    /// 
    /// ## Examples
    /// ```
    /// use cfgmap::{CfgMap, CfgValue::*, Condition::*, Checkable};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    ///
    /// cmap.add("num", Int(10));
    /// submap.add("num", Int(20));
    /// cmap.add("sub", Map(submap));
    /// 
    /// assert!(cmap.contains_key("num"));
    /// assert!(cmap.contains_key("sub/num"));
    /// ```
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Gets a reference to an option within the configuration.
    /// 
    /// It first tries to get 
    /// `category/option` within the normal values. If this doesn't exist, it will then 
    /// try to retrieve `option` from the default path instead (`self.default/option`).
    /// 
    /// Note that if `default` wasn't set on construction, this function will instead retrieve
    /// the value from the root directory (`option`) directly.
    /// 
    /// Returns `None` if the key doesn't exist in either map.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// go through the inner submaps `"a/b/..."` until a submap isn't found, or the end is reached.
    /// This is for convenience sake, as doing this manually can prove to be verbose.
    /// 
    /// This key can also index into lists. So, for example `a/0/b` would try checking if `"a"`
    /// is a list, and index into it. Otherwise it will try to find an internal map with the key `0`.
    /// 
    /// ## Examples
    /// ```
    /// use cfgmap::{CfgMap, CfgValue::*, Checkable, Condition::*};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    /// 
    /// submap.add("OP1", Int(5));
    /// cmap.add("OP1", Int(8));
    /// 
    /// cmap.add("sub", Map(submap));
    /// 
    /// assert!(cmap.get_option("sub", "OP1").check_that(IsExactlyInt(5)));
    /// assert!(cmap.get_option("sub", "OP1").check_that(IsExactlyInt(5)));
    /// assert!(cmap.get_option("sub", "OP2").is_none());
    /// ```
    pub fn get_option(&self, category: &str, option: &str) -> Option<&CfgValue> {
        let fullkey = format!("{}/{}", category, option);
        let default = format!("{}{}", self.default, option);
        self.get(&fullkey).or(self.get(&default))
    }

    /// Updates the option with the new value `to`.
    /// 
    /// It first tries to get 
    /// `category/option` within the normal values. If this doesn't exist, it will then 
    /// try to retrieve `option` from the default path instead (`self.default/option`).
    /// 
    /// Note that if `default` wasn't set on construction, this function will instead retrieve
    /// the value from the root directory (`option`) directly.
    /// 
    /// The `key` can be of the form of the path `"a/b/...y/z/"`, in which case it will
    /// go through the inner submaps `"a/b/..."` until a submap isn't found, or the end is reached.
    /// This is for convenience sake, as doing this manually can prove to be verbose.
    /// 
    /// This key can also index into lists. So, for example `a/0/b` would try checking if `"a"`
    /// is a list, and index into it. Otherwise it will try to find an internal map with the key `0`.
    /// 
    /// ## Examples
    /// ```
    /// use cfgmap::{CfgMap, CfgValue::*, Checkable, Condition::*};
    /// 
    /// let mut cmap = CfgMap::new();
    /// let mut submap = CfgMap::new();
    /// 
    /// submap.add("OP1", Int(5));
    /// cmap.add("OP1", Int(8));
    /// 
    /// cmap.add("sub", Map(submap));
    /// 
    /// let OL1 = cmap.update_option("sub", "OP1", Int(10));
    /// let OL2 = cmap.update_option("foo", "OP1", Int(16));
    /// let OL3 = cmap.update_option("sub", "OP2", Int(99));
    /// 
    /// assert!(cmap.get_option("sub", "OP1").check_that(IsExactlyInt(10)));
    /// assert!(cmap.get_option("foo", "OP1").check_that(IsExactlyInt(16)));
    /// assert!(cmap.get_option("sub", "OP2").is_none());
    /// 
    /// assert_eq!(OL1, Some(Int(5)));
    /// assert_eq!(OL2, Some(Int(8)));
    /// assert_eq!(OL3, None);
    /// ```
    pub fn update_option(&mut self, category: &str, option: &str, to: CfgValue) -> Option<CfgValue> {
        let fullkey = format!("{}/{}", category, option);
        let default = format!("{}{}", self.default, option);

        if let Some(x) = self.get_mut(&fullkey) {
            Some(mem::replace(x, to))
        } else if let Some(x) = self.get_mut(&default) {
            Some(mem::replace(x, to))
        } else {
            None
        }
    }
}

#[cfg(feature = "from_json")]
impl From<JsonValue> for CfgMap {
    fn from(opt: JsonValue) -> Self {
        CfgMap::from_json(opt)
    }
}

#[cfg(feature = "from_toml")]
impl From<TomlValue> for CfgMap {
    fn from(opt: TomlValue) -> Self {
        CfgMap::from_toml(opt)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "from_json")]
    use serde_json;

    #[cfg(feature = "from_toml")]
    use toml;

    use crate::{CfgMap, CfgValue::*, Condition::*, Checkable};

    #[test]
    #[cfg(feature = "from_json")]
    fn from_json_test() {
        let json = serde_json::json! ({
            "string": "string",
            "integer": 12,
            "float": 1.2,
            "null": null,
            "sub": {
                "integer": 20
            },
            "array": [10,20],
        });

        let cmap = CfgMap::from_json(json);

        assert!(cmap.get("string").check_that(IsExactlyStr("string".into())));
        assert!(cmap.get("integer").check_that(IsExactlyInt(12)));
        assert!(cmap.get("float").check_that(IsExactlyFloat(1.2)));
        assert!(cmap.get("null").check_that(IsNull));
        assert!(cmap.get("sub/integer").check_that(IsExactlyInt(20)));
        assert!(cmap.get("array").check_that(IsListWith(Box::new(IsInt)) & IsListWithLength(2)));
    }

    #[test]
    #[cfg(feature = "from_toml")]
    fn from_toml_test() {
        let toml = toml::toml! {
            [package]
            name = "cfgmap"
            version = "0.1.0"
            authors = ["Andrea Jenkins <mctech26@gmail.com>"]

            [lib]
            name = "cfgmap"
            path = "src/cfgmap.rs"

            [dependencies]
            serde_json = { version = "1.0.48", optional = true }
            toml = { version = "0.5.6", optional = true }

            [other]
            date = 2020-02-29
            float = 1.2
            int = 3
            internal.more = "hello"

            [[person]]
            name = "a"

            [[person]]
            name = "b"
        };

        let cmap = CfgMap::from_toml(toml);

        assert!(cmap.get("package/name").check_that(IsExactlyStr("cfgmap".into())));
        assert!(cmap.get("package/version").check_that(IsExactlyStr("0.1.0".into())));
        assert!(cmap.get("package/authors").check_that(IsExactlyList(vec![Str("Andrea Jenkins <mctech26@gmail.com>".into())])));

        assert!(cmap.get("lib/name").check_that(IsExactlyStr("cfgmap".into())));
        assert!(cmap.get("lib/path").check_that(IsExactlyStr("src/cfgmap.rs".into())));

        assert!(cmap.get("dependencies/serde_json/version").check_that(IsExactlyStr("1.0.48".into())));
        assert!(cmap.get("dependencies/serde_json/optional").check_that(IsTrue));
        assert!(cmap.get("dependencies/toml/version").check_that(IsExactlyStr("0.5.6".into())));
        assert!(cmap.get("dependencies/toml/optional").check_that(IsTrue));

        assert!(cmap.get("other/date").check_that(IsDatetime));
        assert!(cmap.get("other/float").check_that(IsExactlyFloat(1.2)));
        assert!(cmap.get("other/int").check_that(IsExactlyInt(3)));
        assert!(cmap.get("other/internal/more").check_that(IsExactlyStr("hello".into())));

        assert!(cmap.get("person").check_that(IsListWith(Box::new(IsMap))));
        assert!(cmap.get("person/0/name").check_that(IsExactlyStr("a".into())));
        assert!(cmap.get("person/1/name").check_that(IsExactlyStr("b".into())));

    }
}