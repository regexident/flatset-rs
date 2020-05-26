// @article{DBLP:journals/corr/KhuongM15,
//   author    = {Paul{-}Virak Khuong and
//                Pat Morin},
//   title     = {Array Layouts for Comparison-Based Searching},
//   journal   = {CoRR},
//   volume    = {abs/1509.05053},
//   year      = {2015},
//   url       = {http://arxiv.org/abs/1509.05053},
//   timestamp = {Thu, 01 Oct 2015 14:28:48 +0200},
//   biburl    = {http://dblp.uni-trier.de/rec/bib/journals/corr/KhuongM15},
//   bibsource = {dblp computer science bibliography, http://dblp.org}
// }

use std::collections::VecDeque;
use std::fmt;
use std::ops::Range;

pub struct LevelOrderIterator {
    queue: VecDeque<Range<usize>>,
}

impl LevelOrderIterator {
    fn from_range(range: Range<usize>) -> LevelOrderIterator {
        let mut queue = VecDeque::new();
        queue.push_back(range);
        LevelOrderIterator { queue: queue }
    }
}

impl Iterator for LevelOrderIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().and_then(|range| {
            let len = range.len();
            if len > 0 {
                let index = range.start + (len >> 1);
                let left = (range.start)..(index);
                let right = (index + 1)..(range.end);
                if left.len() > 0 {
                    self.queue.push_back(left);
                }
                if right.len() > 0 {
                    self.queue.push_back(right);
                }
                Some(index)
            } else {
                None
            }
        })
    }
}

use std::cmp::Ordering;

pub trait LevelOrder {
    type Item;

    fn branchful_binary_search(&self, x: &Self::Item) -> Result<usize, usize>
    where
        Self::Item: Ord;
    fn branchful_binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&Self::Item) -> Ordering;

    fn branchless_binary_search(&self, x: &Self::Item) -> Result<usize, usize>
    where
        Self::Item: Ord;
    fn branchless_binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&Self::Item) -> Ordering;

    fn level_order(&mut self)
    where
        Self::Item: Ord;

    fn branchful_level_order_search(&self, x: &Self::Item) -> Result<usize, usize>
    where
        Self::Item: Ord;
    fn branchful_level_order_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&Self::Item) -> Ordering;

    fn branchless_level_order_search(&self, x: &Self::Item) -> Result<usize, usize>
    where
        Self::Item: Ord;
    fn branchless_level_order_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&Self::Item) -> Ordering;
}

impl<T> LevelOrder for [T]
where
    T: Ord + Debug,
{
    type Item = T;

    fn level_order(&mut self)
    where
        T: Ord,
    {
        let len = self.len();
        let order = LevelOrderIterator::from_range(0..len);
        let mut indexes: Vec<_> = order.collect();
        for i in 0..len {
            while indexes[i] != indexes[indexes[i]] {
                self.swap(indexes[i], indexes[indexes[i]]);
                let j = indexes[i];
                indexes.swap(i, j);
            }
        }
    }

    // #[inline]
    #[inline(never)]
    fn branchful_binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.branchful_binary_search_by(|p| p.cmp(x))
    }

    #[inline(never)]
    fn branchful_binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        let mut base = 0usize;
        let mut s = self;
        loop {
            let (head, tail) = s.split_at(s.len() >> 1);
            if tail.is_empty() {
                return Err(base);
            }
            match f(&tail[0]) {
                Ordering::Less => {
                    base += head.len() + 1;
                    s = &tail[1..];
                }
                Ordering::Greater => s = head,
                Ordering::Equal => return Ok(base + head.len()),
            }
        }
    }

    #[inline(never)]
    fn branchless_binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.branchless_binary_search_by(|p| p.cmp(x))
    }

    #[inline(never)]
    fn branchless_binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        let mut pos = 0;
        let mut len = self.len();
        while len > 1 {
            let half = len >> 1;
            pos += ((f(&self[pos + half]) != Ordering::Greater) as usize) * half;
            len -= half;
        }
        if f(&self[pos]) == Ordering::Equal {
            Ok(pos)
        } else {
            Err(pos)
        }
    }

    // #[inline]
    #[inline(never)]
    fn branchful_level_order_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.branchful_level_order_search_by(|p| p.cmp(x))
    }

    // #[inline]
    #[inline(never)]
    fn branchful_level_order_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        let len: usize = self.len();
        let mut i: usize = 0;
        while i < len {
            match f(&self[i]) {
                Ordering::Equal => return Ok(i),
                Ordering::Less => {
                    i = (i << 1) + 2;
                }
                Ordering::Greater => {
                    i = (i << 1) + 1;
                }
            }
        }
        let ffs = |i: usize| (if i == 0 { 0 } else { i.trailing_zeros() + 1 }) as usize;
        let j = (i + 1) >> ffs(!(i + 1));
        return if j == 0 { Err(len) } else { Ok(j - 1) };
    }

    // #[inline]
    #[inline(never)]
    fn branchless_level_order_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.branchless_level_order_search_by(|p| p.cmp(x))
    }

    // #[inline]
    #[inline(never)]
    fn branchless_level_order_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        let len: usize = self.len();
        let mut i = 0;
        while i < len {
            i = (i << 1) + 1 + ((f(&self[i]) == Ordering::Less) as usize);
        }
        let ffs = |i: usize| (if i == 0 { 0 } else { i.trailing_zeros() + 1 }) as usize;
        let j = (i + 1) >> ffs(!(i + 1));
        return if j == 0 { Err(len) } else { Ok(j - 1) };
    }
}

