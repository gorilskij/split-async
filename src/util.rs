use derive_more::From;
use proc_macro2::Span;
use syn::{
    Error, Ident, LitStr, Path, PathSegment, Token,
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

#[derive(From)]
pub struct SplitArgs(pub TwoIdents);

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

impl Parse for SplitArgs {
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

                return Ok(Self(TwoIdents {
                    sync_ident: ident1,
                    async_ident: ident2,
                }));
            }
        }

        if !input.is_empty() {
            return Err(input.error("unexpected token"));
        }

        Ok(Self(split_idents(&ident1)))
    }
}

pub struct ChooseArgs {
    pub sync_path: Path,
    pub async_path: Path,
}

fn parse_path_or_litstr(input: &ParseStream) -> syn::Result<Path> {
    let input_span = input.span();
    if input.peek(Token![::]) || input.peek(Ident) {
        input.parse()
    } else if input.peek(LitStr) {
        let s: LitStr = input.parse()?;
        Ok(Ident::new(&s.value(), s.span()).into())
    } else {
        Err(syn::Error::new(
            input_span,
            "expected an identifier or a string literal",
        ))
    }
}

fn split_path(path: &Path) -> syn::Result<ChooseArgs> {
    let tail_segment = path
        .segments
        .last()
        .ok_or_else(|| Error::new_spanned(path, "bad path"))?;
    if !tail_segment.arguments.is_empty() {
        return Err(Error::new_spanned(tail_segment, "must be an identifier"));
    }
    let TwoIdents {
        sync_ident,
        async_ident,
    } = split_idents(&tail_segment.ident);
    let mut sync_path = path.clone();
    *sync_path.segments.last_mut().unwrap() = PathSegment::from(sync_ident);
    let mut async_path = path.clone();
    *async_path.segments.last_mut().unwrap() = PathSegment::from(async_ident);
    Ok(ChooseArgs {
        sync_path,
        async_path,
    })
}

impl Parse for ChooseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path1 = parse_path_or_litstr(&input)?;

        if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            if !input.is_empty() {
                let path2 = parse_path_or_litstr(&input)?;

                if path1 == path2 {
                    return Err(Error::new_spanned(path2, "repeated identifier"));
                }

                let _: Option<Token![,]> = input.parse()?;

                if !input.is_empty() {
                    return Err(input.error("unexpected token"));
                }

                return Ok(Self {
                    sync_path: path1,
                    async_path: path2,
                });
            }
        }

        if !input.is_empty() {
            return Err(input.error("unexpected token"));
        }

        split_path(&path1)
    }
}
