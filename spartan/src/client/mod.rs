use crate::Result;
use reqwest::{ClientBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, fmt};
use std::fmt::Debug;
use reqwest::Method;
use serde::de::DeserializeOwned;
use log::debug;

pub const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:126.0) Gecko/20100101";

/// The base error that is returned from the API calls.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApiError {
    #[serde(default)]
    http_status: u16,
    error: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "http_status {}, error {}",
            self.http_status, self.error
        ))
    }
}

impl std::error::Error for ApiError {}

pub struct Server {
    pub user_name: String,
    pub password: String,
    pub host: String,
}

impl Server {
    pub fn new(user_name: String, password: String, host: String) -> Self {
        Self { user_name, password, host }
    }

    pub fn from_env() -> Result<Self> {
        let _ = dotenvy::dotenv(); // Ignoring error - it's ok to not have .env files
        Ok(Self {
            user_name: env::var("SPYPOINT_USER")?,
            password: env::var("SPYPOINT_PWD")?,
            host: env::var("SPYPOINT_HOST")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<Mutex<ClientInner>>,
}

#[derive(Debug)]
struct ClientInner {
    server: String,
    user: String,
    pwd: String,
    auth_token: String,
    uuid: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(server: Server) -> Result<Self> {
        let user_agent =  String::from(USER_AGENT);

        let http = ClientBuilder::new()
            .connect_timeout(Duration::new(15, 0))
            .read_timeout(Duration::new(30, 0))
            .timeout(Duration::new(45, 0))
            .user_agent(user_agent)
            .build()?;

        let inner = Arc::new(Mutex::new(ClientInner {
            server: server.host,
            http_client: http,
            user: server.user_name,
            pwd:server.password,
            auth_token:String::new(),
            uuid:String::new(),
        }));

        Ok(Self { inner })
    }

    pub fn user(&self) -> String {
        let lock = self.inner.lock().unwrap();
        lock.user.clone()
    }

    pub fn user_password(&self) -> String {
        let lock = self.inner.lock().unwrap();
        lock.pwd.clone()
    }

    pub fn server(&self) -> String {
        let lock = self.inner.lock().unwrap();
        lock.server.clone()
    }

    pub fn set_uuid(&self, uid:String) {
        let mut lock = self.inner.lock().unwrap();
        lock.uuid.clone_from(&uid);
    }

    pub fn uuid(&self) -> String {
        let lock = self.inner.lock().unwrap();
        lock.uuid.clone()
    }

    pub fn set_auth(&self, token:String) {
        let mut lock = self.inner.lock().unwrap();
        lock.auth_token.clone_from(&token);
    }

    pub fn auth_token(&self) -> String {
        let lock = self.inner.lock().unwrap();
        lock.auth_token.clone()
    }

    pub fn http_client(&self) -> reqwest::Client {
        let lock = self.inner.lock().unwrap();
        lock.http_client.clone()
    }

    pub async fn retrieve_error(&self, resp: Response) -> Box<dyn std::error::Error> {
        let code = resp.status().as_u16();

        let mut err = match resp.json::<ApiError>().await {
            Ok(x) => x,
            Err(e) => return Box::from(format!("error parsing error response, {}", e)),
        };

        err.http_status = code;
        err.into()
    }

    pub async fn get_request<P: DeserializeOwned + Debug>(&self, path:&str, include_auth:bool) -> Result<P>{
        let url = format!("{}{}", self.server(), path);

        let mut builder = self
            .http_client()
            .request(Method::GET, url);

        if include_auth {
            builder = builder.bearer_auth(self.auth_token());
        }

        let result=builder
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(self.retrieve_error(result).await);
        }

        debug!("client got response: {:?}", result);

        let response = result.json::<P>().await?;

        Ok(response)
    }

    pub async fn send_request<R: Serialize + Debug, P: DeserializeOwned + Debug>(
        &self,
        req: &R,
        method:Method,
        path: &str,
        include_auth: bool,
    ) -> Result<P> {
        let url = format!("{}{}", self.server(), path);

        debug!("client request: {:?}", req);

        let mut builder = self
            .http_client()
            .request(method, url);

        if include_auth {
            builder = builder.bearer_auth(self.auth_token());
        }

        let result=builder
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(self.retrieve_error(result).await);
        }

        debug!("client got response: {:?}", result);

        let response = result.json::<P>().await?;

        Ok(response)
    }
}