use std::fmt::Debug;
use std::iter::FromIterator;
use std::slice::Iter;

pub struct FlatSet<T: Ord> {
    inner: Vec<T>,
}

impl<T> FlatSet<T>
where
    T: Ord + Debug,
{
    pub fn new() -> FlatSet<T> {
        FlatSet { inner: Vec::new() }
    }

    pub fn iter(&self) -> Iter<T> {
        self.inner.iter()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn binary_order<I: IntoIterator<Item = T>>(iter: I) -> FlatSet<T> {
        FlatSet {
            inner: iter.into_iter().collect(),
        }
    }

    #[inline(never)]
    pub fn contains(&self, value: &T) -> bool {
        self.inner.branchful_level_order_search(value).is_ok()
    }

    #[inline(never)]
    pub fn branchful_binary_search_contains(&self, value: &T) -> bool {
        self.inner.branchful_binary_search(value).is_ok()
    }

    #[inline(never)]
    pub fn branchless_binary_search_contains(&self, value: &T) -> bool {
        self.inner.branchless_binary_search(value).is_ok()
    }

    #[inline(never)]
    pub fn branchful_level_order_search_contains(&self, value: &T) -> bool {
        self.inner.branchful_level_order_search(value).is_ok()
    }

    #[inline(never)]
    pub fn branchless_level_order_search_contains(&self, value: &T) -> bool {
        self.inner.branchless_level_order_search(value).is_ok()
    }
}

impl<T> FromIterator<T> for FlatSet<T>
where
    T: Ord + Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> FlatSet<T> {
        let mut vec: Vec<_> = iter.into_iter().collect();
        vec.level_order();
        FlatSet { inner: vec }
    }
}

impl<T> fmt::Debug for FlatSet<T>
where
    T: Ord + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::{FlatSet, LevelOrder};
    use std::iter::FromIterator;

    #[test]
    fn level_order() {
        let range = 0..10;
        let mut level_order: Vec<_> = range.collect();
        level_order.level_order();
        assert_eq!(level_order, vec![5, 2, 8, 1, 4, 7, 9, 0, 3, 6])
    }

    #[test]
    fn branchful_binary_search_by() {
        let range = 0..10;
        let set = FlatSet::binary_order(range.clone());
        assert!(set.len() == range.len());

        for i in range {
            assert!(set.branchful_binary_search_contains(&i) == true);
        }
    }

    #[test]
    fn branchless_binary_search_by() {
        let range = 0..10;
        let set = FlatSet::binary_order(range.clone());
        assert!(set.len() == range.len());

        for i in range {
            assert!(set.branchless_binary_search_contains(&i) == true);
        }
    }

    #[test]
    fn branchful_level_order_search_by() {
        let range = 0..10;
        let set = FlatSet::from_iter(range.clone());
        assert!(set.len() == range.len());

        for i in range {
            assert!(set.branchful_level_order_search_contains(&i) == true);
        }
    }

    #[test]
    fn branchless_level_order_search_by() {
        let range = 0..10;
        let set = FlatSet::from_iter(range.clone());
        assert!(set.len() == range.len());

        for i in range {
            assert!(set.branchless_level_order_search_contains(&i) == true);
        }
    }
}
