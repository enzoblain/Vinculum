use super::args::Arg;
use super::derive::Derive;
use super::traits::FfiLangType;

pub struct Struct<T>
where
    T: FfiLangType,
{
    pub(crate) documentation: Vec<String>,
    pub(crate) derives: Vec<Derive>,
    pub(crate) name: String,
    pub(crate) fields: Vec<Arg<T>>,
}
