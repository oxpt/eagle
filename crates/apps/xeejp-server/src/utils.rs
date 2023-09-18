use serde::Serialize;
use worker::{js_sys::JSON, wasm_bindgen::JsValue, *};

pub fn post(path: &str, headers: Headers, body: &impl Serialize) -> Result<Request> {
    let req = Request::new_with_init(
        &format!("http://example.com{}", path),
        &RequestInit {
            body: Some(
                JSON::stringify(&serde_wasm_bindgen::to_value(body).unwrap())
                    .unwrap()
                    .into(),
            ),
            headers,
            method: Method::Post,
            ..Default::default()
        },
    );
    req
}

pub fn get(path: &str) -> Result<Request> {
    Request::new(&format!("http://example.com{}", path), Method::Get)
}

pub fn forward(req: Request, path: &str) -> Result<Request> {
    Request::new_with_init(
        &format!("http://example.com{}", path),
        &RequestInit {
            method: req.method(),
            headers: req.headers().clone(),
            body: inner_body(req),
            ..Default::default()
        },
    )
}

pub fn inner_body(req: Request) -> Option<JsValue> {
    req.inner().body().map(|b| b.into())
}
