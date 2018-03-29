use std::collections::HashMap;
use std::collections::hash_map::Entry;
use resp::RespValue;

pub struct List {
  hash: HashMap<String, Vec<String>>
}

impl List {
    fn new() -> List {
      List {
        hash: HashMap::new()
      }
    }

    fn lpush(&mut self, key: String, vals: Vec<String>) -> usize {
      let list = self.hash.entry(key).or_insert(Vec::new());
      for val in vals {
        list.push(val);
      }
      list.len()
    }

    fn lpop(&mut self, key: String) -> Option<String> {
      match self.hash.entry(key) {
        Entry::Occupied(l) => {
          let list = l.into_mut();
          if list.len() == 0 {
            return None
          } else {
            return Some(list.remove(0))
          }
        },
        Entry::Vacant(_) => None,
      }
    }

    fn llen(& self, key: String) -> usize {
      match self.hash.get(&key) {
        Some(list) => list.len(),
        None => 0,
      }
    }

    fn lindex(&mut self, key: String, index: isize) -> Option<&String> {
      match self.hash.get(&key) {
        Some(list) => {
          let mut id = index;
          if index < 0 {
            id = index + list.len() as isize;
          }
          return list.get(index as usize);
        },
        None => None,
      }
    }

    //TODO
    // fn lrem(&mut self, key: String, count: isize, value: String) -> usize {
    //   match self.hash.entry(key) {
    //       Entry::Occupied(l) => {
    //         let mut number = 0;
    //         let list = l.into_mut();
    //         if count < 0 {
    //           list.reverse();
    //         }

    //         list
    //         .filter(|val| val == value)
    //         .for_each(|(_, index)| )
    //         return number;
    //       },
    //       Entry::Vacant(_) => 0,
    //   }
    // }

    // fn lrange(&mut self, key: String, start: isize, end: isize) -> String {

    // }

    // fn lset(&mut self, key: String, index: usize, value: String) -> String {

    // }

    // fn ltrim(&mut self, key: String, start: isize, end: isize) -> String {

    // }

    // fn linsert(&mut self, key: String, gap: String, pivot: String, value: String) -> isize {

    // }

    // fn rpush(&mut self, key: String, value: String) -> usize {

    // }
}