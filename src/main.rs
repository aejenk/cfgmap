mod cmap;
use ::cmap::{CfgMap, CfgValue::*};
use ::cmap::conditions::{Condition::*, PrimedCondition};

fn main() {
    let mut map = CfgMap::new();
    let mut submap = CfgMap::new();

    map.add("number".into(), Int(50));
    map.add("string".into(), Str("word".into()));
    submap.add("info".into(), Str("internal".into()));
    map.add("submap".into(), Map(submap));
    map.add_default("def_int".into(), Int(10));

    assert!(map.get_option("submap", "def_int").check_that(Is_Int));
    assert!(map.get_option("submap", "info").check_that(Is_Str));
    assert!(map.get("submap/info").check_that(Is_Str));
    assert!(map.get("submap").check_that(Is_Map));
    assert!(map.get("number").check_that(Is_Int));
    assert!(!map.get("string").check_that(Is_Int | Is_Float | Is_IRange));
    assert!(map.get_default("def_int").check_that(Is_Int));
    assert!(map.get("submap").check_that(Exists));

    let list = List(vec![Int(5), Int(8)]);
    let irange = IRange(6, 10);
    let frange = FRange(1.2, 3.4);
    let string = Str(String::new());

    assert!(list.check_that(Is_Listlike));
    assert!(irange.check_that(Is_Listlike));
    assert!(frange.check_that(Is_Listlike));
    assert!(!string.check_that(Is_Listlike));
}