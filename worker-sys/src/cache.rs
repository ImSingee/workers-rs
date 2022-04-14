use wasm_bindgen::prelude::*;

use crate::Request;
use crate::Response;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends =::js_sys::Object, js_name = Cache, typescript_type = "Cache")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Cache;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = put)]
    pub fn put_with_request(
        this: &Cache,
        request: &Request,
        response: &Response,
    ) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = put)]
    pub fn put_with_str(this: &Cache, request: &str, response: &Response) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = match)]
    pub fn match_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = match)]
    pub fn match_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = match)]
    pub fn match_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = match)]
    pub fn match_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = delete)]
    pub fn delete_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = delete)]
    pub fn delete_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = delete)]
    pub fn delete_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;

    #[wasm_bindgen(method, structural, js_class = "Cache", js_name = delete)]
    pub fn delete_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends =::js_sys::Object, js_name = CacheQueryOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type CacheQueryOptions;
}

impl CacheQueryOptions {
    pub fn new() -> Self {
        #[allow(unused_mut)]
            let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }

    pub fn ignore_method(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ignoreMethod"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}

impl Default for CacheQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends =::js_sys::Object, js_name = CacheStorage, typescript_type = "CacheStorage")]
    pub type CacheStorage;

    #[wasm_bindgen(structural, method, getter, js_class = "CacheStorage", js_name = default)]
    pub fn default(this: &CacheStorage) -> Cache;

    #[wasm_bindgen(method, structural, js_class = "CacheStorage", js_name = open)]
    pub fn open(this: &CacheStorage, cache_name: &str) -> ::js_sys::Promise;
}
