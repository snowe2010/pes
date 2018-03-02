#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
#[macro_use]
extern crate lazy_static;

use proc_macro2::Span;
use proc_macro::TokenStream;
use quote::ToTokens;
//use std::Result;
use syn::DeriveInput;
use syn::Fields;
use syn::FnArg;
use syn::FnDecl;
use syn::Item;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::Type;

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
    quote! {
        lazy_static! {
            static ref MYEVENT_METADATA: RwLock<CommandMetadata<#name>> = RwLock::new(CommandMetadata::new());
            static ref EVENT_BUS: CommandBus = CommandBus::new();
        }
        impl Command for #name {
            fn event_metadata<F, R>(f: F) -> R where F: FnOnce(&CommandMetadata<Self>) -> R {
                f(&*MYEVENT_METADATA.read().unwrap())
            }

            fn mut_metadata<F, R>(f: F) -> R where F: FnOnce(&mut CommandMetadata<Self>) -> R {
                f(&mut *MYEVENT_METADATA.write().unwrap())
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
//            quote! {
//                fun
//            }
            TokenStream::empty()
            /*
            for input in inputs {
                match *input {
                    FnArg::Captured(ref input) => {

                        match input.ty {
                            Type::Path(ref typePath) => {
                                println!("TypePath");
                                let tok  = typePath.path.into_tokens();
                                let qu = quote! {
                                    #tok
                                };
                                return TokenStream::from(qu)
                            }
                            _ => {
                                println!("anything else");
                                return TokenStream::empty();
                            }
                        }

//                        println!("what?? {}", input.ty);
                    }
                    _ => {
                        println!("somethign else");
                        return TokenStream::empty()

                    }
                }
            }
            */
        }
        _ => {
//            item.span().unstable()
//                .error("This is not a struct")
//                .emit();
            TokenStream::empty()
        }
    }

    // Use `quote` to convert the syntax tree back into tokens so we can return them. Note
    // that the tokens we're returning at this point are still just the input, we've simply
    // converted it between a few different forms.
//    let output = quote! { #item };
//    output.into()
}


// Parses the inputted stream.

//fn parse_input(input: TokenStream) -> Result<DeriveInput> {
//    // Construct a string representation of the type definition
//    let as_string = input.to_string();
//    // Parse the string representation
//    let parsed = syn::parse_derive_input(&as_string)
//        .map_err(|e| ErrorKind::ParseError(e))?;
//    Ok(parsed)
//}

//fn generate_from(ast: &DeriveInput) -> Result<Tokens> {
//    let from_str_block = impl_from_str(ast)?;
//    let variants_block = impl_variants(ast)?;
//
//    Ok(quote! {
//            #from_str_block
//            #variants_block
//        })
//}

// Determine if the struct has a field named "bees"
/*
fn has_bees(struct_: &syn::ItemStruct) -> bool {
    match struct_.fields {
        // A field can only be named "bees" if it has a name, so we'll
        // match those fields and ignore the rest.
        Fields::Named(ref fields) => {
            // Unwrap the field names because we know these are named fields.
            fields.named.iter().any(|field| field.ident.unwrap() == "bees")
        }
        // Ignore unit structs or anonymous fields.
        _ => {
            false
        },
    }
}*/

// Generate fun compiler errors
/*
fn light_it_up(struct_: &syn::ItemStruct) {
    if let Fields::Named(ref fields) = struct_.fields {
        // Piece together our exquisite error message.
        let bees = "🐝 ".repeat(17);
        let msg = "🐝   not the bees!!! NOT THE BEEEEEES!!! 🐝";
        // The `join` method places the provided string between the joined items,
        // so putting empty strings at the beginning and end will put extra
        // newline characters at the beginning and end of the error message.
        let bees_msg = ["", bees.as_str(), msg, bees.as_str(), ""].join("\n");
        // Find the field named "bees".
        for field in &fields.named {
            let ident = field.ident.unwrap();
            if ident == "bees" {
                // Deliver the error message.
                ident.span()//.unstable()
                    .error(bees_msg.clone())
                    .emit();
            }
        }
    }
}
*/
