#[cfg(test)]
mod functionality {
    use indextree::IndexTree;

    #[test]
    fn creation_indextree() {
        let dims: Vec<usize> = vec![2, 3, 2];
        let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let it: IndexTree = IndexTree::new(&dims, &skips);
        assert_eq!(*it.dimensions(), dims);
    }

    #[test]
    fn creation_zeroindex() {
        let dims: Vec<usize> = vec![2, 3, 2];
        let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let mut it: IndexTree = IndexTree::new(&dims, &skips);
        assert_eq!(*it.get(), [0, 0, 0].to_vec());
    }

    #[test]
    fn incresing() {
        let dims: Vec<usize> = vec![2, 3, 2];
        let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let mut it: IndexTree = IndexTree::new(&dims, &skips);
        assert_eq!(it.inc(), Ok(false));
        assert_eq!(*it.get(), [0, 0, 1].to_vec());
        assert_eq!(it.inc(), Ok(true));
        assert_eq!(*it.get(), [0, 1, 0].to_vec());
        assert_eq!(it.inc(), Ok(false));
        assert_eq!(*it.get(), [0, 1, 1].to_vec());
        assert_eq!(it.inc(), Ok(true));
        assert_eq!(*it.get(), [0, 2, 0].to_vec());
    }

    #[test]
    fn incresing_skip() {
        let dims: Vec<usize> = vec![2, 3, 2];
        let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let mut it: IndexTree = IndexTree::new(&dims, &skips);
        assert_eq!(it.inc(), Ok(false));
        assert_eq!(*it.get(), [0, 0, 1].to_vec());
        assert_eq!(it.inc_skip(0), Ok(true));
        assert_eq!(*it.get(), [0, 1, 0].to_vec());
        assert_eq!(it.inc_skip(1), Ok(false));
        assert_eq!(*it.get(), [0, 1, 1].to_vec());
        assert_eq!(it.inc_skip(2), Ok(false));
        assert_eq!(*it.get(), [1, 0, 0].to_vec());
        assert_eq!(it.inc_skip(3), Ok(false));
        assert_eq!(*it.get(), [1, 1, 0].to_vec());
        assert_eq!(it.inc_skip(4), Ok(false));
        assert_eq!(*it.get(), [1, 1, 1].to_vec());
        assert_eq!(it.inc_skip(5), Ok(true));
        assert_eq!(*it.get(), [1, 2, 0].to_vec());
    }

    #[test]
    fn incresing_skip_v() {
        let dims: Vec<usize> = vec![2, 3, 2];
        let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let mut it: IndexTree = IndexTree::new(&dims, &skips);
        assert_eq!(it.inc(), Ok(false));
        assert_eq!(*it.get(), [0, 0, 1].to_vec());
        assert_eq!(it.inc_skip_v(0), Ok(true));
        assert_eq!(*it.get(), [0, 1, 0].to_vec());
        assert_eq!(it.inc_skip_v(1), Ok(false));
        assert_eq!(*it.get(), [1, 0, 0].to_vec());
        assert_eq!(it.inc_skip_v(2), Ok(false));
        assert_eq!(*it.get(), [1, 1, 0].to_vec());
        assert_eq!(it.inc_skip_v(3), Ok(false));
        assert_eq!(*it.get(), [1, 1, 1].to_vec());
        assert_eq!(it.inc_skip_v(4), Ok(true));
        assert_eq!(*it.get(), [1, 2, 0].to_vec());
        assert_eq!(it.inc_skip_v(5), Ok(false));
        assert_eq!(*it.get(), [1, 2, 1].to_vec());
    }

    #[test]
    fn correctness_check() {
        let dims: Vec<usize> = vec![2, 3, 2];
        let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let mut it: IndexTree = IndexTree::new(&dims, &skips);
        let mut it2: IndexTree = it.clone();
        assert_eq!(it.inc_skip_v(1), Ok(false));
        assert_eq!(*it.get(), [1, 0, 0].to_vec());
        assert_eq!(it.inc_skip_v(1), Err(()));
        assert_eq!(*it.get(), [2, 0, 0].to_vec());
        assert_eq!(it.check(), false);

        for _ in 0..(2 * 3 * 2 - 1) {
            let _ = it2.inc();
            assert_eq!(it2.check(), true);
        }
        assert_eq!(it2.inc(), Err(()));
        assert_eq!(*it2.get(), [2, 0, 0].to_vec());
        assert_eq!(it2.check(), false);

        assert_eq!(it, it2);
    }

    #[test]
    fn iterator_increase() {
        let dims: Vec<usize> = vec![2, 3, 2];
        let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let it: IndexTree = IndexTree::new(&dims, &skips).into_iter();
        let checks: [Vec<usize>; 13] = [
            [0, 0, 0].to_vec(),
            [0, 0, 1].to_vec(),
            [0, 1, 0].to_vec(),
            [0, 1, 1].to_vec(),
            [0, 2, 0].to_vec(),
            [0, 2, 1].to_vec(),
            [1, 0, 0].to_vec(),
            [1, 0, 1].to_vec(),
            [1, 1, 0].to_vec(),
            [1, 1, 1].to_vec(),
            [1, 2, 0].to_vec(),
            [1, 2, 1].to_vec(),
            [2, 0, 0].to_vec(),
        ];

        for (i, item) in it.enumerate() {
            assert_eq!(item, checks[i]);
        }
    }

    #[test]
    fn iterator_increase_skip_v() {
        let dims: Vec<usize> = vec![2, 3, 2];
        let skips: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let mut it: IndexTree = IndexTree::new(&dims, &skips).into_iter();
        let checks: [Vec<usize>; 11] = [
            [0, 0, 0].to_vec(),
            [0, 0, 1].to_vec(),
            [0, 1, 0].to_vec(),
            [0, 1, 1].to_vec(),
            [0, 2, 0].to_vec(),
            [1, 0, 0].to_vec(),
            [1, 0, 1].to_vec(),
            [1, 1, 0].to_vec(),
            [1, 2, 0].to_vec(),
            [1, 2, 1].to_vec(),
            [2, 0, 0].to_vec(),
        ];
        let mut i = 0;
        while let Some(item) = it.next() {
            assert_eq!(item, checks[i]);
            let _ = match i {
                4 => it.inc_skip_v(1),
                6 => it.inc_skip_v(2),
                7 => it.inc_skip_v(2),
                8 => it.inc_skip_v(3),
                _ => Ok(false),
            };
            i = i + 1;
        }
    }
}
