use super::CfgMap;
use super::CfgValue;
use toml::{value::Value, value::Table};

fn tomlval_to_cfgval(value: Value) -> CfgValue {
    match value {
        Value::String(x) => CfgValue::Str(x),
        Value::Integer(x) => CfgValue::Int(x),
        Value::Float(x) => CfgValue::Float(x),
        Value::Boolean(x) => CfgValue::Bool(x),
        Value::Array(x) => {
            CfgValue::List(x.into_iter().map(|v| tomlval_to_cfgval(v)).collect())
        },
        Value::Table(x) => tomlmap_to_cfgval(x),
        Value::Datetime(x) => CfgValue::Datetime(x),
    }
}

fn tomlmap_to_cfgval(map: Table) -> CfgValue {
    CfgValue::Map(CfgMap::with_hashmap(map.into_iter().map(|(k,v)| {
        (k, tomlval_to_cfgval(v))
    }).collect()))
}

/// Only works if the value is a toml `Map`.
pub(crate) fn toml_to_cfg(value: Value) -> CfgMap {
    if let CfgValue::Map(x) = tomlval_to_cfgval(value) {
        x
    } else {
        panic!("Toml value passed wasn't a Table.")
    }
}