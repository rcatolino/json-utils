#[feature(macro_rules)];

extern mod jsonutils;
extern mod extra;

use jsonutils::TContainer;
use extra::json;
use extra::json::{Json, Boolean, List, Null, Number, Object, String};

#[test]
fn test_bool() {
  let js1 = Boolean(false);
  let js2 = Null;
  let a = js1.peek().unwrap();
  if *a {
    fail!();
  }
  if js2.peek::<bool>().is_some() {
    fail!();
  }
}

#[test]
fn test_f64() {
  let js1 = Number(0.1);
  let js2 = Boolean(true);
  let a : &f64 = js1.peek().unwrap();
  if *a != 0.1 {
    fail!();
  }
  if js2.peek::<f64>().is_some() {
    fail!();
  }
}

#[test]
fn test_str() {
  let js1 = String(~"json");
  let js2 = Boolean(true);
  let a = js1.peek::<~str>().unwrap();
  if *a != ~"json" {
    fail!();
  }
  if js2.peek::<~str>().is_some() {
    fail!();
  }
}

#[test]
fn test_null() {
  let js1 = Null;
  let js2 = Boolean(true);
  if !js1.empty() {
    fail!();
  }
  if js2.empty() {
    fail!();
  }
}

#[test]
fn test_array() {
  let js1 = json::from_str("[\"str\", \"ostr\", \"value\"]").unwrap();
  if js1.empty() {
    fail!();
  }

  if js1.peek::<List>().unwrap().len() == 0 {
    fail!();
  }

  for (value, expected) in js1.array_iter().zip((~["str", "ostr", "value"]).move_iter()) {
    if !value.peek::<~str>().unwrap().equiv(&expected) {
      fail!();
    }
  }
}

#[test]
fn test_move_array() {
  let js1 = json::from_str("[\"str\", \"ostr\", \"value\"]").unwrap();
  if js1.empty() {
    fail!();
  }

  if js1.peek::<List>().unwrap().len() == 0 {
    fail!();
  }

  for (value, expected) in js1.array_move_iter().zip((~["st", "ost", "valu"]).move_iter()) {
    let mut s = value.take::<~str>().unwrap();
    s.pop_char();
    if !s.equiv(&expected) {
      fail!();
    }
  }
}

#[test]
fn test_array_mut() {
  let mut js1 = json::from_str("[1, 2, 3]").unwrap();
  if js1.empty() {
    fail!();
  }

  if js1.peek::<List>().unwrap().len() == 0 {
    fail!();
  }

  for (value, expected) in js1.array_mut_iter().zip((~[2f64, 3f64, 4f64]).move_iter()) {
    let val = value.mut_peek::<f64>().unwrap();
    *val += 1f64;
    if *val != expected {
      fail!();
    }
  }
}

#[test]
fn test_property() {
  let js1 = json::from_str("{\"key\": \"value\"}").unwrap();
  if js1.empty() {
    fail!();
  }

  js1.peek::<~Object>().unwrap();
  if js1.property::<~str>(~"key").unwrap() != &~"value" {
    fail!();
  }
}

#[test]
fn test_move_property() {
  let mut js1 = json::from_str("{\"key\": \"value\"}").unwrap();
  if js1.empty() {
    fail!();
  }

  js1.peek::<~Object>().unwrap();
  let mut s = js1.pop_property::<~str>(~"key").unwrap();
  s.pop_char();
  if s != ~"valu" {
    fail!();
  }
}

#[test]
fn test_mut_property() {
  let mut js1 = json::from_str("{\"key\": 122}").unwrap();
  if js1.empty() {
    fail!();
  }

  js1.peek::<~Object>().unwrap();
  let f = js1.mut_property::<f64>(~"key").unwrap();
  *f += 4f64;
  if *f != 126f64 {
    fail!();
  }
}

#[test]
fn test_property_json() {
  let mut js1 = json::from_str("{\"key\": [1, 2, 3]}").unwrap();
  if js1.empty() {
    fail!();
  }

  let list = js1.pop_property::<List>(~"key").unwrap();
  for (value, expected) in list.iter().zip((~[1f64, 2f64, 3f64]).move_iter()) {
    if *value.peek::<f64>().unwrap() != expected {
      fail!();
    }
  }
}

#[test]
fn test_property_json2() {
  let mut js1 = json::from_str("{\"key\": [1, 2, 3]}").unwrap();
  if js1.empty() {
    fail!();
  }

  let list = js1.pop_property::<Json>(~"key").unwrap();
  for (value, expected) in list.array_iter().zip((~[1f64, 2f64, 3f64]).move_iter()) {
    if *value.peek::<f64>().unwrap() != expected {
      fail!();
    }
  }
}

macro_rules! unpack(
  ($json: ident, $struct_name: ident: $($field: ident)+) => (
    $struct_name { $($field: $json.pop_property(stringify!($field).to_owned()).unwrap(),)+ }
  )
)

#[allow(dead_code)]
struct JsTest {
  key: ~str,
  k2: bool,
  k3: Json,
}

#[test]
fn test_unpack_struct() {
  let mut js1 = json::from_str("{\"key\": \"strvalue\", \"k2\": true, \"k3\": {\"kk1\": 1}}").unwrap();

  let mut jstest = unpack!(js1, JsTest: key k2 k3);
  if jstest.key != ~"strvalue" || jstest.k2 != true ||
     jstest.k3.pop_property::<f64>(~"kk1").unwrap() != 1f64 {
    fail!();
  }
}

