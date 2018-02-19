
use router::{Router, Route};

pub struct Crust {
    pub router: Router,
}

impl Crust {
    pub fn new() -> Crust {
        Crust {
            router: Router::new()
        }
    }

    pub fn send(&self, name: &String) {
        println!("Inside crust!");
        println!("Find matches for which function to call");
        let matches = self.router.route(name);

        println!("For each match, call the handler!");
        for route in matches {
            // Retrieve and set the requests parameters.
//            info_!("Matched: {}", route);
            println!("Match #");
            // FIXME: Users should not be able to use this.
//            request.set_params(route);

            // Dispatch the request to the handler.
//            let outcome = (route.handler)(request, data);

            // Check if the request processing completed or if the request needs
            // to be forwarded. If it does, continue the loop to try again.
//            info_!("{} {}", White.paint("Outcome:"), outcome);
//            match outcome {
//                o@Outcome::Success(_) | o @Outcome::Failure(_) => return o,
//                Outcome::Forward(unused_data) => data = unused_data,
//            };
        }
    }
}
