use std::{
    marker::PhantomData,
    ops::{Index, IndexMut}
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Id<T> {
    index: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> { }

impl<T> Id<T> {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdColl<T> {
    data: Vec<T>,
}

impl<T> IdColl<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) -> Id<T> {
        let index = self.data.len();
        self.data.push(value);
        Id::new(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Id<T>, &T)> {
        self.data.iter().enumerate().map(|(i, v)| (Id::new(i), v))
    }
}

impl<T> From<Vec<T>> for IdColl<T> {
    fn from(vec: Vec<T>) -> Self {
        Self { data: vec }
    }
}

impl<T> Index<Id<T>> for IdColl<T> {
    type Output = T;

    fn index(&self, id: Id<T>) -> &Self::Output {
        &self.data[id.index]
    }
}

impl<T> IndexMut<Id<T>> for IdColl<T> {
    fn index_mut(&mut self, id: Id<T>) -> &mut Self::Output {
        &mut self.data[id.index]
    }
}
