
use crate::amf::error;

pub type DecodeResult<T> = Result<T, error::DecodeError>;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair<K, V> {
    /// The key of the pair.
    pub key: K,

    /// The value of the pair.
    pub value: V,
}

#[allow(dead_code)]
fn iter_boxed<I, T>(iter: I) -> Box<dyn Iterator<Item = T>>
where
    I: Iterator<Item = T> + 'static,
{
    Box::new(iter)
}
