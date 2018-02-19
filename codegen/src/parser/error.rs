use syntax::ast::*;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::codemap::{Span, Spanned, dummy_spanned};

use utils::{span, MetaItemExt};
use super::Function;

/// This structure represents the parsed `error` attribute.
pub struct ErrorParams {
    pub annotated_fn: Function,
//    pub code: Spanned<u16>,
}

impl ErrorParams {
    /// Parses the route attribute from the given decorator context. If the
    /// parse is not successful, this function exits early with the appropriate
    /// error message to the user.
    pub fn from(ecx: &mut ExtCtxt,
                sp: Span,
                meta_item: &MetaItem,
                annotated: &Annotatable)
                -> ErrorParams {
        let function = Function::from(annotated).unwrap_or_else(|item_sp| {
            ecx.span_err(sp, "this attribute can only be used on functions...");
            ecx.span_fatal(item_sp, "...but was applied to the item above.");
        });

        ErrorParams {
            annotated_fn: function,
//            code: parse_code(ecx, &meta_items[0])
        }
    }
}
