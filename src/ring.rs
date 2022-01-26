use std::iter::Chain;
use std::slice::Iter;

const DEFAULT_CAP: usize = 30;

#[derive(Clone, Debug)]
pub struct Ring<T, const CAP: usize = DEFAULT_CAP> {
    items: Vec<T>,
    index: usize,
}

impl<T, const CAP: usize> Ring<T, CAP> {
    pub fn new() -> Self {
        debug_assert!(CAP != 0);

        Self {
            items: Vec::new(),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        if self.is_full() {
            CAP
        } else {
            self.index
        }
    }

    pub fn is_full(&self) -> bool {
        debug_assert!(self.items.len() <= CAP);
        self.items.len() == CAP
    }

    pub fn iter(&self) -> Chain<Iter<T>, Iter<T>> {
        let start = &self.items[self.index..];
        let end = &self.items[..self.index];

        start.iter().chain(end)
    }

    pub fn push(&mut self, elem: T) {
        if self.is_full() {
            self.items[self.index] = elem;
        } else {
            debug_assert!(self.items.len() == self.index);
            self.items.push(elem);
        }

        self.inc();
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.index = 0;
    }

    fn inc(&mut self) {
        self.index += 1;

        if self.index == CAP {
            self.index = 0;
        }
    }
}

impl<T, const CAP: usize> Default for Ring<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, const CAP: usize> IntoIterator for &'a Ring<T, CAP> {
    type IntoIter = Chain<Iter<'a, T>, Iter<'a, T>>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
