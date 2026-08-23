use crate::codegen::types::errors::InvalidArgumentName;
use crate::codegen::types::traits::FfiLangType;

pub struct Arg<T>
where
    T: FfiLangType,
{
    pub(crate) name: String,
    pub(crate) r#type: T,
}

impl<T> Arg<T>
where
    T: FfiLangType,
{
    pub fn try_new(name: impl Into<String>, r#type: T) -> Result<Self, InvalidArgumentName> {
        let name: String = name.into();

        if !is_valid_variable_name(&name) {
            return Err(InvalidArgumentName(name));
        }

        Ok(Self {
            name: normalize_arg_name(name),
            r#type,
        })
    }
}

#[inline]
pub(crate) fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut bytes = name.as_bytes().iter();

    let Some(&first) = bytes.next() else {
        return false;
    };

    if !first.is_ascii_lowercase() {
        return false;
    }

    bytes.all(|&c| c.is_ascii_alphanumeric() || c == b'_' || c == b'\'')
}

#[inline]
pub(crate) fn normalize_arg_name(name: String) -> String {
    if is_rust_keyword(&name) {
        format!("r#{name}")
    } else {
        name
    }
}

pub(crate) fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
    )
}
