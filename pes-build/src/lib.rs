#[macro_use] extern crate quote;
extern crate syn;

use syn::{Attribute, Item, ItemFn, ItemMod, Lit, Meta, MetaNameValue,
          ItemImpl, ImplItem, ImplItemMethod, Type, TypePath};
use std::env;
use std::io::{Read, Write};
use std::fs::File;
use std::path::{Path, PathBuf};

/// Look for a simple attribute matching a string
fn any_attr_is(attrs: &[Attribute], ident: &str) -> bool {
    attrs.iter().any(|a| match a.interpret_meta() {
        Some(Meta::Word(i)) if i == ident => true,
        _ => false
    })
}

/// Parse a list of items for #[command_handler]ed functions (recurse into modules)
///
/// mod_path: parent dir of the mod we are parsing
/// items: list of items in the current mod
///
/// Returns a list of item paths (relative to the current module)
fn parse(attribute: &str, mod_path: PathBuf, items: Vec<Item>) -> Vec<syn::Path> {
    let mut names = vec![];

    for item in items {
        match item {
            // handle a registered function
            Item::Fn(ItemFn { ref attrs, ident, .. })
            if any_attr_is(attrs, attribute) => {
                names.push(ident.into());
            }

            // match functions inside impls
            Item::Impl(ItemImpl { items, self_ty, .. }) => {
                for item in items {
                    match item {
                        ImplItem::Method(ImplItemMethod { ref attrs, ref sig, .. })
                        if any_attr_is(attrs, attribute) => {
                            let method_name: syn::PathSegment = sig.ident.into();
                            match *self_ty {
                                Type::Path(TypePath { ref path, .. }) => {
                                    let mut built = path.clone();
                                    built.segments.push(method_name);
                                    names.push(built);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }

            // handle a module
            Item::Mod(module) => {
                let (the_path, the_items, the_ident);

                // what kind of module is it?
                match module {
                    // inline module!
                    ItemMod { content: Some((_, items)), ident, .. } => {
                        the_items = items;
                        the_ident = ident;
                        the_path = mod_path.clone();
                    }
                    // non-inline module!
                    ItemMod { attrs, ident, .. } => {
                        // read the #[path] attr if present
                        let mut path = None;
                        for attr in attrs {
                            match attr.interpret_meta() {
                                Some(Meta::NameValue(MetaNameValue { ident, lit: Lit::Str(ref s), .. }))
                                if ident == "path" => {
                                    path = Some(s.value());
                                }

                                _ => {}
                            }
                        }

                        // read in the module contents from file, wherever it is
                        let mut content = String::new();
                        let mut file = match path {
                            // from a path attribute
                            Some(p) => {
                                the_path = Path::new(&p).parent().unwrap().to_owned();
                                File::open(&p).expect(&p)
                            }

                            // no path attribute -- try $name.rs and $name/mod.rs
                            None => {
                                match File::open(mod_path.join(format!("{}.rs", ident))) {
                                    Ok(file) => {
                                        the_path = mod_path.clone();
                                        file
                                    }
                                    Err(_) => {
                                        the_path = mod_path.join(ident.as_ref());
                                        File::open(mod_path.join(ident.as_ref()).join("mod.rs")).expect(&format!("{}/{}/mod.rs", mod_path.display(), ident))
                                    }
                                }
                            }
                        };
                        file.read_to_string(&mut content).unwrap();
                        the_items = syn::parse_file(&content).unwrap().items;
                        the_ident = ident;
                    }
                }

                // recurse to find registered functions within the new module
                names.extend(
                    parse(attribute, the_path, the_items)
                        .into_iter()
                        .map(|mut p| {
                            // prepend the module path to the found items
                            p.segments.insert(0, the_ident.into());
                            p
                        })
                );
            }

            _ => {}
        }
    }

    names
}

/// Find registered functions in the given crate. Call this in your build script!
///
/// root: path to the crate root (e.g. src/main.rs or src/lib.rs)
pub fn go<P: AsRef<Path>>(root: P) {
    println!("BUILDING ");
    let root = root.as_ref();
    let outfile = Path::new(&env::var("OUT_DIR").unwrap()).join("pes-gen.rs");
    // Exfiltrate the name of the generated file so that macbuild!() can include it
    println!("cargo:rustc-env=PESGENBUILD={}", outfile.display());

    // Get registered functions from the crate
    let mut content = String::new();
    File::open(root).unwrap().read_to_string(&mut content).unwrap();
    let ast = syn::parse_file(&content).unwrap();
    let event_handlers = parse("event_handler", root.parent().unwrap().to_owned(), ast.items.clone());
    let command_handlers = parse("command_handler", root.parent().unwrap().to_owned(), ast.items);

    // Generate bootstrap function
    let mut out = File::create(outfile).unwrap();
    writeln!(out, "{}", quote! {
        pub fn bootstrap() {
            println!("IN THE BOOTSTRAP");
            #(COMMAND_BUS.register(#command_handlers,0);)*
            #(EVENT_BUS.register(#event_handlers,0);)*
        }
    }).unwrap();
}

