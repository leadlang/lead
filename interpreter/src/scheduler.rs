use std::thread::JoinHandle;

use crate::Application;

pub struct Scheduler {
  pub(crate) tasks: Vec<Task>,
}

impl Scheduler {
  pub fn new() -> Self {
    Self {
      tasks: Vec::with_capacity(10),
    }
  }
}

pub struct Task {}
