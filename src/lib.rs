#![doc = include_str!("../readme.md")]

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct IndexTree {
  index: Vec<usize>,
  dimen: Vec<usize>,
  skips: Vec<usize>,
  length: usize,
  next: bool,
  good: bool,
}

type Response = Result<(usize, bool), ()>;

impl IndexTree {
  /// Generate an IndexTree (plus sanitize the skips indexes)
  pub fn new(dimen: &Vec<usize>, skips: &Vec<usize>) -> IndexTree {
    let dimension_v = dimen
      .to_vec()
      .iter()
      .map(|x| *x)
      .filter(|x| x > &0)
      .collect::<Vec<_>>()
      .to_vec();

    let dim_len = dimension_v.len();
    let mut skips_v = skips.to_vec();
    skips_v.sort();
    skips_v.dedup();
    let skips_v: Vec<usize> = skips_v
      .iter()
      .map(|x| *x)
      .filter(|x| (x < &(dim_len + 2)) & (x > &1))
      .collect::<Vec<_>>()
      .to_vec();

    IndexTree {
      index: vec![0; dim_len.into()],
      dimen: dimension_v,
      skips: skips_v,
      length: dim_len.into(),
      next: false,
      good: true,
    }
  }

  /// Fix the current index to the tree structure.
  /// Returns if good index and if completed the current subtree
  fn fix(&mut self, zero: usize) -> Response {
    self.next = false;
    let mut b = false;
    let mut p = 1;
    let len = self.length;
    let zero = len + 1 - zero;

    for i in 1..zero + 1 {
      self.index[self.length - i] = 0;
    }

    let mut i = zero + 1;

    while (self.index[len - i] >= self.dimen[len - i]) & (i < len) {
      b = true;
      p = len - i;

      self.index[p - 1] = self.index[p - 1] + 1;
      self.index[p] = self.index[p] - self.dimen[p];
      i += 1;
    }

    if self.index[0] >= self.dimen[0] {
      self.good = false;
      return Err(());
    }
    Ok((p - 1, b))
  }

  // /// TODO: Debug function to fix to a specific index
  // fn index(&mut self, index: Vec<usize>) {
  //     if index.len() == self.length {
  //         self.index = index;
  //     } else {
  //         self.index = vec![0; self.length];
  //     }
  // }

  /// Check if index is bad
  pub fn check(&self) -> bool {
    self.good
  }

  /// Increase index for a specific skip (of the initial vector)
  /// Sanitize skip if not consistent.
  pub fn inc_skip_v(&mut self, skip: usize) -> Response {
    if (skip >= self.skips.len()) | (skip == 0) {
      self.inc_skip(self.length + 1)
    } else {
      self.inc_skip(self.skips[skip - 1])
    }
  }

  /// Increase index for a specific skip (of the initial vector)
  /// Sanitize skip if not consistent.
  pub fn inc_skip(&mut self, skip: usize) -> Response {
    if (skip > self.length + 1) | (skip < 2) {
      self.index[self.length - 1] = self.index[self.length - 1] + 1;
      self.fix(self.length + 1)
    } else {
      self.index[skip - 2] = self.index[skip - 2] + 1;
      self.fix(skip)
    }
  }

  /// Increase index in lowest subtree
  pub fn inc(&mut self) -> Response {
    self.next = false;
    self.inc_skip(0)
  }

  /// Get the current index
  pub fn get(&mut self) -> &Vec<usize> {
    &self.index
  }

  /// Return the iterator dimensions
  pub fn dimensions(&self) -> &Vec<usize> {
    &self.dimen
  }
}

impl Iterator for IndexTree {
  type Item = Vec<usize>;
  fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    if self.next {
      let _ = self.inc();
    };
    self.next = true;
    let i = self.get().to_vec();
    if self.check() {
      Some(i)
    } else {
      None
    }
  }
}

impl Display for IndexTree {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(
      f,
      "{:?} {:?} {:?} {}",
      self.index, self.dimen, self.skips, self.length
    )
  }
}
