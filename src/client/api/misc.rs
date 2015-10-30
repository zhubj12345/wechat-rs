use rustc_serialize::json::Json;

use types::WeChatResult;
use client::WeChatClient;
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatMisc<T: SessionStore> {
    client: WeChatClient<T>,
}

impl<T: SessionStore> WeChatMisc<T> {

    #[inline]
    pub fn new(client: WeChatClient<T>) -> WeChatMisc<T> {
        WeChatMisc {
            client: client,
        }
    }

    pub fn get_wechat_ips(&self) -> WeChatResult<Vec<String>> {
        let data = try!(self.client.get("getcallbackip", vec![]));
        let ip_list = &data["ip_list"];
        let ip_array = ip_list.as_array().unwrap();
        let mut ips: Vec<String> = Vec::new();
        for v in ip_array.iter() {
            if let &Json::String(ref ip) = v {
                ips.push(ip.to_owned());
            }
        }
        Ok(ips)
    }

    pub fn short_url(&self, long_url: &str) -> WeChatResult<String> {
        let body = json!({
            "action": "long2short",
            "long_url": (long_url)
        });
        let data = try!(self.client.post("shorturl", vec![], body.as_object().unwrap()));
        let short = &data["short_url"];
        let short = short.as_string().unwrap();
        Ok(short.to_owned())
    }
}
