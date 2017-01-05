use std::str::FromStr;
use std::collections::HashSet;

use syntax::ast::*;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::codemap::{Span, Spanned, dummy_spanned};

use utils::{IdentExt, ArgExt};
use utils::{span, MetaItemExt, SpanExt, is_valid_ident};
use super::{Function, ParamIter};
use super::keyvalue::KVSpanned;
use super::method::Method;

/// This structure represents the parsed `route` attribute.
///
/// It contains all of the information supplied by the user and the span where
/// the user supplied the information. This structure can only be obtained by
/// calling the `RouteParams::from` function and passing in the entire decorator
/// environment.
#[derive(Debug)]
pub struct RouteParams {
    pub annotated_fn: Function,
}

impl RouteParams {
    /// Parses the route attribute from the given decorator context. If the
    /// parse is not successful, this function exits early with the appropriate
    /// error message to the user.
    pub fn from(ecx: &mut ExtCtxt,
                sp: Span,
                meta_item: &MetaItem,
                annotated: &Annotatable)
                -> RouteParams {
        let function = Function::from(annotated).unwrap_or_else(|item_sp| {
            ecx.span_err(sp, "this attribute can only be used on functions...");
            ecx.span_fatal(item_sp, "...but was applied to the item above.");
        });

        RouteParams {
            annotated_fn: function,
        }
    }
}
