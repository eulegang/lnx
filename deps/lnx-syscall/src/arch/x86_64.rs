use crate::{size_of_type, SysCall};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Error, FnArg, Ident, Pat, Result, ReturnType, Type};

pub fn word_size() -> usize {
    8
}

impl SysCall {
    pub(crate) fn generate(&self) -> Result<TokenStream> {
        match &self.sig.output {
            ReturnType::Default => Err(Error::new(
                self.sig.span(),
                "Signature needs an output to work",
            )),

            ReturnType::Type(_, t) => match t.as_ref() {
                Type::Never(_) => self.generate_never(),
                _ => self.generate_out(t),
            },
        }
    }

    fn generate_never(&self) -> Result<TokenStream> {
        let sig = &self.sig;
        let vis = &self.vis;
        let num = &self.num;
        let regs = SysCall::regs(sig.inputs.iter())?;

        Ok(quote! {
            #vis #sig {
                unsafe {
                    ::core::arch::asm!(
                        "syscall",

                        in("rax") #num,
                        #(#regs)*

                        options(noreturn),
                    );
                }
            }
        })
    }

    fn generate_out(&self, typ: &Type) -> Result<TokenStream> {
        let sig = &self.sig;
        let vis = &self.vis;
        let num = &self.num;
        let out = typ;
        let regs = SysCall::regs(sig.inputs.iter())?;

        let out_reg = match size_of_type(typ) {
            8 => Register::AX.qword(),
            4 => Register::AX.dword(),
            2 => Register::AX.word(),
            1 => Register::AX.byte(),
            _ => return Err(Error::new(typ.span(), "incompatible type used")),
        };

        Ok(quote! {
            #vis   #sig {
                let ret: #out;
                unsafe {
                    ::core::arch::asm!(
                        "syscall",

                        in("rax") #num,
                        #(#regs)*

                        lateout(#out_reg) ret,
                        clobber_abi("sysv64")
                    );
                }

                ret
            }
        })
    }

    fn regs<'a>(inputs: impl Iterator<Item = &'a FnArg>) -> Result<Vec<TokenStream>> {
        ARG_ORDER
            .iter()
            .zip(inputs)
            .map(|(reg, arg)| {
                let (name, size) = match arg {
                    FnArg::Receiver(_) => {
                        return Err(Error::new(arg.span(), "no syscall needs a receiver"))
                    }
                    FnArg::Typed(t) => {
                        let name = match t.pat.as_ref() {
                            Pat::Ident(ident) => format!("{}", ident.ident),
                            _ => {
                                return Err(Error::new(
                                    arg.span(),
                                    "must use an ident for function parameters",
                                ));
                            }
                        };

                        let size = size_of_type(&t.ty);

                        (name, size)
                    }
                };

                let name = Ident::new(&name, Span::call_site());
                let reg_name = match size {
                    8 => reg.qword(),
                    4 => reg.dword(),
                    2 => reg.word(),
                    1 => reg.byte(),

                    _ => return Err(Error::new(arg.span(), "incompatible type used")),
                };

                Ok(quote! { in(#reg_name) #name, })
            })
            .collect::<Result<Vec<_>>>()
    }
}

enum Register {
    AX,
    DI,
    SI,
    DX,
    G10,
    G8,
    G9,
}

impl Register {
    fn byte(&self) -> &str {
        match self {
            Register::AX => "al",
            Register::DI => "dil",
            Register::SI => "sil",
            Register::DX => "dl",
            Register::G10 => "r10b",
            Register::G8 => "r8b",
            Register::G9 => "r9b",
        }
    }

    fn word(&self) -> &str {
        match self {
            Register::AX => "ax",
            Register::DI => "di",
            Register::SI => "si",
            Register::DX => "dx",
            Register::G10 => "r10w",
            Register::G8 => "r8w",
            Register::G9 => "r9w",
        }
    }

    fn dword(&self) -> &str {
        match self {
            Register::AX => "eax",
            Register::DI => "edi",
            Register::SI => "esi",
            Register::DX => "edx",
            Register::G10 => "r10d",
            Register::G8 => "r8d",
            Register::G9 => "r9d",
        }
    }

    fn qword(&self) -> &str {
        match self {
            Register::AX => "rax",
            Register::DI => "rdi",
            Register::SI => "rsi",
            Register::DX => "rdx",
            Register::G10 => "r10",
            Register::G8 => "r8",
            Register::G9 => "r9",
        }
    }
}

const ARG_ORDER: [Register; 6] = [
    Register::DI,
    Register::SI,
    Register::DX,
    Register::G10,
    Register::G8,
    Register::G9,
];
