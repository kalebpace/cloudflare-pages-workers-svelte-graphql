use worker::*;

use serde_json::json;

use js_sys::Date as JsDate;

mod utils;
mod algos;

fn get_request_info(req: &Request) -> String {
    let date_str = Date::now().to_string();
    let path = req.path();
    let coords = req.cf().coordinates().unwrap_or_default();
    let region = req.cf().region().unwrap_or("unknown region".into());
    format!("{} - [{}], located at: {:?}, within: {}", date_str, path, coords, region)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new(());

    router
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get("/echo-request-info", |req, _| {
            let request_info = get_request_info(&req);
            Response::ok(request_info)
        })
        .get("/calc/fibonacci/:number", |_, ctx| {
            if let Some(number) = ctx.param("number") {
                let start = JsDate::now();
                let n = number.parse::<u32>().unwrap();
                let result = algos::recurse_fibonacci(n);
                let duration = JsDate::now() - start;

                let response = &json!({
                    "fibonacci_result": result,
                    "execution_duration": duration
                });
                console_log!("{}", response);

                Response::from_json(response)
            } else {
                Response::error("Bad Request", 400)
            }
        })
        .run(req, env)
        .await
}
