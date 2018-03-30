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
        list.insert(0, val);
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
          return list.get(id as usize);
        },
        None => None,
      }
    }

    fn lrem(&mut self, key: String, count: isize, value: &str) -> usize {
      match self.hash.entry(key) {
          Entry::Occupied(l) => {
            let list = l.into_mut();
            let mut remove_count = count;
            if remove_count < 0 {
              remove_count = -remove_count;
            }
            let mut removed_count = 0;
            while remove_count > removed_count || count == 0 {
              if let Some(index) = list.iter().position(|ref val| val.as_str() == value) {
                list.remove(index);
                removed_count = removed_count + 1;
              } else {
                break;
              }
            }
            return removed_count as usize;
          },
          Entry::Vacant(_) => 0,
      }
    }

    fn fix_range(start: isize, end: isize, len: usize) -> Option<(usize, usize)> {
      let mut e = end;      
      let mut s = start;
      if start < 0 {
        e = (len as isize) + start + 1;
      }
      if end < 0 {
        s = (len as isize) + end;
      }
      if s > e || e > (len as isize) {
        return None;
      }

      Some((s as usize, e as usize))
    }

    fn lrange(&self, key: String, start: isize, end: isize) -> Option<&[String]> {
      match self.hash.get(&key) {
          Some(list) => {
            let len = list.len();
            if len == 0 {
              return None;
            }
            match List::fix_range(start, end, len) {
                Some((s, e)) => Some(&list[s..e]),
                None => None,
            }
          },
          None => None,
      }
    }

    fn lset(&mut self, key: String, index: isize, value: String) -> Result<(), &'static str> {
      match self.hash.get_mut(&key) {
          Some(list) => {
            let mut idx = index;
            let len: isize = list.len() as isize;
            if index < 0 {
              idx = index + list.len() as isize + 1;
            }
            if len < idx {
              return Err("Index Out of Range");
            }
            list[idx as usize] = value;
            return Ok(());
          },
          None => Err("Not Found"),
      }
    }

    fn ltrim(&mut self, key: String, start: isize, end: isize) -> Option<()> {
      match self.hash.get_mut(&key) {
          None => None,        
          Some(list) => {
            let len = list.len();
            if len == 0 {
              return None;
            }
            match List::fix_range(start, end, len) {
                Some((s, e)) => {
                  list.drain(s..e);
                  return Some(());
                },
                None => None,
            }
          },
      }
    }

    fn linsert(&mut self, key: String, gap: &str, pivot: &str, value: &str) -> Option<isize> {
      match self.hash.get_mut(&key) {
        None => None,
        Some(list) => {
          match list.iter().position(|ref x| x.as_str() == pivot) {
            None => Some(-1),
            Some(index) => {
              match &gap as &str {
                "BEFORE" => {
                  if index == 0 {
                    list.insert(0, value.to_string());
                  } else {
                    list.insert(index - 1, value.to_string());
                  }
                },
                _ => list.insert(index + 1, value.to_string()),
              };
              Some(list.len() as isize)
            }
          }
        }
      }
    }

    fn rpush(&mut self, key: String, vals: Vec<String>) -> Option<usize> {
      let list = self.hash.entry(key).or_insert(Vec::new());
      for value in vals {
        list.push(value);
      }
      Some(list.len())
    }

    fn rpushx(&mut self, key: String, vals: Vec<String>) -> Option<usize> {
      if self.hash.contains_key(&key) {
        return self.rpush(key, vals);
      }
      return None;
    }

    fn rpop(&mut self, key: String) -> Option<String> {
      match self.hash.get_mut(&key) {
        None => None,
        Some(list) => list.pop(),
      }
    }

    fn rpoplpush(&mut self, source: String, dest: String) -> Result<(), &'static str> {
      if !self.hash.contains_key(&source) || !self.hash.contains_key(&dest) {
        return Err("Invalid List");
      }
      match self.lpop(source) {
          None => Err("Invalid Source List"),        
          Some(item) => {
            self.lpush(dest, vec![item]);
            Ok(())
          },
      }
    }
}