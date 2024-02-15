#![doc = include_str!("../readme.md")]

#[derive(Debug, Clone, PartialEq)]
pub struct IndexTree {
    index: Vec<usize>,
    dimen: Vec<usize>,
    skips: Vec<usize>,
    length: usize,
    next: bool,
}

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
        }
    }

    /// Fix the current index to the tree structure.
    /// Returns if good index and if completed the current subtree
    fn fix(&mut self) -> Result<bool, ()> {
        self.next = false;
        let mut b = false;
        for i in 1..self.length {
            if self.index[self.length - i] >= self.dimen[self.length - i] {
                b = true;
                self.index[self.length - i - 1] = self.index[self.length - i - 1]
                    + self.index[self.length - i] / self.dimen[self.length - i];
                self.index[self.length - i] =
                    self.index[self.length - i] % self.dimen[self.length - i];
            }
        }
        if self.index[0] >= self.dimen[0] {
            return Err(());
        }
        Ok(b)
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
        if self.index[0] >= self.dimen[0] {
            return false;
        }
        true
    }

    /// Increase index for a specific skip (of the initial vector)
    /// Sanitize skip if not consistent.
    pub fn inc_skip_v(&mut self, skip: usize) -> Result<bool, ()> {
        if (skip >= self.skips.len()) | (skip == 0) {
            self.index[self.length - 1] = self.index[self.length - 1] + 1;
        } else {
            self.index[self.skips[skip - 1] - 2] = self.index[self.skips[skip - 1] - 2] + 1;
            for i in (self.skips[skip - 1] - 1)..self.length {
                self.index[i] = 0;
            }
        }
        self.fix()
    }

    /// Increase index for a specific skip (of the initial vector)
    /// Sanitize skip if not consistent.
    pub fn inc_skip(&mut self, skip: usize) -> Result<bool, ()> {
        if (skip > self.length + 1) | (skip < 2) {
            self.index[self.length - 1] = self.index[self.length - 1] + 1;
        } else {
            self.index[skip - 2] = self.index[skip - 2] + 1;
            for i in (skip - 1)..self.length {
                self.index[i] = 0;
            }
        }
        self.fix()
    }

    /// Increase index in lowest subtree
    pub fn inc(&mut self) -> Result<bool, ()> {
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
