use serde_json::value::Value;
use surf::Client;
use surf_cookie_middleware::CookieMiddleware;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn in_memory() {
    let client = Client::new().with(CookieMiddleware::new());

    client
        .get("https://httpbin.org/cookies/set/USER_ID/10")
        .await.unwrap();

    let cookies: Value = client
        .get("https://httpbin.org/cookies")
        .recv_json()
        .await.unwrap();

    assert_eq!(
        cookies.get("cookies").unwrap().get("USER_ID").unwrap(),
        "10"
    );
}
