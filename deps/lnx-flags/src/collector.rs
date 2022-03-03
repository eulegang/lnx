use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, visit::Visit, Data, DeriveInput, Field};

pub struct Model<'ast> {
    pub ident: &'ast Ident,
    pub fields: Vec<&'ast Field>,
}

pub enum Collector<'ast> {
    Init,
    Struct(&'ast Ident, Vec<&'ast Field>),
    Error(Span, String),
}

impl<'ast> Visit<'ast> for Collector<'ast> {
    fn visit_derive_input(&mut self, i: &'ast DeriveInput) {
        if let Collector::Init = self {
            match i.data {
                Data::Enum(_) | Data::Union(_) => {
                    *self = Collector::Error(i.span(), "only structs are supported".to_string());
                }

                Data::Struct(_) => {
                    *self = Collector::Struct(&i.ident, Vec::new());
                }
            }
        }

        syn::visit::visit_derive_input(self, i);
    }

    fn visit_field(&mut self, field: &'ast syn::Field) {
        if let Collector::Struct(_, ref mut fields) = self {
            fields.push(field)
        }

        syn::visit::visit_field(self, field);
    }
}

impl<'ast> TryFrom<Collector<'ast>> for Model<'ast> {
    type Error = syn::Error;

    fn try_from(value: Collector<'ast>) -> Result<Self, Self::Error> {
        match value {
            Collector::Init => Err(syn::Error::new(
                Span::call_site(),
                "Uninitialized collector",
            )),
            Collector::Struct(ident, fields) => Ok(Model { ident, fields }),
            Collector::Error(span, msg) => Err(syn::Error::new(span, msg)),
        }
    }
}
