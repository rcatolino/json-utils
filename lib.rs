#[crate_id = "jsonutils"];
#[feature(macro_rules)];

extern mod extra;
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

