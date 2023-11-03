use serde::Serialize;
use worker::{js_sys::JSON, wasm_bindgen::JsValue, *};

pub fn post(path: &str, headers: Headers, body: &impl Serialize) -> Result<Request> {
    Request::new_with_init(
        // host is not used
        &format!("http://xee.jp{}", path),
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
    )
}

pub fn get(path: &str) -> Result<Request> {
    // host is not used
    Request::new(&format!("http://xee.jp{}", path), Method::Get)
}

pub fn forward(req: Request, path: &str) -> Result<Request> {
    Request::new_with_init(
        &format!(
            // host is not used
            "http://xee.jp{}{}",
            path,
            req.url()?
                .query()
                .map(|q| format!("?{}", q))
                .unwrap_or_default()
        ),
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

pub fn handle_error<T, E: std::error::Error>(result: std::result::Result<T, E>) -> Result<T> {
    match result {
        Ok(t) => Ok(t),
        Err(e) => {
            console_error!("{}", e);
            Err(Error::RustError(e.to_string()))
        }
    }
}

pub fn assert_some<T>(option: Option<T>) -> Result<T> {
    match option {
        Some(t) => Ok(t),
        None => Err(Error::RustError(format!(
            "Expected Some for {}",
            std::any::type_name::<T>()
        ))),
    }
}
