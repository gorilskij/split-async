use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprBlock, File, GenericArgument, GenericParam, Item, PathArguments, PathSegment, Stmt,
    Type, TypeParamBound, WherePredicate, parse_quote,
    visit_mut::{self, VisitMut, visit_expr_closure_mut, visit_item_mut, visit_path_segment_mut},
};

use crate::util::TwoIdents;

// pub struct ReplaceGenericType<'a> {
//     generic_type: &'a str,
//     arg_type: &'a PathSegment,
// }

// impl<'a> ReplaceGenericType<'a> {
//     pub fn new(generic_type: &'a str, arg_type: &'a PathSegment) -> Self {
//         Self {
//             generic_type,
//             arg_type,
//         }
//     }

//     pub fn replace_generic_type(item: &mut Item, generic_type: &'a str,
// arg_type: &'a PathSegment) {         let mut s = Self::new(generic_type,
// arg_type);         s.visit_item_mut(item);
//     }
// }

// impl<'a> VisitMut for ReplaceGenericType<'a> {
//     fn visit_item_mut(&mut self, i: &mut Item) {
//         if let Item::Fn(item_fn) = i {
//             // remove generic type from generics <T, F>
//             let args = item_fn
//                 .sig
//                 .generics
//                 .params
//                 .iter()
//                 .filter_map(|param| {
//                     if let GenericParam::Type(type_param) = &param {
//                         if type_param.ident.to_string().eq(self.generic_type)
// {                             None
//                         } else {
//                             Some(param)
//                         }
//                     } else {
//                         Some(param)
//                     }
//                 })
//                 .collect::<Vec<_>>();
//             item_fn.sig.generics.params =
// args.into_iter().cloned().collect();

//             // remove generic type from where clause
//             if let Some(where_clause) = &mut
// item_fn.sig.generics.where_clause {                 let new_where_clause =
// where_clause                     .predicates
//                     .iter()
//                     .filter_map(|predicate| {
//                         if let WherePredicate::Type(predicate_type) =
// predicate {                             if let Type::Path(p) =
// &predicate_type.bounded_ty {                                 if
// p.path.segments[0].ident.to_string().eq(self.generic_type) {
// None                                 } else {
//                                     Some(predicate)
//                                 }
//                             } else {
//                                 Some(predicate)
//                             }
//                         } else {
//                             Some(predicate)
//                         }
//                     })
//                     .collect::<Vec<_>>();

//                 where_clause.predicates =
// new_where_clause.into_iter().cloned().collect();             };
//         }
//         visit_item_mut(self, i)
//     }

//     fn visit_path_segment_mut(&mut self, i: &mut PathSegment) {
//         // replace generic type with target type
//         if i.ident.to_string().eq(&self.generic_type) {
//             *i = self.arg_type.clone();
//         }
//         visit_path_segment_mut(self, i);
//     }
// }

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
                        let TwoIdents { sync_ident, .. } = syn::parse(input).unwrap();
                        *e = parse_quote!(#sync_ident)
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_item_mut(&mut self, i: &mut Item) {
        // // find generic parameter of Future and replace it with its Output type
        // if let Item::Fn(item_fn) = i {
        //     let mut inputs: Vec<(String, PathSegment)> = vec![];

        //     // generic params: <T:Future<Output=()>, F>
        //     for param in &item_fn.sig.generics.params {
        //         // generic param: T:Future<Output=()>
        //         if let GenericParam::Type(type_param) = param {
        //             let generic_type_name = type_param.ident.to_string();

        //             // bound: Future<Output=()>
        //             for bound in &type_param.bounds {
        //                 inputs.extend(search_trait_bound(&generic_type_name, bound));
        //             }
        //         }
        //     }

        //     if let Some(where_clause) = &item_fn.sig.generics.where_clause {
        //         for predicate in &where_clause.predicates {
        //             if let WherePredicate::Type(predicate_type) = predicate {
        //                 let generic_type_name = if let Type::Path(p) =
        // &predicate_type.bounded_ty {
        // p.path.segments[0].ident.to_string()                 } else {
        //                     panic!("Please submit an issue");
        //                 };

        //                 for bound in &predicate_type.bounds {
        //                     inputs.extend(search_trait_bound(&generic_type_name,
        // bound));                 }
        //             }
        //         }
        //     }

        //     for (generic_type_name, path_seg) in &inputs {
        //         ReplaceGenericType::replace_generic_type(i, generic_type_name,
        // path_seg);     }
        // }
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

// fn search_trait_bound(
//     generic_type_name: &str,
//     bound: &TypeParamBound,
// ) -> Vec<(String, PathSegment)> {
//     let mut inputs = vec![];

//     if let TypeParamBound::Trait(trait_bound) = bound {
//         let segment =
// &trait_bound.path.segments[trait_bound.path.segments.len() - 1];         let
// name = segment.ident.to_string();         if name.eq("Future") {
//             // match Future<Output=Type>
//             if let PathArguments::AngleBracketed(args) = &segment.arguments {
//                 // binding: Output=Type
//                 if let GenericArgument::AssocType(binding) = &args.args[0] {
//                     if let Type::Path(p) = &binding.ty {
//                         inputs.push((generic_type_name.to_owned(),
// p.path.segments[0].clone()));                     }
//                 }
//             }
//         }
//     }
//     inputs
// }

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
                        let TwoIdents { async_ident, .. } = syn::parse(input).unwrap();
                        *e = parse_quote!(#async_ident)
                    }
                }
            }
            _ => {}
        }
    }
}
