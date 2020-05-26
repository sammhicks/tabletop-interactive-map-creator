#![allow(dead_code)]

use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone)]
pub struct List<T>(Rc<Vec<T>>);

impl<T> List<T> {
    pub fn new() -> Self {
        Self(Rc::new(Vec::new()))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
}

impl<T: Clone> List<T> {
    pub fn push_front(&self, value: T) -> Self {
        let mut new_items = Vec::with_capacity(self.0.len() + 1);
        new_items.push(value);
        new_items.extend_from_slice(self.0.as_slice());

        Self(Rc::new(new_items))
    }

    pub fn push_back(&self, value: T) -> Self {
        let mut new_items = Vec::with_capacity(self.0.len() + 1);
        new_items.extend_from_slice(self.0.as_slice());
        new_items.push(value);

        Self(Rc::new(new_items))
    }
}

impl<T: Clone + PartialEq> List<T> {
    pub fn set(&self, index: usize, value: T) -> Option<Self> {
        if *self.get(index)? == value {
            return Some(Self(self.0.clone()));
        }

        let mut new_items = Vec::with_capacity(self.0.len());

        new_items.extend_from_slice(self.0.get(0..index)?);
        new_items.push(value);
        new_items.extend_from_slice(self.0.get((index + 1)..self.0.len())?);

        Some(Self(Rc::new(new_items)))
    }
}

impl<T: Default> List<T> {
    pub fn with_length(len: usize) -> Self {
        Self(Rc::new((0..len).map(|_| T::default()).collect()))
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self(Rc::new(Vec::new()))
    }
}

impl<T> std::convert::From<Vec<T>> for List<T> {
    fn from(v: Vec<T>) -> Self {
        Self(Rc::new(v))
    }
}

impl<T> std::iter::FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(Rc::new(Vec::from_iter(iter)))
    }
}

impl<T> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
