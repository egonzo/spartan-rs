use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::client::Client;
use crate::Result;

pub(crate) const PATH_LOGIN:&str = "/api/v3/user/login";
pub (crate) const PATH_CAMERAS_ALL: &str = "/api/v3/camera/all";


#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub(crate) struct Login {
    username:String,
    password:String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
struct LoginResponse {
    uuid:String,
    token:String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
struct AllCamerasResponse {
}

/// Login logs in to the api. If successful it sets the auth token and uuid on the client.
pub(crate) async fn login(client: &Client, login: Login) -> Result<()> {
    let result:LoginResponse = client.send_request(&login, Method::POST, PATH_LOGIN, false).await?;

    client.set_auth(result.token);
    client.set_uuid(result.uuid);

    Ok(())
}

pub(crate) async fn cameras(client: &Client) -> Result<AllCamerasResponse> {
   let result:AllCamerasResponse = client.get_request(PATH_CAMERAS_ALL, true).await?;

   Ok(result)
}

#[cfg(test)]
mod tests {
    use std::fmt::format;
    use httpmock::prelude::*;
    use crate::{client, spypoint};
    use crate::client::Server;
    use crate::spypoint::{Login, LoginResponse, PATH_CAMERAS_ALL, PATH_LOGIN};


    #[test]
    fn login() {
        let mock_server = MockServer::start();
        let url = format!("http://{}", mock_server.address());

        let resp = LoginResponse {
            uuid:String::from("7777777777777AA"),
            token:String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw")
        };

        let login_mock = mock_server.mock(|when, then| {
            when.method(POST).path(PATH_LOGIN);
            then.status(200).body(LOGIN_RESPONSE);
        });

        let server = Server{
           user_name:String::from("ed"),
            password:String::from("money"),
            host:url,
        };

        let client = client::Client::new(server).expect("spypoint client");

        let l = Login{
            username: client.user(),
            password: client.user_password()
        };

        tokio_test::block_on(async {
            let result = spypoint::login(&client,l).await;

            login_mock.assert();

            assert!(!result.is_err());
        });
    }

    fn all_cameras() {
        let mock_server = MockServer::start();
        let url = format!("http://{}", mock_server.address());

        let resp = LoginResponse {
            uuid:String::from("7777777777777AA"),
            token:String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw")
        };

        let auth = format!("Bearer {}", resp.token);

        let login_mock = mock_server.mock(|when, then| {
            when.method(GET).path(PATH_CAMERAS_ALL)
                .header("Authorization", auth);
            then.status(200).body(LOGIN_RESPONSE);
        });

        let server = Server{
            user_name:String::from("ed"),
            password:String::from("money"),
            host:url,
        };

        let client = client::Client::new(server).expect("spypoint client");
        client.set_auth(resp.token.clone());

        tokio_test::block_on(async {
            let result = spypoint::cameras(&client).await;

            login_mock.assert();

            assert!(!result.is_err());
        });
    }

const LOGIN_RESPONSE :&str=r#"{
  "uuid": "5f14230017d3051e",
  "token": "eyJyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw"
}"#;

}

