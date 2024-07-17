# IndexTree

*tl;dr:* multi-index Iterator for a structured finite tree (isomorphic to Cartesian product)
  with pre-defined skips.

`IndexTree` is an iterator that allows the traverse of a tree with a structure similar to the
  Cartesian product of a finite amount of index-list, i.e. each level of the tree represents the choice
  of a new index leaving the leaves to represent all the possible combinations of indices.

Motivation and a concrete application example can be
  found in my [blog post](https://charlietrip.neocities.org/blog/240131-rust-aessolver).


## Iterator Intuition

For example, consider the index list `[3,2]` which identifies two lists of three and two elements
  respectively.
  This can be represented as the Cartesian product of the indices `{0,1,2} x {0,1}`, we get the tree:

```{verbatim}
            |- 0 : => [0,0]
      |- 0 -|
      |     |- 1 : => [0,1]
      |
      |     |- 0 : => [1,0]
Root -|- 1 -|
      |     |- 1 : => [1,1]
      |
      |     |- 0 : => [2,0]
      |- 2 -|
            |- 1 : => [2,1]
```

The iterator allows for both standard and skipping traverses of the tree.
  The standard traverse starts from `[0,0]` and returns the next lexicographical index in the tree.

```{verbatim}
            |- 0 : => [0,0]   
      |- 0 -|                 |
      |     |- 1 : => [0,1]   | 
      |                       |
      |     |- 0 : => [1,0]   | standard
Root -|- 1 -|                 |
      |     |- 1 : => [1,1]   | traverse
      |                       |
      |     |- 0 : => [2,0]   |
      |- 2 -|                 V
            |- 1 : => [2,1]   
```

The skipping traverse allows skipping the whole subtree level concerning the current index position.
  In the diagram, we are skipping the `2`nd level and call this a `2`-skip.

```{verbatim}
            |- 0 : => [0,0] -->| skipping 
      |- 0 -|                  | (1st level)
      |     |- 1 : => [0,1] ->/  traverse
      |                      /
      |                     L
      |     |- 0 : => [1,0] -->| skipping 
Root -|- 1 -|                  | current
      |     |- 1 : => [1,1] ->/  subtree 
      |                      / 
      |                     L  
      |     |- 0 : => [2,0] 
      |- 2 -|               
            |- 1 : => [2,1] 
```



## Iterator Design

`IndexTree` is instantiated from an index-list vector `dimen: &Vec<usize>` and an index-skips vector
  `skips: &Vec<usize>` representing all the pre-defined skips which are sanitized to be contained 
  between `2` and `dimen.len()+1`.

Internally, `IndexTree` stores the current index as `index: Vec<usize>` which can be moved for
  standard or skipping traverse.
The variables `length` and `next: bool` are used for quick checks of dimension and allowing
the usage of `IndexTree` as an `Iterator`.

Moving the index via `inc()`,`inc_skip(n)` or `inc_skip_v(n)` returns a `Result<(usize,bool),()>`
	representing with `Ok((_,false))` if the increase remained in the same subtree, `Ok((j,true))` if
	the current index moved to the next subtree (at level `j`) or `Err()` if the current index moved
	outside the tree, a.k.a. incorrect index or terminated iterator.
  Using the previous example, `[0,0] => inc() -> Ok((0,false)) => [0,1] => inc() -> Ok((1,true)) => [1,0]`.
  while `[0,0] => inc_skip(2) -> Ok((0,false)) => [1,0]`.

Beware that skipping traverses `inc_skip(n)`, `inc_skip_v(n)` will return `Ok(true)` only if
  the index moves to a different subtree on a higher level.
  For example, for index-list `[3,2,2]`,
  `[0,0,0] => inc_skip(3) -> Ok((0,false)) => [0,1,0] => inc_skip(3) -> Ok((2,true)) => [1,0,0]`.



## Standard Traversing Examples

For standard traversing, the iterator can be created *as is* via, e.g., `into_iter()`
(standard ownership and mutability rules apply):

```
use indextree::IndexTree;

let dims: Vec<usize> = vec![2, 3, 2];
let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
let it: IndexTree = IndexTree::new(&dims, &skips);
// Observe: it.skips == vec![2, 3, 4]

for item in it.into_iter() {
    // Code and
    // item : Vec<usize> == vec![i0,i1,i2]
}
```

If the code requires more fine-graded movement, one can traverse manually via `inc()` and `get()`:

```
use indextree::IndexTree;

let dims: Vec<usize> = vec![2, 3, 2];
let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
let mut it: IndexTree = IndexTree::new(&dims, &skips);

type Response = Result<(usize, bool), ()>;

// To increase by one element
let res : Response = it.inc();
let item : &Vec<usize> = it.get(); 
```




## Skipping Traversing Example

For skipping traversing, the iterator can be created *as is* via, e.g., `into_iter()` and later
  traversed by calling `next()` accordingly.
  To skip, one calls `inc_skip(n)` to skip the `n`-th level or `inc_skip_v(n)` to skip the `n`-th
  index stored in `IndexTree`'s internal `skips: &Vec<usize>`.

```
use indextree::IndexTree;

let dims: Vec<usize> = vec![2, 3, 2];
let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
let mut it: IndexTree = IndexTree::new(&dims, &skips).into_iter();

while let Some(item) = it.next() {
    // Code and
    // item : Vec<usize> == vec![i0,i1,i2]
    
    let n : usize = 0;
    
    // Skip using inc_skip_v(n) where n is index to skip
    it.inc_skip_v(n);
    
    // Skip using inc_skip_v(n) where n is the index on the instantiated skips vector
    it.inc_skip_v(n);
}
```


**AAA**: currently, skipping traversing cannot be done using a `for` loop because of borrowing rules.



# TODO

- [ ] Consistency with skip index
- [ ] Clean up the code
- [ ] Benchmark against standard loops
- [ ] Optimize the execution time
- [ ] Check for further generalization

