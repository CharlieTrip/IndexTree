use index_tree::IndexTree;

const _OBJ: [u8; 11] = [10, 15, 80, 72, 1, 0, 81, 72, 30, 8, 100];
const DIM: [usize; 6] = [10, 10, 100, 10, 10, 10];

fn main() {
  let mut tree = IndexTree::new(&DIM.to_vec(), &vec![]);
  while tree.check() {
    let _ = tree.get();
    let _ = tree.inc();
  }
  println!("Yes?");
}
