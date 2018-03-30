use std::collections::HashMap;
use std::collections::hash_map::Entry;
use resp::RespValue;

pub struct List {
  vec: Vec<String>,
}

impl List {
    fn new() -> List {
      List {
        vec: Vec::new()
      }
    }

    fn lpush(&mut self, vals: Vec<String>) -> usize {
      for val in vals {
        self.vec.insert(0, val);
      }
      self.vec.len()
    }

    fn lpop(&mut self) -> Option<String> {
      if self.vec.len() == 0 {
        return None
      } else {
        return Some(self.vec.remove(0))
      }
    }

    fn llen(&self) -> usize {
      return self.vec.len()
    }

    fn lindex(&mut self, index: isize) -> Option<&String> {
      let mut id = index;
      if index < 0 {
        id = index + self.vec.len() as isize;
      }
      return self.vec.get(id as usize);
    }

    fn lrem(&mut self, count: isize, value: &str) -> usize {
      let mut removed_count = 0;      
      let mut remove_count = count;
      if remove_count < 0 {
        remove_count = -remove_count;
      }
      while remove_count > removed_count || count == 0 {
        if let Some(index) = self.vec.iter().position(|ref val| val.as_str() == value) {
          self.vec.remove(index);
          removed_count = removed_count + 1;
        } else {
          break;
        }
      }
      return removed_count as usize;
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
      if self.vec.len() == 0 {
        return None;
      }
      match List::fix_range(start, end, self.vec.len()) {
          Some((s, e)) => Some(&self.vec[s..e]),
          None => None,
      }
    }

    fn lset(&mut self, index: isize, value: String) -> Result<(), &'static str> {
      let mut idx = index;
      let len: isize = self.vec.len() as isize;
      if index < 0 {
        idx = index + self.vec.len() as isize + 1;
      }
      if len < idx {
        return Err("Index Out of Range");
      }
      self.vec[idx as usize] = value;
      return Ok(());
    }

    fn ltrim(&mut self, start: isize, end: isize) -> Option<()> {
      if self.vec.len() == 0 {
        return None;
      }
      match List::fix_range(start, end, self.vec.len()) {
        Some((s, e)) => {
          self.vec.drain(s..e);
          return Some(());
        },
        None => None,
      }
    }

    fn linsert(&mut self, gap: &str, pivot: &str, value: &str) -> Option<isize> {
      match self.vec.iter().position(|ref x| x.as_str() == pivot) {
        None => Some(-1),
        Some(index) => {
          match &gap as &str {
            "BEFORE" => {
              if index == 0 {
                self.vec.insert(0, value.to_string());
              } else {
                self.vec.insert(index - 1, value.to_string());
              }
            },
            _ => self.vec.insert(index + 1, value.to_string()),
          };
          Some(self.vec.len() as isize)
        }
      }
    }

    fn rpush(&mut self, vals: Vec<String>) -> Option<usize> {
      for value in vals {
        self.vec.push(value);
      }
      Some(self.vec.len())
    }

    fn rpop(&mut self) -> Option<String> {
      self.vec.pop()
    }
}