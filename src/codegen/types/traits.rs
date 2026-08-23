use crate::codegen::types::generics::GenericResolver;

pub trait FfiType {
    fn rust_type_name(&self) -> String;
    fn rust_return_conversion(&self) -> &'static str;

    fn is_generic(&self) -> bool;
    fn resolve_generics(&mut self, resolver: &mut GenericResolver);
}

pub trait FfiTypeCodegen {
    fn rust_value_expr(&self, value_name: &str, type_param: &str) -> String;
    fn target_pattern(&self, binding_name: &str) -> String;
    fn target_value_expr(&self, binding_name: &str) -> String;
}

pub trait FfiLangType: FfiType + FfiTypeCodegen {}

impl<T> FfiLangType for T where T: FfiType + FfiTypeCodegen {}
