use syntax::codemap::{Span, BytePos};

pub trait SpanExt {
    fn shorten_to(self, to_length: usize) -> Span;

    /// Trim the span on the left and right by `length`.
    fn trim(self, length: u32) -> Span;
}

impl SpanExt for Span {
    fn shorten_to(mut self, to_length: usize) -> Span {
        self.hi = self.lo + BytePos(to_length as u32);
        self
    }

    fn trim(mut self, length: u32) -> Span {
        self.lo = self.lo + BytePos(length);
        self.hi = self.hi - BytePos(length);
        self
    }
}
