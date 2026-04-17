use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use crate::util::indexable::Indexable;

pub struct By<K: Indexable, V, const N: usize>([V;N], PhantomData<K>);
impl<K: Indexable, V, const N: usize> Index<K> for By<K, V, N> {
  type Output = V;
  fn index(&self, index: K) -> &Self::Output {
    &self.0[index.idx()]
  }
}
impl<K: Indexable, V, const N: usize> IndexMut<K> for By<K, V, N> {
  fn index_mut(&mut self, index: K) -> &mut Self::Output { &mut self.0[index.idx()] }
}
impl<K:Indexable, V: Copy, const N: usize> By<K, V, N> {
  pub const fn new(value: V) -> Self {
    Self([value; N], PhantomData)
  }
}