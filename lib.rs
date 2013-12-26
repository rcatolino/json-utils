#[crate_id = "json-utils"];
#[feature(macro_rules)];

extern mod extra;
use extra::json;
use extra::json::{Json, Boolean, List, Null, Number, Object, String};
use extra::treemap::TreeMap;
use std::iter::Iterator;
use std::vec;

pub trait FromT<T> {
  fn take(t: T) -> Option<Self>;
  fn peek<'a>(t: &'a T) -> Option<&'a Self>;
  fn mut_peek<'a>(t: &'a mut T) -> Option<&'a mut Self>;
}

#[deriving(Clone)]
pub struct TIterator<'a, T> {
  iter: vec::VecIterator<'a, T>,
}

pub struct TMoveIterator<T> {
  iter: vec::MoveIterator<T>,
}

pub struct TMutIterator<'a, T> {
  iter: vec::VecMutIterator<'a, T>,
}

impl<'a, T> Iterator<&'a T> for TIterator<'a, T> {
  fn next(&mut self) -> Option<&'a T> {
    self.iter.next()
  }
}

impl<T> Iterator<T> for TMoveIterator<T> {
  fn next(&mut self) -> Option<T> {
    self.iter.next()
  }
}

impl<'a, T> Iterator<&'a mut T> for TMutIterator<'a, T> {
  fn next(&mut self) -> Option<&'a mut T> {
    self.iter.next()
  }
}

pub trait TContainer {
  fn take<U:FromT<Self>>(self) -> Option<U> {
    FromT::take(self)
  }
  fn peek<'a, U:FromT<Self>>(&'a self) -> Option<&'a U> {
    FromT::peek(self)
  }
  fn mut_peek<'a, U:FromT<Self>>(&'a mut self) -> Option<&'a mut U> {
    FromT::mut_peek(self)
  }
  fn pop_property<'a, U:FromT<Json>>(&'a mut self, prop: ~str) -> Option<U>;
  fn property<'a, U:FromT<Self>>(&'a self, prop: ~str) -> Option<&'a U>;
  fn mut_property<'a, U:FromT<Self>>(&'a mut self, prop: ~str) -> Option<&'a mut U>;
  fn array_move_iter(self) -> TMoveIterator<Self>;
  fn array_iter<'a>(&'a self) -> TIterator<'a, Self>;
  fn array_mut_iter<'a>(&'a mut self) -> TMutIterator<'a, Self>;
  fn empty(&self) -> bool;
}

impl TContainer for Json {
  fn pop_property<'a, U:FromT<Json>>(&'a mut self, prop: ~str) -> Option<U> {
    self.mut_peek().and_then(|map: &'a mut ~Object| map.pop(&prop)).
                    and_then(|p: Json| p.take())
  }

  fn property<'a, U:FromT<Json>>(&'a self, prop: ~str) -> Option<&'a U> {
    self.peek().and_then(|map: &'a ~Object| map.find(&prop)).
                and_then(|p: &'a Json| p.peek())
  }

  fn mut_property<'a, U:FromT<Json>>(&'a mut self, prop: ~str) -> Option<&'a mut U> {
    self.mut_peek().and_then(|map: &'a mut ~Object| map.find_mut(&prop)).
                    and_then(|p: &'a mut Json| p.mut_peek())
  }

  fn array_move_iter(self) -> TMoveIterator<Json> {
    TMoveIterator { iter: match self {
      List(values) => values.move_iter(),
      _ => (~[]).move_iter(),
    }}
  }

  fn array_iter<'a>(&'a self) -> TIterator<'a, Json> {
    TIterator { iter: match *self {
      List(ref values) => values.iter(),
      _ => (&'static[]).iter(),
    }}
  }

  fn array_mut_iter<'a>(&'a mut self) -> TMutIterator<'a, Json> {
    TMutIterator { iter: match *self {
      List(ref mut values) => values.mut_iter(),
      _ => (&'static mut[]).mut_iter(),
    }}
  }

  fn empty(&self) -> bool {
    match *self { Null => true, _ => false }
  }
}

impl FromT<Json> for Json {
  fn take(js: Json) -> Option<Json> {
    Some(js)
  }

  fn peek<'a>(js: &'a Json) -> Option<&'a Json> {
    Some(js)
  }

  fn mut_peek<'a>(js: &'a mut Json) -> Option<&'a mut Json> {
    Some(js)
  }
}

macro_rules! fjimpl(
  ($kind: ty, $variant: ident) => (

    impl FromT<Json> for $kind {
      fn take(js: Json) -> Option<$kind> {
        match js { $variant(v) => Some(v), _ => None }
      }

      fn peek<'a>(js: &'a Json) -> Option<&'a $kind> {
        match *js { $variant(ref v) => Some(v), _ => None }
      }

      fn mut_peek<'a>(js: &'a mut Json) -> Option<&'a mut $kind> {
        match *js { $variant(ref mut v) => Some(v), _ => None }
      }
    }
  )
)

fjimpl!(bool, Boolean)
fjimpl!(~str, String)
fjimpl!(f64, Number)
fjimpl!(~[Json], List)
fjimpl!(~TreeMap<~str, Json>, Object)

#[cfg(test)]

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
