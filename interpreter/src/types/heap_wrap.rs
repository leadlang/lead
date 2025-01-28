use crate::Application;

use super::{Args, BufValue, Heap};

pub struct HeapWrapper<'a> {
  pub(crate) heap: &'a mut Heap,
  pub(crate) args: &'a Args,
  pub(crate) pkg_name: &'a str,
  pub(crate) app: *mut Application<'a>,
}

impl<'a> HeapWrapper<'a> {
  pub fn upgrade(self) -> &'a mut Heap {
    let app = unsafe { &mut *self.app };

    app.log_info.call_mut((self.pkg_name,));

    self.heap
  }

  pub fn get(&self, key: &str) -> Option<&BufValue> {
    if self.args.contains(&(key as *const str)) {
      return self.heap.get(key);
    }

    None
  }

  pub fn get_mut(&mut self, key: &str) -> Option<&mut BufValue> {
    if self.args.contains(&(key as *const str)) {
      return self.heap.get_mut(key);
    }

    None
  }

  pub fn remove(&mut self, key: &str) -> Option<Option<BufValue>> {
    if self.args.contains(&(key as *const str)) {
      return self.heap.remove(key);
    }

    None
  }
}
