//use std::fmt;
//
//use codegen::CommandGatewayHandlerInfo;
//use handler::Handler;
//
//use term_painter::ToStyle;
//use term_painter::Color::*;
//
///// A route: a method, its handler, path, rank, and format/content type.
//#[derive(Clone)]
//pub struct Route {
//    /// A function that should be called when the route matches.
//    pub handler: Handler,
//    pub name: String,
//}
//
//impl Route {
//    /// Creates a new route with the handler.
//    pub fn new(handler: Handler, name: String) -> Route {
//        Route {
//            handler: handler,
//            name: name,
//        }
//    }
//
//}
////
////impl Clone for Route {
////    fn clone(&self) -> Route {
////        Route {
////            handler: self.handler,
////            name: self.name,
////        }
////    }
////}
//
//
//#[doc(hidden)]
//impl<'a> From<&'a CommandGatewayHandlerInfo> for Route {
//    fn from(info: &'a CommandGatewayHandlerInfo) -> Route {
//        let mut route = Route::new(info.handler, info.name.clone());
//        route
//    }
//}
//
//impl fmt::Debug for Route {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        <Route as fmt::Display>::fmt(self, f)
//    }
//}
//
//impl fmt::Display for Route {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", Green.paint(&self.name))
//
////        if self.rank > 1 {
////            write!(f, " [{}]", White.paint(&self.rank))?;
////        }
////
////        if !self.content_type.is_any() {
////            write!(f, " {}", Yellow.paint(&self.content_type))
////        } else {
////            Ok(())
////        }
//    }
//}
