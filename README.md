# cfgmap

This crate contains a new data structure that acts as a wrapper around a `HashMap`.
It provides its own data enum for values `(CfgValue)`, and contains multiple helper functions
that let you navigate the hashmap easily.

Its primary purpose is for configuration, allowing for validation as well. In essence, a `CfgMap`
would represent a configuration for an application. So far, alternatives for configuration would be
to use a data format library directly, or utilise a struct that a
configuration file, like JSON or TOML, would serialise into.

This can be more than satisfactory, especially for basic configurations, however in certain situations
it can prove to be more than a bit cumbersome. For example, if you plan on using default options in the case
that certain options aren't set, having multiple nested objects to validate and go through, etc.

It is very easy to make a new `CfgMap`, there are multiple methods:

```rust
use cfgmap::CfgMap;

let map1 = CfgMap::new();
let mut map2 = CfgMap::new();
map2.default = "default".into();
```

`CfgMap` allows for some functionality with regards to default values. For `map1` above, `default` was never set, so
the values would be retrieved from the root. For `map2` however, it's assumed that all default values are located in
`default`.

You can also create a `CfgMap` using different methods:

- `with_hashmap(HashMap<String, CfgValue>)`: Useful for when you want to craft a hashmap manually and convert it into
    a `CfgMap`.
- `from_toml(toml::value::Value)`: Generates a `CfgMap` representation of the `toml` value passed. This value has to be
    a `Table`, otherwise the function will panic.
- `from_json(json::Value)`: Same as `from_toml`, but for `json`.

The last two methods are optional. In order to use them, you would need to add the `from_toml` and `from_json` features.

`CfgMap` also comes with support for a certain `path` syntax with its keys:

```rust
cfgmap.get("hello/there/pal");
```

This helps to make access to nested items easy. The line above is essentially equal to:

```rust
map.get("hello")
    .and_then(|a| a.as_map())
    .and_then(|a| a.get("there"))
    .and_then(|a| a.as_map())
    .and_then(|a| a.get("pal"));
```

Note that if `hello` or `there` weren't `CfgMap`s as well, the whole expression would evaluate to `None`.
This key can also contain array indexes. For example, with `a/0/c`, it will check whether `a` is a `Map` or
a `List`. If its the former, it will try to find a key with the value `0`. If its the latter, it will instead
try to index into the list.

Now, what if you want to check what a certain value evaluates to? This is something that you'll encounter
very quickly if you'd like to use any value. This crate comes with an extensive support for `Conditions`!

```rust
use cfgmap::{Condition::*, Checkable};
let is_number = cfgmap.get("hello/there/pal").check_that(IsInt | IsFloat);
```

The above line will check whether the value at `hello/there/pal` is a `CfgValue::Int` or a `CfgValue::Float`.
There are more conditions listed [*here*](./enum.Condition.html). If there are more conditions that you'd like added,
feel free to open up an issue or open a PR! All of these serve as utilities to help validate a certain value.

Defaults can also be used quite easily:+

```rust
map.get_option("http_settings", "ip_address");
```

Let's say that `map` was initialised with its default at `default`. The above line will be equivalent to the following:

```rust
map.get("http_settings/ip_address").or(map.get("default/ip_address"));
```

You can also update an option like this, using `update_option`. This works similar to using `add`, except that it doesn't
add a new option if it isn't found, only updating an existing one.

All `HashMap` methods are also available, since `CfgMap` implements `Deref` and `DerefMut` for `HashMap<String, CfgValue>`.
For example, you can call `.iter()` on it, even though that is not directly implemented.

### Example:
```rust
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
```

License: MIT
