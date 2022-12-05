use tauri::{command, AppHandle, Runtime, Window};

use detail::{HttpClientBuilder, Method, RequestOptions, Result, RichResponse};

#[command]
pub(crate) async fn invoke<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    url: String,
    method: Method,
    options: RequestOptions,
) -> Result<RichResponse> {
    let client = HttpClientBuilder::new().build()?.build(url, method);
    let req = client.inject(options);
    let res = req.send().await?;
    let info = res.response_info().await?;
    Ok(info)
}

mod detail {
    use chrono::Local;
    use reqwest::{Client, ClientBuilder, RequestBuilder, Response};
    use serde::{Deserialize, Serialize, Serializer};
    use serde_json::Value;
    use std::collections::HashMap;
    use std::time::Duration;

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error(transparent)]
        ReqError(#[from] reqwest::Error),
    }

    impl Serialize for Error {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.to_string().as_ref())
        }
    }
    #[derive(Debug)]
    pub struct HttpClientBuilder {
        value: ClientBuilder,
    }

    impl HttpClientBuilder {
        pub fn new() -> Self {
            let client = reqwest::Client::builder().danger_accept_invalid_certs(true);
            Self { value: client }
        }
        pub fn build(self) -> Result<HttpClient> {
            let client = self.value.build()?;
            Ok(HttpClient { value: client })
        }
    }

    #[derive(Deserialize, Debug)]
    pub enum Method {
        Get,
        Post,
        Put,
        Patch,
        Delete,
        Head,
    }

    pub struct HttpClient {
        value: Client,
    }

    impl HttpClient {
        pub fn build(&self, url: String, method: Method) -> HttpRequest {
            match method {
                Method::Get => self.get(url),
                Method::Post => self.post(url),
                Method::Put => self.put(url),
                Method::Patch => self.patch(url),
                Method::Delete => self.delete(url),
                Method::Head => self.head(url),
            }
        }
        fn get(&self, url: String) -> HttpRequest {
            let request = self.value.get(url);
            HttpRequest { value: request }
        }
        fn post(&self, url: String) -> HttpRequest {
            let request = self.value.post(url);
            HttpRequest { value: request }
        }
        fn put(&self, url: String) -> HttpRequest {
            let request = self.value.put(url);
            HttpRequest { value: request }
        }
        fn patch(&self, url: String) -> HttpRequest {
            let request = self.value.patch(url);
            HttpRequest { value: request }
        }
        fn delete(&self, url: String) -> HttpRequest {
            let request = self.value.delete(url);
            HttpRequest { value: request }
        }
        fn head(&self, url: String) -> HttpRequest {
            let request = self.value.head(url);
            HttpRequest { value: request }
        }
    }

    #[derive(Deserialize, Debug)]
    pub enum ContentType {
        #[serde(rename = "application/json")]
        JSON, // .json T
        #[serde(rename = "multipart/form-data")]
        MULTIPART, // .multipart  reqwest::multipart::Form
        #[serde(rename = "application/x-www-form-urlencoded")]
        FORM, // .form  HashMap
        #[serde(rename = "text/html")]
        HTML, // 以下全部 .body string
        #[serde(rename = "text/plain")]
        TEXT,
        #[serde(rename = "application/xml")]
        XML,
    }

    #[derive(Deserialize, Debug)]
    pub struct RequestOptions {
        headers: Option<Vec<(String, String)>>,
        content_type: Option<ContentType>,
        basic_auth: Option<(String, String)>,
        bearer_auth: Option<String>,
        query: Option<Vec<(String, String)>>, // any
        timeout: Option<Duration>,
        body: Option<String>,                // ContentType::other
        json: Option<String>,                // ContentType::JSON
        form: Option<Vec<(String, String)>>, // ContentType::MULTIPART/ContentType::FORM
    }
    pub struct HttpRequest {
        value: RequestBuilder,
    }

    impl HttpRequest {
        pub async fn send(self) -> Result<HttpResponse> {
            let start_mills: i64 = Local::now().timestamp_millis();
            let res = self.value.send().await?;
            let end_mills: i64 = Local::now().timestamp_millis();
            Ok(HttpResponse {
                res,
                interval: end_mills - start_mills,
            })
        }
        pub fn inject(mut self, options: RequestOptions) -> Self {
            self.value = match options.query {
                Some(v) => self.value.query(&v),
                None => self.value,
            };
            self.value = match options.basic_auth {
                Some(v) => self.value.basic_auth(v.0, Some(v.1)),
                None => self.value,
            };
            self.value = match options.bearer_auth {
                Some(token) => self.value.bearer_auth(token),
                None => self.value,
            };
            self.value = match options.timeout {
                Some(timeout) => self.value.timeout(timeout),
                None => self.value,
            };
            self.value = match options.content_type {
                Some(ContentType::FORM) => match options.form {
                    Some(form) => {
                        let mut form_map = HashMap::new();
                        for (_, (key, value)) in form.iter().enumerate() {
                            form_map.insert(key.clone(), value.clone());
                        }
                        self.value.form(&form)
                    }
                    None => self.value,
                },
                Some(ContentType::JSON) => match options.json {
                    Some(json) => self
                        .value
                        .json::<Value>(&serde_json::from_str(&json).unwrap()),
                    None => self.value,
                },
                Some(ContentType::MULTIPART) => match options.form {
                    Some(form) => {
                        let mut form_map = reqwest::multipart::Form::new();
                        for (_, (key, value)) in form.iter().enumerate() {
                            form_map = form_map.text(key.clone(), value.clone());
                        }
                        self.value.form(&form)
                    }
                    None => self.value,
                },
                _ => match options.body {
                    Some(body) => self.value.body(body),
                    None => self.value,
                },
            };
            self.value = match options.headers {
                Some(v) => {
                    for (_, value) in v.iter().enumerate() {
                        let (key, value) = value;
                        self.value = self.value.header(key, value);
                    }
                    self.value
                }
                None => self.value,
            };
            self
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct RichResponse {
        pub headers: HashMap<String, String>,
        pub status: String,
        pub content_type: String,
        pub body: String,
        pub interval: i64,
        pub size: usize,
    }

    pub struct HttpResponse {
        res: Response,
        interval: i64,
    }

    impl HttpResponse {
        fn headers(&self) -> HashMap<String, String> {
            let header_map = self.res.headers();
            let mut headers = HashMap::new();
            for (key, value) in header_map.into_iter() {
                headers.insert(key.to_string(), value.to_str().unwrap().to_string());
            }
            headers
        }
        fn status(&self) -> String {
            self.res.status().as_str().to_string()
        }
        pub async fn response_info(self) -> Result<RichResponse> {
            let headers = self.headers();
            let status = self.status();
            let content_type = headers.get("content-type").unwrap().clone();
            let body = self.res.text().await?;
            let size = body.clone().chars().count();
            Ok(RichResponse {
                headers,
                status,
                content_type,
                body,
                interval: self.interval,
                size,
            })
        }
    }
}
