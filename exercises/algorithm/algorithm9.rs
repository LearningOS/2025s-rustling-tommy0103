/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        let mut ret = Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        };
        // ret.items.push(T::default());
        ret
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count = self.count + 1;
        let mut idx = self.items.len() - 1;
        
        loop {
            if idx == 1 {
                break; // 已经是根节点
            }
            let parent_idx = idx / 2;
            if (self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                break; // 满足堆性质
            }
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		if self.children_present(self.left_child_idx(idx)) {
            self.left_child_idx(idx)
        }
        else if self.children_present(self.right_child_idx(idx)){
            self.right_child_idx(idx)
        }
        else {
            0
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + std::cmp::PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        let item = self.items.swap_remove(1);
        self.count -= 1;
        
        if self.items.len() > 1 {
            let mut idx = 1;
            loop {
                let left = 2 * idx;
                let right = 2 * idx + 1;
                let mut smallest = idx;

                if left < self.items.len() && !(self.comparator)(&self.items[smallest], &self.items[left]) {
                    smallest = left;
                }

                if right < self.items.len() && !(self.comparator)(&self.items[smallest], &self.items[right]) {
                    smallest = right;
                }

                if smallest != idx {
                    self.items.swap(idx, smallest);
                    idx = smallest;
                } else {
                    break;
                }
            }
        }

        Some(item)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}