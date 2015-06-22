use url::Url;
use hyper::{self, Client};
use rustc_serialize::json::{self, Json, Object};

use errors::WeChatError;

pub struct WeChatClient {
    pub appid: String,
    pub secret: String,
    pub access_token: String,
}

impl WeChatClient {
    pub fn new(appid: &str, secret: &str) -> WeChatClient {
        WeChatClient {
            appid: appid.to_owned(),
            secret: secret.to_owned(),
            access_token: "".to_owned(),
        }
    }

    pub fn with_access_token(appid: &str, secret: &str, access_token: &str) -> WeChatClient {
        WeChatClient {
            appid: appid.to_owned(),
            secret: secret.to_owned(),
            access_token: access_token.to_owned(),
        }
    }

    pub fn post(&self, url: &str, params: Vec<(&str, &str)>, data: &Object) -> Result<Json, WeChatError> {
        let mut querys = params.clone();
        let mut http_url = Url::parse(url).unwrap();
        if !self.access_token.is_empty() {
            querys.push(("access_token", &self.access_token));
        }
        http_url.set_query_from_pairs(querys.into_iter());
        let body = match json::encode(data) {
            Ok(text) => text,
            Err(_) => "".to_owned(),
        };
        let mut client = Client::new();
        let mut res = match client.post(http_url).body(&body).send() {
            Ok(_res) => _res,
            Err(_) => { return Err(WeChatError::ClientError { errcode: -1, errmsg: "".to_owned() }); }
        };
        if res.status != hyper::Ok {
            return Err(WeChatError::ClientError { errcode: -2, errmsg: "Request status error".to_owned() })
        }
        let obj = match Json::from_reader(&mut res) {
            Ok(decoded) => { decoded },
            Err(_) => { return Err(WeChatError::ClientError { errcode: -3, errmsg: "Json decode error".to_owned() }); }
        };
        Ok(obj)
    }

    pub fn get(&self, url: &str, params: Vec<(&str, &str)>) -> Result<Json, WeChatError> {
        let mut querys = params.clone();
        let mut http_url = Url::parse(url).unwrap();
        if !self.access_token.is_empty() {
            querys.push(("access_token", &self.access_token));
        }
        http_url.set_query_from_pairs(querys.into_iter());
        let mut client = Client::new();
        let mut res = match client.post(http_url).send() {
            Ok(_res) => _res,
            Err(_) => { return Err(WeChatError::ClientError { errcode: -1, errmsg: "".to_owned() }); }
        };
        if res.status != hyper::Ok {
            return Err(WeChatError::ClientError { errcode: -2, errmsg: "Request status error".to_owned() })
        }
        let obj = match Json::from_reader(&mut res) {
            Ok(decoded) => { decoded },
            Err(_) => { return Err(WeChatError::ClientError { errcode: -3, errmsg: "Json decode error".to_owned() }); }
        };
        Ok(obj)
    }
}