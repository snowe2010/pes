#![feature(proc_macro)]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;

use syn::Item;
use syn::Fields;
use syn::spanned::Spanned;
use proc_macro::TokenStream;
use proc_macro2::Span;

#[proc_macro_attribute]
pub fn not_the_bees(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the `TokenStream` into a syntax tree, specifically an `Item`. An `Item` is a
    // syntax item that can appear at the module level i.e. a function definition, a struct
    // or enum definition, etc.
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    // Match on the parsed item and respond accordingly.
    match item {
        // If the attribute was applied to a struct, we're going to do
        // some more work to figure out if there's a field named "bees".
        // It's important to take a reference to `struct_item`, otherwise
        // you partially move `item`.
        Item::Struct(ref struct_item) => {
            if has_bees(struct_item) {
                light_it_up(struct_item);
            }
        },

        // If the attribute was applied to any other kind of item, we want
        // to generate a compiler error.
        _ => {
            // This is how you generate a compiler error. You can also
            // generate a "note," or a "warning."
            item.span().unstable()
                .error("This is not a struct")
                .emit();
        },
    }

    // Use `quote` to convert the syntax tree back into tokens so we can return them. Note
    // that the tokens we're returning at this point are still just the input, we've simply
    // converted it between a few different forms.
    let output = quote!{ #item };
    output.into()
}

#[proc_macro_derive(Event)]
pub fn event(input: TokenStream) -> TokenStream {
    // Parse the `TokenStream` into a syntax tree, specifically an `Item`. An `Item` is a
    // syntax item that can appear at the module level i.e. a function definition, a struct
    // or enum definition, etc.
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    // Match on the parsed item and respond accordingly.
    match item {
        // If the attribute was applied to a struct, we're going to do
        // some more work to figure out if there's a field named "bees".
        // It's important to take a reference to `struct_item`, otherwise
        // you partially move `item`.
        Item::Struct(ref struct_item) => {
            let output = quote! {
                impl struct_item.struct_token.into_tokens() {
                  fn test() {

                  }
                }
            };
            output.into()
        },
        // If the attribute was applied to any other kind of item, we want
        // to generate a compiler error.
        _ => {
            // This is how you generate a compiler error. You can also
            // generate a "note," or a "warning."
            item.span().unstable()
                .error("This is not a struct")
                .emit();
            TokenStream::empty()
        },
    }
}
/// Determine if the struct has a field named "bees"
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
}

/// Generate fun compiler errors
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
                ident.span().unstable()
                    .error(bees_msg.clone())
                    .emit();
            }
        }
    }
}
