#[crate_id = "json-utils"];
#[feature(macro_rules)];

extern mod extra;

use std::vec;
use extra::json::{Json, Boolean, List, Null, Number, Object, String};
use extra::treemap::TreeMap;

pub trait FromT<T> {
  fn take(t: T) -> Option<Self>;
  fn peek<'a>(t: &'a T) -> Option<&'a Self>;
  fn mut_peek<'a>(t: &'a mut T) -> Option<&'a mut Self>;
}

#[deriving(Clone)]
pub struct TIterator<'a, T> {
  // todo make empty staticaly allocated ?
  priv empty: &'static[T],
  priv iter: vec::VecIterator<'a, T>,
}

impl<'a> Iterator<&'a Json> for TIterator<'a, Json> {
  fn next(&mut self) -> Option<&'a Json> {
    self.iter.next()
  }
}

pub trait TContainer<T> {
  fn take<U:FromT<T>>(self) -> Option<U>;
  fn peek<'a, U:FromT<T>>(&'a self) -> Option<&'a U>;
  fn mut_peek<'a, U:FromT<T>>(&'a mut self) -> Option<&'a mut U>;
  fn array_iter<'a>(&'a self) -> TIterator<'a, T>;
  fn empty(&self) -> bool;
}

impl TContainer<Json> for Json {
  fn take<U:FromT<Json>>(self) -> Option<U> {
    FromT::take(self)
  }

  fn peek<'a, U:FromT<Json>>(&'a self) -> Option<&'a U> {
    FromT::peek(self)
  }

  fn mut_peek<'a, U:FromT<Json>>(&'a mut self) -> Option<&'a mut U> {
    FromT::mut_peek(self)
  }

  fn array_iter<'a>(&'a self) -> TIterator<'a, Json> {
    let empty = &'static[];
    let mut ti = TIterator { empty: empty, iter: empty.iter() };
    match *self {
      List(ref values) => ti.iter = values.iter(),
      _ => {}
    }
    ti
  }

  fn empty(&self) -> bool {
    match *self { Null => true, _ => false }
  }
}

macro_rules! fjimpl(
  ($kind: ty, $variant: ident) => (

    impl FromT<Json> for $kind {
      fn take(js: Json) -> Option<$kind> {
        match js {
          $variant(v) => Some(v),
          _ => None,
        }
      }

      fn peek<'a>(js: &'a Json) -> Option<&'a $kind> {
        match *js {
          $variant(ref v) => Some(v),
          _ => None,
        }
      }

      fn mut_peek<'a>(js: &'a mut Json) -> Option<&'a mut $kind> {
        match *js {
          $variant(ref mut v) => Some(v),
          _ => None,
        }
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
