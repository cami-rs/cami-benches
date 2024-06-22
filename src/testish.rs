use crate::data::{Data, DataItems, OptAsData};
use alloc::vec::IntoIter;

extern crate alloc;

#[repr(transparent)]
pub struct DataTest<T>(IntoIter<T>);

impl<T> DataItems for DataTest<T>
where
    DataTest<T>: OptAsData,
{
    fn num_items(&mut self) -> usize {
        self.0.len()
    }
}

impl Data for DataTest<String> {
    fn string(&mut self) -> String {
        self.0.next().unwrap()
    }
}

impl<T> DataTest<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self(vec.into_iter())
    }
}
