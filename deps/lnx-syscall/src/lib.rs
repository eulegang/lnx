use proc_macro::TokenStream as TS;

use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitInt, Result, Signature, Token, Type, Visibility,
};

mod arch;

use arch::word_size;

#[proc_macro]
pub fn syscall(ts: TS) -> TS {
    let syscall = parse_macro_input!(ts as SysCall);
    match syscall.generate() {
        Ok(ts) => ts.into(),
        Err(ts) => ts.into_compile_error().into(),
    }
}

struct SysCall {
    num: LitInt,
    vis: Visibility,
    sig: Signature,
}

impl Parse for SysCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let num = input.parse()?;
        let _: Token![,] = input.parse()?;
        let vis = input.parse()?;
        let sig = input.parse()?;

        Ok(SysCall { num, vis, sig })
    }
}

fn size_of_type(ty: &Type) -> usize {
    match ty {
        Type::Path(p) => {
            let segs = &p.path.segments;

            if segs.len() > 1 {
                return 0;
            }

            match format!("{}", segs.first().unwrap().ident).as_str() {
                "i32" => 4,
                "u32" => 4,
                "i64" => 8,
                "u64" => 8,
                "usize" => word_size(),
                "isize" => word_size(),

                _ => 0,
            }
        }

        Type::Reference(_) => word_size(),
        Type::Ptr(_) => word_size(),

        _ => 0,
    }
}
