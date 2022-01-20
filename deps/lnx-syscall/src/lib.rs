use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Ident, LitInt, Result, Token, Type,
};

#[proc_macro]
pub fn syscall(ts: TS) -> TS {
    let syscall = parse_macro_input!(ts as SysCall);
    match generate(&syscall) {
        Ok(ts) => ts.into(),
        Err(ts) => ts.into_compile_error().into(),
    }
}

struct SysCall {
    num: LitInt,
    name: Ident,
    ret: Type,
    regs: Punctuated<SysCallArg, Token![,]>,
}

struct SysCallArg {
    reg: Ident,
    name: Ident,
    ty: Type,
}

impl Parse for SysCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let num = input.parse()?;
        let _: Token![,] = input.parse()?;
        let name = input.parse()?;
        let _: Token![->] = input.parse()?;
        let ret = input.parse()?;

        let continue_comma: Option<Token![,]> = input.parse()?;

        let regs = if continue_comma.is_some() {
            Punctuated::<SysCallArg, Token![,]>::parse_terminated(input)?
        } else {
            Punctuated::<SysCallArg, Token![,]>::new()
        };

        Ok(SysCall {
            num,
            name,
            ret,
            regs,
        })
    }
}

impl Parse for SysCallArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let reg = input.parse()?;
        let _: Token![=] = input.parse()?;
        let name = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty = input.parse()?;

        Ok(SysCallArg { reg, name, ty })
    }
}

#[cfg(target_arch = "x86_64")]
fn generate(syscall: &SysCall) -> Result<TokenStream> {
    let name = &syscall.name;
    let num = &syscall.num;
    let ret = &syscall.ret;

    let ret_reg = match size_of_type(&ret) {
        8 => "rax",
        4 => "eax",

        size => {
            return Err(syn::Error::new(
                ret.span(),
                format!("Unable to generate size for type {ret:?} estimated: {size}"),
            ))
        }
    };

    let params = syscall.regs.iter().map(|arg| {
        let name = &arg.name;
        let ty = &arg.ty;
        quote! { #name: #ty }
    });

    let regs = syscall.regs.iter().map(|arg| {
        let reg: String = format!("{}", &arg.reg);
        let name = &arg.name;

        quote! { in(#reg) #name, }
    });

    Ok(quote! {
        pub(crate) fn #name(#(#params),*) -> #ret {
            let ret: #ret;
            unsafe {
                asm!(
                    "syscall",

                    in("rax") #num,
                    #(#regs)*

                    lateout(#ret_reg) ret,
                    clobber_abi("sysv64"),
                );
            }

            ret
        }
    })
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

#[cfg(target_pointer_width = "64")]
fn word_size() -> usize {
    8
}

#[cfg(target_pointer_width = "32")]
fn word_size() -> usize {
    4
}
