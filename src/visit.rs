use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprBlock, File, Item, Stmt, parse_quote,
    visit_mut::{self, VisitMut, visit_expr_closure_mut, visit_item_mut},
};

use crate::util::ChooseArgs;

pub struct IntoSync;

impl IntoSync {
    pub fn into_sync(&mut self, item: TokenStream) -> TokenStream {
        let mut syntax_tree: File = syn::parse(item.into()).unwrap();
        self.visit_file_mut(&mut syntax_tree);
        quote!(#syntax_tree)
    }
}

impl VisitMut for IntoSync {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        // Delegate to the default impl to visit nested expressions.
        visit_mut::visit_expr_mut(self, node);

        match node {
            Expr::Await(expr) => *node = (*expr.base).clone(),

            Expr::Async(expr) => {
                let inner = &expr.block;
                let sync_expr = if let [Stmt::Expr(expr, None)] = inner.stmts.as_slice() {
                    // remove useless braces when there is only one statement
                    expr.clone()
                } else {
                    Expr::Block(ExprBlock {
                        attrs: expr.attrs.clone(),
                        block: inner.clone(),
                        label: None,
                    })
                };
                *node = sync_expr;
            }

            e @ Expr::Macro(_) => {
                let Expr::Macro(expr) = e else { unreachable!() };
                if let Some(ident) = expr.mac.path.get_ident() {
                    if ident == "choose" {
                        let input: proc_macro::TokenStream = expr.mac.tokens.clone().into();
                        let ChooseArgs { sync_path, .. } = syn::parse(input).unwrap();
                        *e = parse_quote!(#sync_path)
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_item_mut(&mut self, i: &mut Item) {
        match i {
            Item::Fn(item_fn) => {
                item_fn.sig.asyncness = None;
            }
            _ => {}
        }
        visit_item_mut(self, i);
    }

    fn visit_expr_closure_mut(&mut self, i: &mut syn::ExprClosure) {
        visit_expr_closure_mut(self, i);
        i.asyncness = None;
    }
}

pub struct IntoAsync;

impl IntoAsync {
    pub fn into_async(&mut self, item: TokenStream) -> TokenStream {
        let mut syntax_tree: File = syn::parse(item.into()).unwrap();
        self.visit_file_mut(&mut syntax_tree);
        quote!(#syntax_tree)
    }
}

impl VisitMut for IntoAsync {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        // Delegate to the default impl to visit nested expressions.
        visit_mut::visit_expr_mut(self, node);

        match node {
            e @ Expr::Macro(_) => {
                let Expr::Macro(expr) = e else { unreachable!() };
                if let Some(ident) = expr.mac.path.get_ident() {
                    if ident == "choose" {
                        // let mut syntax_tree: Punctuated<syn::Ident, Comma> =
                        // syn::parse().unwrap();
                        let input: proc_macro::TokenStream = expr.mac.tokens.clone().into();
                        let ChooseArgs { async_path, .. } = syn::parse(input).unwrap();
                        *e = parse_quote!(#async_path)
                    }
                }
            }
            _ => {}
        }
    }
}
