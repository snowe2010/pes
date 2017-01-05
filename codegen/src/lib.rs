#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(custom_attribute)]
#![feature(quote, concat_idents, plugin_registrar, rustc_private, unicode)]
#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#[macro_use]
extern crate lazy_static;

#[macro_use] extern crate syntax;
#[macro_use] extern crate log;
extern crate syntax_ext;
extern crate rustc;
extern crate rustc_plugin;
extern crate CQRuSt;

#[macro_use] mod utils;
mod parser;

use std::env;
use std::mem::transmute;

use syntax::ptr::P;
use syntax::ast::{Item};
use syntax::ast::{ItemKind, Expr, MetaItem, Mutability, VariantData, Ident};
use syntax::codemap::{Span, Spanned};
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ext::base::SyntaxExtension;
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::symbol::Symbol;

use utils::{emit_item, span, sep_by_tok, option_as_expr, strip_ty_lifetimes};
use utils::SpanExt;
use parser::{Function, Method, Param, RouteParams};
use utils::{IdentExt, ArgExt};

use rustc_plugin::Registry;

const DEBUG_ENV_VAR: &'static str = "ROCKET_CODEGEN_DEBUG";

const PARAM_PREFIX: &'static str = "rocket_param_";
const ROUTE_STRUCT_PREFIX: &'static str = "static_rocket_route_info_for_";
const CATCH_STRUCT_PREFIX: &'static str = "static_rocket_catch_info_for_";
const ROUTE_FN_PREFIX: &'static str = "rocket_route_fn_";
const CATCH_FN_PREFIX: &'static str = "rocket_catch_fn_";

static ONLY_STRUCTS_ERR: &'static str = "`FromForm` can only be derived for \
    structures with named fields.";
static PRIVATE_LIFETIME: &'static str = "'rocket";

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(Symbol::intern("CQRuSt"), MultiDecorator(Box::new(command_handler)));
}

pub fn command_handler(ecx: &mut ExtCtxt, sp: Span, meta_item: &MetaItem,
                       annotated: &Annotatable, push: &mut FnMut(Annotatable)) {
    let i_sp = meta_item.span.shorten_to(stringify!(Get).len());
    println!("isp : {:#?}", i_sp);
    let method = Some(span(Method::Get, i_sp));
    println!("method : {:#?}", method);
    let function = Function::from(annotated).unwrap_or_else(|item_sp| {
        println!("oh no");
        ecx.span_err(sp, "this attribute can only be used on functions...");
        ecx.span_fatal(item_sp, "...but was applied to the item above.");
    });
    println!("meta_item: {:#?}", meta_item);
    println!("meta_item_list: {:#?}", meta_item.meta_item_list());


    // Check that there are no meta items, i.e. CQRuSt(something here) or even CQRuSt()
    match meta_item.meta_item_list() {
        Some(it) => {
            ecx.struct_span_err(sp, "incorrect use of attribute")
                .help("attributes in CQRuSt must have the form: #[name(...)]")
                .emit();
            ecx.span_fatal(sp, "malformed attribute");
        },
        None => {}
    }

    let route = RouteParams::from(ecx, sp, meta_item, annotated);

    let user_fn_name = route.annotated_fn.ident();
    let route_fn_name = user_fn_name.prepend("CQRS_").prepend(ROUTE_FN_PREFIX);

    emit_item(push, quote_item!(ecx,
        fn $route_fn_name<'_b>() {
            println!("YAYAYAYAY");
        }
    ).unwrap());

    let struct_name = user_fn_name.prepend("CQRS_").prepend(ROUTE_STRUCT_PREFIX);

    emit_item(push, quote_item!(ecx,
        #[allow(non_upper_case_globals)]
        pub static $struct_name: CQRuSt::CommandGatewayHandlerInfo =
            CQRuSt::CommandGatewayHandlerInfo {
                handler: $route_fn_name,
            };
    ).unwrap());
    println!("function : {:#?}", function);
    //    return item;
}


lazy_static! {
    pub static ref COMMANDGATEWAY: CommandGateway = CommandGateway::new();
}

pub struct CommandGateway {
    handlers: Vec<i32>
}

impl CommandGateway {
    pub fn new() -> CommandGateway {
        CommandGateway {
            handlers: Vec::new()
        }
    }
}
