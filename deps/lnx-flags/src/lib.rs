use quote::ToTokens;

use syn::{parse_macro_input, visit::Visit, DeriveInput};

mod collector;
mod emit;

#[proc_macro_derive(Flags)]
pub fn flags(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let mut collector = collector::Collector::Init;
    collector.visit_derive_input(&input);

    let model: collector::Model = match collector.try_into() {
        Ok(m) => m,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };

    let mut tokens = proc_macro2::TokenStream::new();
    emit::Op::and().to_impl(&model).to_tokens(&mut tokens);
    emit::Op::or().to_impl(&model).to_tokens(&mut tokens);
    emit::Op::not().to_impl(&model).to_tokens(&mut tokens);

    tokens.into()
}
