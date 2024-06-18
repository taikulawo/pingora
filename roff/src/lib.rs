pub(crate) mod config;
pub(crate) mod listener;
pub(crate) mod proxy;
pub(crate) mod stream;
pub type SmallString = smol_str::SmolStr;
pub type SmallVec<T> = smallvec::SmallVec<[T; 16]>;
