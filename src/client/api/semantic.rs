use rustc_serialize::json::Json;
use rustc_serialize::Encodable;

use client::{WeChatClient, WeChatResult};

#[derive(Debug, Clone)]
pub struct WeChatSemantic<'a> {
    client: &'a WeChatClient,
}

impl<'a> WeChatSemantic<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatSemantic<'a> {
        WeChatSemantic {
            client: client,
        }
    }

    pub fn search_simple(&self, query: &str, category: &str) -> WeChatResult<Json> {
        let body = json!({
            "query": (query),
            "category": (category),
            "appid": (self.client.appid)
        });
        self.search(&body)
    }

    pub fn search<D: Encodable>(&self, data: &D) -> WeChatResult<Json> {
        self.client.post("https://api.weixin.qq.com/semantic/semproxy/search", vec![], data)
    }
}