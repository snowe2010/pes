#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
#[macro_use]
extern crate lazy_static;
extern crate pes_common;

use proc_macro2::Span;
use proc_macro::TokenStream;
use quote::ToTokens;
//use std::Result;
use syn::DeriveInput;
use syn::Fields;
use syn::FnArg;
use syn::FnDecl;
use syn::Item;
use syn::Ident;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::Type;

use pes_common::{CommandMetadata, CommandBus, Command};
use std::sync::RwLock;


#[proc_macro_derive(Command)]
pub fn event(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.into()
}

fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let stringified_name = format!("MYEVENT_METADATA_{}", name.to_string().to_uppercase());
//    println!("Stringified name {}", stringified_name);
    let ident = Ident::from(stringified_name);
    quote! {
        lazy_static! {
            static ref #ident: RwLock<CommandMetadata<#name>> = RwLock::new(CommandMetadata::new());
        }
        impl Command for #name {
            fn event_metadata<F, R>(f: F) -> R where F: FnOnce(&CommandMetadata<Self>) -> R {
                f(&*#ident.read().unwrap())
            }

            fn mut_metadata<F, R>(f: F) -> R where F: FnOnce(&mut CommandMetadata<Self>) -> R {
                f(&mut *#ident.write().unwrap())
            }
        }
    }
}

#[proc_macro_attribute]
pub fn event_handler(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    match item {
        Item::Fn(ref struct_item) => {
            let declaration: &Box<FnDecl> = &struct_item.decl;
            let inputs: &Punctuated<FnArg, Comma> = &declaration.inputs;
            println!("inputs {}", inputs.len());
            TokenStream::empty()
        }
        _ => {
//            item.span().unstable()
//                .error("This is not a struct")
//                .emit();
            TokenStream::empty()
        }
    }

}

