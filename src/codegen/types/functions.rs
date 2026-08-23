use crate::codegen::types::GenericResolver;
use crate::codegen::types::args::Arg;
use crate::codegen::types::derive::Derive;
use crate::codegen::types::traits::FfiLangType;

pub struct Function<T>
where
    T: FfiLangType,
{
    pub(crate) documentation: Vec<String>,
    pub(crate) derives: Vec<Derive>,
    pub(crate) name: String,
    pub(crate) generic_resolver: GenericResolver,
    pub(crate) args: Vec<Arg<T>>,
    pub(crate) return_type: T,
}
