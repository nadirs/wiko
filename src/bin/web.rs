extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::middleware::AfterMiddleware;

fn index(_request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Welcome to Wiko!")))
}

#[derive(Default,Debug)]
struct WikoLogger {}

impl AfterMiddleware for WikoLogger {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        println!("{} {} {}", req.method, req.url, res.status.unwrap());
        Ok(res)
    }
}

fn main() {
    let mut chain = Chain::new(index);

    chain.link_after(WikoLogger::default());

    let server = Iron::new(chain).http("localhost:3000").expect("Error starting iron");
    println!("{:?}", server);
}
