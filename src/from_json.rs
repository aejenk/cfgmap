use super::CfgMap;
use super::CfgValue;
use serde_json::{Value, Map};

fn jsonval_to_cfgval(value: Value) -> CfgValue {
    match value {
        Value::Null => CfgValue::Null,
        Value::Bool(x) => CfgValue::Bool(x),
        Value::Number(x) => {
            if x.is_i64() | x.is_u64() {
                CfgValue::Int(x.as_i64().unwrap())
            } else {
                CfgValue::Float(x.as_f64().unwrap())
            }
        },
        Value::String(x) => CfgValue::Str(x),
        Value::Array(x) => {
            CfgValue::List(x.into_iter().map(|v| jsonval_to_cfgval(v)).collect())
        },
        Value::Object(x) => jsonmap_to_cfgval(x)
    }
}

fn jsonmap_to_cfgval(map: Map<String, Value>) -> CfgValue {
    CfgValue::Map(CfgMap::with_hashmap(map.into_iter().map(|(k,v)| {
        (k, jsonval_to_cfgval(v))
    }).collect()))
}

/// Only works if the value is a json `Map`.
pub(crate) fn json_to_cfg(value: Value) -> CfgMap {
    if let CfgValue::Map(x) = jsonval_to_cfgval(value) {
        x
    } else {
        panic!("Json value passed wasn't Object.")
    }
}