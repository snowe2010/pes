use codegen::CommandGatewayHandlerInfo;

/// A route: a method, its handler, path, rank, and format/content type.
pub struct Route {
    /// A function that should be called when the route matches.
    pub handler: Handler,
}

impl Route {
    /// Creates a new route with the handler.
    pub fn new<S>(handler: Handler) -> Route {
        Route {
            handler: handler,
        }
    }

}

#[doc(hidden)]
impl<'a> From<&'a CommandGatewayHandlerInfo> for Route {
    fn from(info: &'a CommandGatewayHandlerInfo) -> Route {
        let mut route = Route::new(info.handler);
        route
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Route as fmt::Display>::fmt(self, f)
    }
}
