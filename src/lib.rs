use rand::{Rng, seq::SliceRandom};
use serde_json::json;
use worker::*;

mod compliments {
    use const_format::str_split;
    pub const LOW_PRIORTY: &[&'static str] = &str_split!(include_str!("../compliments/p-low"), "\n");
    pub const HIGH_PRIORTY: &[&'static str] = &str_split!(include_str!("../compliments/p-high"), "\n");
}

use compliments::{HIGH_PRIORTY, LOW_PRIORTY};

pub fn compliment() -> &'static str {
    let mut rng = rand::thread_rng();
    let uses_low_priority = rng.gen_bool(1.0 / 3.0);
    if uses_low_priority {
        LOW_PRIORTY.choose(&mut rng).as_deref().unwrap()
    } else {
        HIGH_PRIORTY.choose(&mut rng).as_deref().unwrap()
    }

}



fn log_request(req: &Request) {
    console_log!(
            "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok(compliment()))
        .run(req, env)
        .await
}