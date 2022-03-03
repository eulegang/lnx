use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, BinOp, Field, UnOp};

use crate::collector::Model;

enum Operator {
    Bin(BinOp),
    Un(UnOp),
}

pub struct Op {
    tr: Ident,
    op: Operator,
    meth: Ident,
}

impl Op {
    pub fn or() -> Op {
        Op {
            tr: format_ident!("BitOr"),
            op: Operator::Bin(BinOp::BitOr(Default::default())),
            meth: format_ident!("bitor"),
        }
    }

    pub fn and() -> Op {
        Op {
            tr: format_ident!("BitAnd"),
            op: Operator::Bin(BinOp::BitAnd(Default::default())),
            meth: format_ident!("bitand"),
        }
    }

    pub fn not() -> Op {
        Op {
            tr: format_ident!("Not"),
            op: Operator::Un(UnOp::Not(Default::default())),
            meth: format_ident!("not"),
        }
    }

    pub fn to_impl<'a>(&'a self, model: &'a Model<'a>) -> Impl<'a> {
        let op = self;

        Impl { op, model }
    }
}

pub struct Impl<'a> {
    op: &'a Op,
    model: &'a Model<'a>,
}

impl ToTokens for Impl<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tr = &self.op.tr;
        let ty = &self.model.ident;
        let meth = &self.op.meth;
        let op = &self.op.op;

        let curly = self.model.fields.get(0).map_or(true, |f| f.ident.is_some());

        let mapping = self
            .model
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let name = if let Some(s) = &f.ident {
                    s.clone()
                } else {
                    format_ident!("arg{i}")
                };

                match op {
                    Operator::Bin(bin) => {
                        let s = make_ref(&format_ident!("self"), f, i);
                        let o = make_ref(&format_ident!("other"), f, i);

                        quote! { let #name = #s #bin #o; }
                    }

                    Operator::Un(un) => {
                        let s = make_ref(&format_ident!("self"), f, i);

                        quote! { let #name = #un #s; }
                    }
                }
            })
            .collect::<Vec<_>>();

        let mut args = Punctuated::<Ident, Comma>::new();
        for (i, field) in self.model.fields.iter().enumerate() {
            if let Some(name) = &field.ident {
                args.push(name.clone())
            } else {
                args.push(format_ident!("arg{i}"));
            }
        }

        let collect = if curly {
            quote! { #ty { #args } }
        } else {
            quote! { #ty ( #args ) }
        };

        let func_def = match op {
            Operator::Bin(_) => {
                quote! {
                    fn #meth(self, other: #ty) -> #ty {
                        #(#mapping)*

                        #collect
                    }
                }
            }
            Operator::Un(_) => {
                quote! {
                    fn #meth(self) -> #ty {
                        #(#mapping)*

                        #collect
                    }
                }
            }
        };

        let q = quote! {
            #[automatically_derived]
            impl ::core::ops::#tr for #ty {
                type Output = #ty;

                #func_def
            }
        };

        q.to_tokens(tokens);
    }
}

fn make_ref(base: &Ident, field: &Field, pos: usize) -> syn::Expr {
    let member = if let Some(id) = &field.ident {
        syn::Member::Named(id.clone())
    } else {
        let index = pos as u32;
        let span = Span::call_site();
        syn::Member::Unnamed(syn::Index { index, span })
    };

    let attrs = Vec::new();

    let ident = base.clone();
    let arguments = syn::PathArguments::None;
    let seg = syn::PathSegment { ident, arguments };
    let mut segments = syn::punctuated::Punctuated::new();
    segments.push(seg);
    let leading_colon = None;

    let path = syn::Path {
        leading_colon,
        segments,
    };

    let qself = None;

    let expr_path = syn::ExprPath { attrs, qself, path };

    let base = Box::new(syn::Expr::Path(expr_path));
    let dot_token = syn::token::Dot::default();

    let attrs = Vec::new();

    syn::ExprField {
        attrs,
        base,
        dot_token,
        member,
    }
    .into()
}
