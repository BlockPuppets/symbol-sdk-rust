/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::convert::TryFrom;
use std::fmt::Debug;

use async_trait::async_trait;
use reqwest::Method;

use crate::clients::async_client::request::Request;
use crate::clients::{consts::HTTP_REQUEST_TIMEOUT, Error, SymbolResponse};
use crate::SymbolError;

#[derive(Debug)]
pub struct Response<R> {
    pub result: R,
}

impl<R: for<'de> serde::Deserialize<'de>> TryFrom<SymbolResponse> for Response<R> {
    type Error = Error;

    fn try_from(resp: SymbolResponse) -> Result<Self, Error> {
        match resp.result {
            Some(ret) => Ok(Self {
                result: serde_json::from_value::<R>(ret)
                    .map_err(Error::DeserializeResponseJsonError)?,
            }),
            None => Err(Error::ResultNotFound(resp)),
        }
    }
}

impl<R> std::ops::Deref for Response<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

async fn send_json_request<T: for<'de> serde::Deserialize<'de>>(
    client: reqwest::Client,
    url: reqwest::Url,
    method: Method,
) -> Result<T, Error> {
    let resp = client
        .request(method, url)
        .send()
        .await
        .map_err(Error::NetworkError)?;

    if resp.status().as_u16() == 409 {
        let err: SymbolError = resp.json().await.map_err(Error::InvalidHTTPResponse)?;
        return Err(Error::SymbolError(err));
    }
    if !resp.status().is_success() {
        return Err(Error::InvalidHTTPStatus(
            format!("{:#?}", resp),
            resp.status(),
        ));
    }
    resp.json().await.map_err(Error::InvalidHTTPResponse)
}

#[async_trait]
pub trait HttpClient: Sync + Send + 'static {
    async fn single_request(&self, request: &Request) -> Result<SymbolResponse, Error>;
}

pub struct SimpleHttpClient {
    pub http_client: reqwest::Client,
    pub url: reqwest::Url,
}

impl SimpleHttpClient {
    pub fn new<T: reqwest::IntoUrl>(server_url: T) -> Result<Self, reqwest::Error> {
        let reqwest_client = reqwest::ClientBuilder::new()
            // .use_native_tls()
            .timeout(HTTP_REQUEST_TIMEOUT)
            .build()?;
        Ok(Self {
            http_client: reqwest_client,
            url: server_url
                .into_url()
                .expect("Invalid server_url provided to SimpleHttpClient"),
        })
    }
}

#[async_trait]
impl HttpClient for SimpleHttpClient {
    async fn single_request(&self, request: &Request) -> Result<SymbolResponse, Error> {
        let mut uri_str = request.base_path.to_string();

        if !request.path_params.is_empty() {
            request
                .path_params
                .iter()
                .for_each(|(key, val)| uri_str = uri_str.replace(&format!("{{{}}}", key), &val));
        }

        if !request.query_params.is_empty() {
            let mut query_string = ::url::form_urlencoded::Serializer::new("".to_owned());

            request.query_params.iter().for_each(|(key, val)| {
                query_string.append_pair(key, val);
            });

            let query_string_str = query_string.finish();
            if query_string_str != "" {
                uri_str += "?";
                uri_str += &query_string_str;
            }
        }

        let url = self.url.join(&uri_str).unwrap();

        let rpc_resp: SymbolResponse =
            send_json_request(self.http_client.clone(), url, request.method.clone()).await?;
        Ok(rpc_resp)
    }
}
