#[crate_id = "json-utils"];
#[feature(macro_rules)];

extern mod extra;

use std::vec;
use extra::json::{Json, Boolean, List, Null, Number, Object, String};
use extra::treemap::TreeMap;

pub trait FromJson {
  fn take(js: Json) -> Option<Self>;
  fn peek<'a>(js: &'a Json) -> Option<&'a Self>;
  fn mut_peek<'a>(js: &'a mut Json) -> Option<&'a mut Self>;
}

#[deriving(Clone)]
pub struct JsonIterator<'a> {
  // todo make empty staticaly allocated ?
  priv empty: &'static[Json],
  priv iter: vec::VecIterator<'a, Json>,
}

impl<'a> Iterator<&'a Json> for JsonIterator<'a> {
  fn next(&mut self) -> Option<&'a Json> {
    self.next()
  }
}

pub trait JsonContainer {
  fn take<T:FromJson>(self) -> Option<T>;
  fn peek<'a, T:FromJson>(&'a self) -> Option<&'a T>;
  fn mut_peek<'a, T:FromJson>(&'a mut self) -> Option<&'a mut T>;
  fn array_iter<'a>(&'a self) -> JsonIterator<'a>;
}

impl JsonContainer for Json {
  fn take<T:FromJson>(self) -> Option<T> {
    FromJson::take(self)
  }

  fn peek<'a, T:FromJson>(&'a self) -> Option<&'a T> {
    FromJson::peek(self)
  }

  fn mut_peek<'a, T:FromJson>(&'a mut self) -> Option<&'a mut T> {
    FromJson::mut_peek(self)
  }

  fn array_iter<'a>(&'a self) -> JsonIterator<'a> {
    let empty = &'static[];
    let mut jsi = JsonIterator { empty: empty, iter: empty.iter() };
    match *self {
      List(ref values) => jsi.iter = values.iter(),
      _ => {}
    }
    jsi
  }
}

macro_rules! fjimpl(
  ($kind: ty, $variant: ident) => (

    impl FromJson for $kind {
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
  let js1 = Boolean(true);
  let js2 = Null;
  let a = js1.peek().unwrap();
  let b = js2.peek().unwrap();
  if *a {
    println("coucou");
  }
  if *b {
    println("coucou2");
  }
}

#[test]
fn test_f64() {
  let js1 = Number(0.1);
  let js2 = Null;
  let a : &f64 = js1.peek().unwrap();
  let b : &f64 = js2.peek().unwrap();
  if *a == 0.1 {
    println("coucou");
  }
  if *b == 0.1 {
    println("coucou2");
  }
}
