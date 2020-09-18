extern crate proc_macro;
use self::proc_macro::TokenStream;

use proc_macro_hack::proc_macro_hack;
use quote::{quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ExprCall, Token};

extern crate xprlib;

/// Derivative macro arguments
struct DerivativeArgs {
    /// target function to calculate derivative
    fun: ExprCall,
    /// variable w.r.t. we want to get derivative
    wrt: Expr
}

impl Parse for DerivativeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let fun: ExprCall = input.parse()?;
        input.parse::<Token![,]>()?;
        let wrt: Expr = input.parse()?;
        Ok(DerivativeArgs {
            fun,
            wrt,
        })
    }
}

/// Gradient of scalar function.
#[proc_macro_hack]
pub fn derivative(input: TokenStream) -> TokenStream {
    let DerivativeArgs {fun, wrt} = parse_macro_input!(input as DerivativeArgs);
    let out = quote! { {
            #wrt.seed();
            let gres = #fun;
            #wrt.unseed();
            gres.der()
        }
    };
    out.into()
}

/// Gradient arguments
struct GradientArgs {
    fun: ExprCall,
    wrt: Vec<Expr>,
}

impl Parse for GradientArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let fun: ExprCall = input.parse()?;
        input.parse::<Token![,]>()?;
        let wrt = Punctuated::<Expr, Token![,]>::parse_terminated(input)?.into_iter().collect();
        Ok(GradientArgs {
            fun,
            wrt,
        })
    }
}

/// Gradient of scalar function.
#[proc_macro_hack]
pub fn gradient(input: TokenStream) -> TokenStream {
    let GradientArgs {fun, wrt} = parse_macro_input!(input as GradientArgs);
    // count gradient elements to allocate vector once
    let count: Vec<_> = wrt.iter().map(|arg| {
        quote! {
            cnt += #arg.len();
        }
    }).collect();
    // eval gradient
    let grad = wrt.iter().map(|arg| {
        quote! {
            #arg.iter().for_each(|d| {
                d.seed();
                let v = #fun;
                d.unseed();
                ders.push(v.der());
            })
        }
    });
    // make evaluations
    let out = quote! { {
            let mut cnt: usize = 0;
            #(#count;)*
            let mut ders = Vec::with_capacity(cnt);
            #(#grad;)*
            ders
        }
    };
    out.into()
}
