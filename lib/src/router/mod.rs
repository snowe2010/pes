mod route;

pub use self::route::Route;
use std::collections::HashMap;

#[derive(Default)]
pub struct Router {
    pub routes: HashMap<String, Vec<Route>>,
    // using 'selector' for now
}

impl Router {
    pub fn new() -> Router {
        Router { routes: HashMap::new() }
    }

    pub fn add(&mut self, route: Route) {
        let selector = route.name.clone();
        self.routes.entry(selector).or_insert_with(|| vec![]).push(route);
    }

    pub fn route<'b>(&'b self, req: &String) -> Vec<&'b Route> {
        self.routes.get(req)
            .map_or(vec![], |routes| {
            let mut matches: Vec<_> = routes.iter()
//                .filter(|r| r.collides_with(req))
                .collect();

            matches
        })
    }

//    pub fn has_collisions(&self) -> bool {
//        let mut result = false;
//        for routes in self.routes.values() {
//            for (i, a_route) in routes.iter().enumerate() {
//                for b_route in routes.iter().skip(i + 1) {
//                    if a_route.collides_with(b_route) {
//                        result = true;
//                        warn!("{} and {} collide!", a_route, b_route);
//                    }
//                }
//            }
//        }
//
//        result
//    }
}
