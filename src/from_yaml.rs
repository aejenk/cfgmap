use super::CfgMap;
use super::CfgValue;
use yaml_rust::Yaml as Value;
use yaml_rust::yaml::Hash;

fn yamlval_to_cfgval(value: Value) -> CfgValue {
    match value {
        Value::String(x) => CfgValue::Str(x),
        Value::Integer(x) => CfgValue::Int(x),
        Value::Real(x) => CfgValue::Float(x.parse().unwrap()),
        Value::Boolean(x) => CfgValue::Bool(x),
        Value::Array(x) => {
            CfgValue::List(x.into_iter().map(|v| yamlval_to_cfgval(v)).collect())
        },
        Value::Hash(x) => yamlmap_to_cfgval(x),
        Value::Null => CfgValue::Null,
        Value::BadValue => CfgValue::BadValue,
        Value::Alias(x) => CfgValue::Alias(x)
    }
}

fn yamlmap_to_cfgval(map: Hash) -> CfgValue {
    CfgValue::Map(CfgMap::with_hashmap(map.into_iter().map(|(k,v)| {
        (k.into_string().unwrap(), yamlval_to_cfgval(v))
    }).collect()))
}

/// Only works if the value is a json `Map`.
pub(crate) fn yaml_to_cfg(value: Value) -> CfgMap {
    if let CfgValue::Map(x) = yamlval_to_cfgval(value) {
        x
    } else {
        panic!("Yaml value passed wasn't a Hash.")
    }
}