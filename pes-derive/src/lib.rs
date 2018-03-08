#![feature(proc_macro)]

extern crate pes_common;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput, FnArg, FnDecl, Ident, Item};
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[proc_macro_derive(Command)]
pub fn command(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gen = generate_command_impl(&ast);
    gen.into()
}
#[proc_macro_derive(Event)]
pub fn event(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gen = generate_event_impl(&ast);
    gen.into()
}

/// Generate the Command trait implementation for a specific Command
fn generate_command_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let stringified_name = format!("COMMAND_METADATA_{}", name.to_string().to_uppercase());
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

/// Generate the Event trait implementation for a specific Event
fn generate_event_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let stringified_name = format!("EVENT_METADATA_{}", name.to_string().to_uppercase());
    let ident = Ident::from(stringified_name);
    quote! {
        lazy_static! {
            static ref #ident: RwLock<EventMetadata<#name>> = RwLock::new(EventMetadata::new());
        }
        impl Event for #name {
            fn event_metadata<F, R>(f: F) -> R where F: FnOnce(&EventMetadata<Self>) -> R {
                f(&*#ident.read().unwrap())
            }

            fn mut_metadata<F, R>(f: F) -> R where F: FnOnce(&mut EventMetadata<Self>) -> R {
                f(&mut *#ident.write().unwrap())
            }
        }
    }
}

/// Handle the `#[event_handler]` attribute.
///
/// This has actually been already handled by the build script hack, so
/// the macro itself is a no-op.
#[proc_macro_attribute]
pub fn event_handler(_metadata: TokenStream, input: TokenStream) -> TokenStream { input }

/// Handle the `#[command_handler]` attribute.
///
/// This has actually been already handled by the build script hack, so
/// the macro itself is a no-op.
#[proc_macro_attribute]
pub fn command_handler(_metadata: TokenStream, input: TokenStream) -> TokenStream { input }

/// Generates code to import the generated function.
///
/// The build script hack exfiltrates the path to the generated file in
/// an environment variable, which we use here to `include!` the file. I
/// tried to do it using `#[path="..."] mod ...;` but that doesn't appear
/// to work with a macro generating the path (cf. RFC issue 1516, issue 48250).
///
/// (This could be a `macro_rules!` macro, but I already needed the proc
/// macro crate for `#[register]`, so here we are.)
#[proc_macro]
pub fn macbuild(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        include!(env!("PESGENBUILD"));
    };

    expanded.into()
}
