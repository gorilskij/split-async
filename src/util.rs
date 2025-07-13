use proc_macro2::Span;
use syn::{
    Error, Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

pub struct TwoIdents {
    pub sync_ident: Ident,
    pub async_ident: Ident,
}

pub fn split_idents(name: &Ident) -> TwoIdents {
    TwoIdents {
        sync_ident: proc_macro2::Ident::new(&format!("sync_{name}"), Span::call_site()),
        async_ident: proc_macro2::Ident::new(&format!("async_{name}"), Span::call_site()),
    }
}

fn parse_ident_or_litstr(input: &ParseStream) -> syn::Result<Ident> {
    let input_span = input.span();
    if input.peek(Ident) {
        input.parse()
    } else if input.peek(LitStr) {
        let s: LitStr = input.parse()?;
        Ok(Ident::new(&s.value(), s.span()))
    } else {
        Err(syn::Error::new(
            input_span,
            "expected an identifier or a string literal",
        ))
    }
}

impl Parse for TwoIdents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident1 = parse_ident_or_litstr(&input)?;

        if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            if !input.is_empty() {
                let ident2 = parse_ident_or_litstr(&input)?;

                if ident1 == ident2 {
                    return Err(Error::new_spanned(ident2, "repeated identifier"));
                }

                let _: Option<Token![,]> = input.parse()?;

                if !input.is_empty() {
                    return Err(input.error("unexpected token"));
                }

                return Ok(Self {
                    sync_ident: ident1,
                    async_ident: ident2,
                });
            }
        }

        if !input.is_empty() {
            return Err(input.error("unexpected token"));
        }

        Ok(split_idents(&ident1))
    }
}
