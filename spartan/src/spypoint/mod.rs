use log::debug;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::Result;

pub const PATH_LOGIN: &str = "/api/v3/user/login";
pub const PATH_CAMERAS_ALL: &str = "/api/v3/camera/all";
pub const PATH_CAMERA: &str = "/api/v3/camera/";
pub const PATH_PHOTOS: &str = "/api/v3/photo/all";

// **** Login
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct LoginResponse {
    uuid: String,
    token: String,
}

/// Login logs in to the api. If successful it sets the auth token and uuid on the client.
pub async fn login(client: &Client, login: Login) -> Result<()> {
    let result: LoginResponse = client
        .send_request(&login, Method::POST, PATH_LOGIN, false)
        .await?;

    client.set_auth(result.token);
    client.set_uuid(result.uuid);

    Ok(())
}

// ***** Cameras
pub type Cameras = Vec<Camera>;

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Camera {
    #[serde(rename = "activationDate")]
    pub activation_date: String,

    #[serde(rename = "config")]
    pub config: Config,

    #[serde(rename = "hdSince")]
    pub hd_since: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "status")]
    pub status: Status,

    #[serde(rename = "ucid")]
    pub ucid: String,

    #[serde(rename = "user")]
    pub user: String,

    #[serde(rename = "isCellular")]
    pub is_cellular: bool,

    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<Subscription>,

    #[serde(rename = "dataMatrixKey")]
    pub data_matrix_key: String,

    #[serde(rename = "ptpNotifications")]
    pub ptp_notifications: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(rename = "batteryType")]
    pub battery_type: String,

    #[serde(rename = "capture")]
    pub capture: bool,

    #[serde(rename = "captureMode")]
    pub capture_mode: String,

    #[serde(rename = "dateFormat")]
    pub date_format: String,

    #[serde(rename = "delay")]
    pub delay: String,

    #[serde(rename = "multiShot")]
    pub multi_shot: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "operationMode")]
    pub operation_mode: String,

    #[serde(rename = "quality")]
    pub quality: String,

    #[serde(rename = "schedule")]
    pub schedule: Vec<Vec<i64>>,

    #[serde(rename = "sensibility")]
    pub sensibility: Sensibility,

    #[serde(rename = "smallPicWidth")]
    pub small_pic_width: i64,

    #[serde(rename = "stamp")]
    pub stamp: bool,

    #[serde(rename = "temperatureUnit")]
    pub temperature_unit: String,

    #[serde(rename = "timeFormat")]
    pub time_format: i64,

    #[serde(rename = "transmitAuto")]
    pub transmit_auto: bool,

    #[serde(rename = "transmitFormat")]
    pub transmit_format: String,

    #[serde(rename = "transmitFreq")]
    pub transmit_freq: i64,

    #[serde(rename = "transmitTime")]
    pub transmit_time: TransmitTime,

    #[serde(rename = "transmitUser")]
    pub transmit_user: bool,

    #[serde(rename = "triggerSpeed")]
    pub trigger_speed: String,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Sensibility {
    #[serde(rename = "high")]
    high: i64,

    #[serde(rename = "level")]
    level: String,

    #[serde(rename = "low")]
    low: i64,

    #[serde(rename = "medium")]
    medium: i64,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TransmitTime {
    #[serde(rename = "hour")]
    hour: i64,

    #[serde(rename = "minute")]
    minute: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    pub date_time: String,
    pub latitude: String,
    pub longitude: String,
    pub position: Position,
    #[serde(rename = "geohash")]
    pub geo_hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<f64>,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Status {
    #[serde(rename = "batteries")]
    pub batteries: Vec<i64>,

    #[serde(rename = "batteryType")]
    pub battery_type: String,

    #[serde(rename = "capability")]
    pub capability: Capability,

    #[serde(rename = "installDate")]
    pub install_date: String,

    #[serde(rename = "lastUpdate")]
    pub last_update: String,

    #[serde(rename = "memory")]
    pub memory: Memory,

    #[serde(rename = "model")]
    pub model: String,

    #[serde(rename = "modemFirmware")]
    pub modem_firmware: String,

    #[serde(rename = "notifications")]
    pub notifications: Vec<Option<serde_json::Value>>,

    #[serde(rename = "serial")]
    pub serial: i64,

    #[serde(rename = "signal")]
    pub signal: Signal,

    #[serde(rename = "sim")]
    pub sim: String,

    #[serde(rename = "temperature")]
    pub temperature: Temperature,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "coordinates")]
    pub coordinates: Vec<Coordinate>,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Capability {
    #[serde(rename = "hdRequest")]
    hd_request: bool,

    #[serde(rename = "survivalMode")]
    survival_mode: bool,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Memory {
    #[serde(rename = "size")]
    pub size: i64,

    #[serde(rename = "used")]
    pub used: i64,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Signal {
    #[serde(rename = "bar")]
    pub bar: i64,

    #[serde(rename = "dBm")]
    d_bm: i64,

    #[serde(rename = "mcc")]
    mcc: i64,

    #[serde(rename = "mnc")]
    mnc: i64,

    #[serde(rename = "type")]
    signal_type: String,

    #[serde(rename = "processed")]
    pub processed: Processed,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Processed {
    #[serde(rename = "percentage")]
    percentage: i64,

    #[serde(rename = "bar")]
    pub bar: i64,

    #[serde(rename = "lowSignal")]
    low_signal: bool,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Temperature {
    #[serde(rename = "value")]
    pub value: i64,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Subscription {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "cameraId")]
    camera_id: String,

    #[serde(rename = "paymentStatus")]
    pub(crate) payment_status: String,

    #[serde(rename = "isActive")]
    is_active: bool,

    #[serde(rename = "plan")]
    plan: Plan,

    #[serde(rename = "currency")]
    currency: String,

    #[serde(rename = "paymentFrequency")]
    payment_frequency: String,

    #[serde(rename = "isFree")]
    is_free: bool,

    #[serde(rename = "startDateBillingCycle")]
    start_date_billing_cycle: String,

    #[serde(rename = "endDateBillingCycle")]
    end_date_billing_cycle: String,

    #[serde(rename = "monthEndBillingCycle")]
    month_end_billing_cycle: String,

    #[serde(rename = "photoCount")]
    pub photo_count: i64,

    #[serde(rename = "isAutoRenew")]
    is_auto_renew: bool,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Plan {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "isActive")]
    is_active: bool,

    #[serde(rename = "isFree")]
    is_free: bool,

    #[serde(rename = "isSelectable")]
    is_selectable: bool,

    #[serde(rename = "photoCountPerMonth")]
    photo_count_per_month: i64,

    #[serde(rename = "pricePerMonthIfPaidPerMonth")]
    price_per_month_if_paid_per_month: i64,

    #[serde(rename = "pricePerMonthIfPaidAnnually")]
    price_per_month_if_paid_annually: i64,

    #[serde(rename = "pricePerYear")]
    price_per_year: i64,

    #[serde(rename = "pricePerMonthIfPaidAnnuallyInsidersClub")]
    price_per_month_if_paid_annually_insiders_club: i64,

    #[serde(rename = "pricePerMonthIfPaidPerMonthInsidersClub")]
    price_per_month_if_paid_per_month_insiders_club: i64,

    #[serde(rename = "pricePerYearInsidersClub")]
    price_per_year_insiders_club: i64,

    #[serde(rename = "rebateIfPaidAnnually")]
    rebate_if_paid_annually: i64,

    #[serde(rename = "rebatePercentageInsidersClub")]
    rebate_percentage_insiders_club: i64,

    #[serde(rename = "showBanner")]
    show_banner: String,

    #[serde(rename = "isUpgradable")]
    is_upgradable: bool,

    #[serde(rename = "isDowngradable")]
    is_downgradable: bool,
}

pub async fn cameras(client: &Client) -> Result<Cameras> {
    let result: Cameras = client.get_request(PATH_CAMERAS_ALL, true).await?;

    debug!("spypoint::cameras,result=> \n{:?}\n", result);
    Ok(result)
}

pub async fn camera(client: &Client, camera_id: String) -> Result<Camera> {
    let path = format!("{}{}", PATH_CAMERA, camera_id);

    let result: Camera = client.get_request(path.as_str(), true).await?;

    Ok(result)
}

// ****** photos

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PhotosRequest {
    #[serde(rename = "camera")]
    camera: Vec<String>,

    #[serde(rename = "dateEnd")]
    date_end: String,

    #[serde(rename = "mediaTypes")]
    media_type: Vec<String>,

    #[serde(rename = "species")]
    species: Vec<String>,

    #[serde(rename = "limit")]
    limit: i64,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PhotosResponse {
    #[serde(rename = "photos")]
    pub photos: Vec<Photo>,

    #[serde(rename = "cameraId")]
    pub camera_id: Option<serde_json::Value>,

    #[serde(rename = "cameraIds")]
    pub camera_ids: Vec<String>,

    #[serde(rename = "countPhotos")]
    pub count_photos: i64,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Photo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "tag")]
    pub tag: Vec<String>,
    #[serde(rename = "originName")]
    pub origin_name: String,
    #[serde(rename = "originSize")]
    pub origin_size: i64,
    #[serde(rename = "originDate")]
    pub origin_date: String,
    #[serde(rename = "small")]
    pub small: Hd,
    #[serde(rename = "medium")]
    pub medium: Hd,
    #[serde(rename = "large")]
    pub large: Hd,
    #[serde(rename = "camera")]
    pub camera: String,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Hd {
    #[serde(rename = "verb")]
    pub verb: String,

    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "host")]
    pub host: String,

    #[serde(rename = "headers")]
    pub headers: Vec<Header>,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Header {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "value")]
    pub value: String,
}

/// photos returns list of photo for a camera.
pub async fn camera_photos(
    client: &Client,
    camera_id: String,
    limit: Option<i64>,
) -> Result<PhotosResponse> {
    let req = PhotosRequest {
        camera: vec![camera_id],
        limit: limit.unwrap_or(125),
        date_end: String::from("2100-01-01T00:00:00.000Z"),
        ..Default::default()
    };

    debug!("spypoint::camera_photos, request: {:?}", req);
    let response = client
        .send_request(&req, Method::POST, PATH_PHOTOS, true)
        .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;

    use crate::{client, spypoint};
    use crate::client::Server;
    use crate::spypoint::{
        Login, LoginResponse, PATH_CAMERA, PATH_CAMERAS_ALL, PATH_LOGIN, PATH_PHOTOS,
    };

    #[test]
    fn login() {
        let mock_server = MockServer::start();
        let url = format!("http://{}", mock_server.address());

        let resp = LoginResponse {
            uuid: String::from("7777777777777AA"),
            token: String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw")
        };

        let login_mock = mock_server.mock(|when, then| {
            when.method(POST).path(PATH_LOGIN);
            then.status(200).body(LOGIN_RESPONSE);
        });

        let server = Server {
            user_name: String::from("ed"),
            password: String::from("money"),
            host: url,
        };

        let client = client::Client::new(server).expect("spypoint client");

        let l = Login {
            username: client.user(),
            password: client.user_password(),
        };

        tokio_test::block_on(async {
            let result = spypoint::login(&client, l).await;

            login_mock.assert();

            assert!(!result.is_err());
        });
    }

    fn camera_photos() {
        let mock_server = MockServer::start();
        let url = format!("http://{}", mock_server.address());
        let resp = LoginResponse {
            uuid: String::from("7777777777777AA"),
            token: String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw")
        };
        let auth = format!("Bearer {}", resp.token);
        let login_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(PATH_PHOTOS)
                .header("Authorization", auth);
            then.status(200).body(CAMERA_PHOTOS);
        });

        let server = Server {
            user_name: String::from("ed"),
            password: String::from("money"),
            host: url,
        };

        let client = client::Client::new(server).expect("spypoint client");
        client.set_auth(resp.token.clone());

        tokio_test::block_on(async {
            let result =
                spypoint::camera_photos(&client, "66985496c6eb10dbad5c51f6".to_string(), Some(120))
                    .await;

            login_mock.assert();

            assert!(!result.is_err());
            let photos = result.expect("camera response");

            assert_eq!(photos.camera_ids.len(), 1);
            assert_eq!(photos.photos.len(), 4);
            assert_eq!(photos.count_photos, 4);

            for photo in photos.photos {
                assert!(!photo.camera.is_empty());
                assert!(!photo.date.is_empty());
                assert!(!photo.id.is_empty());
                assert!(!photo.origin_date.is_empty());
                assert!(!photo.origin_name.is_empty());
                assert!(photo.origin_size > 0);
                assert!(!photo.tag.is_empty());

                assert!(!photo.large.verb.is_empty());
                assert!(!photo.large.path.is_empty());
                assert!(!photo.large.host.is_empty());
                assert!(!photo.large.headers.is_empty());
            }
        });
    }

    #[test]
    fn camera() {
        let mock_server = MockServer::start();
        let url = format!("http://{}", mock_server.address());

        let resp = LoginResponse {
            uuid: String::from("7777777777777AA"),
            token: String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw")
        };

        let auth = format!("Bearer {}", resp.token);

        let camera_id = "5f145aaf173ca3001571df15";
        let path = format!("{}{}", PATH_CAMERA, camera_id);

        let login_mock = mock_server.mock(|when, then| {
            when.method(GET).path(path).header("Authorization", auth);
            then.status(200).body(CAMERA_RESPONSE);
        });

        let server = Server {
            user_name: String::from("ed"),
            password: String::from("money"),
            host: url,
        };

        let client = client::Client::new(server).expect("spypoint client");
        client.set_auth(resp.token.clone());

        tokio_test::block_on(async {
            let result = spypoint::camera(&client, camera_id.to_string()).await;

            login_mock.assert();

            assert!(!result.is_err());
            let c = result.expect("camera response");

            assert_eq!(c.config.name, "Clover Field");
        });
    }

    #[test]
    fn all_cameras() {
        let mock_server = MockServer::start();
        let url = format!("http://{}", mock_server.address());

        let resp = LoginResponse {
            uuid: String::from("7777777777777AA"),
            token: String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw")
        };

        let auth = format!("Bearer {}", resp.token);

        let login_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(PATH_CAMERAS_ALL)
                .header("Authorization", auth);
            then.status(200).body(CAMERA_ALL);
        });

        let server = Server {
            user_name: String::from("ed"),
            password: String::from("money"),
            host: url,
        };

        let client = client::Client::new(server).expect("spypoint client");
        client.set_auth(resp.token.clone());

        tokio_test::block_on(async {
            let result = spypoint::cameras(&client).await;

            login_mock.assert();

            assert!(!result.is_err());
            let c = result.expect("all cameras response");

            println!("cameras: \n{:?}\n", c);

            assert!(c.len() > 0);
            assert!(!c[0].config.name.is_empty());
            assert!(!c[0].status.last_update.is_empty());
            assert!(!c[0].id.is_empty());
            assert!(!c[0].activation_date.is_empty());
        });
    }

    const LOGIN_RESPONSE: &str = r#"{
  "uuid": "5f14230017d3051e",
  "token": "eyJyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw"
}"#;

    const CAMERA_PHOTOS: &str = r#"
    {"cameraId":null,"cameraIds":["66985496c6eb10dbad5c51f6"],"countPhotos":4,"photos":[{"camera":"66985496c6eb10dbad5c51f6","date":"2024-07-17T23:52:04.697Z","id":"669859240be0b2c3a252c536","large":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0004_2024071723529TSux.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=48f4edc1cd037ef86062a7dad966c61423048104d47ef5a106db6a105ba6d56b","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"medium":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0004_M_2024071723529TSux.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=07dd13be3f4978cc004d21fc29d2d64908f28515811e0df8aa1f4e6065ff3688","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"originDate":"2024-07-17T19:51:41.000Z","originName":"PICT0004.JPG","originSize":16483,"small":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0004_S_2024071723529TSux.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=94728715d1e2266e25deccc7465681d674419214a227d58395125a40f3533c7f","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"tag":["day"]},{"camera":"66985496c6eb10dbad5c51f6","date":"2024-07-17T23:50:54.470Z","id":"669858de7038784df54e44fb","large":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0003_202407172350WKsik.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=72084d6f90626113747912c40befff80949b00f4fd1f478e358493ca40dcdb70","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"medium":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0003_M_202407172350WKsik.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=f51f37fa06ce04b8a7637e3467b81e40e865a2258585b2fa3c05a4edb0cc87df","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"originDate":"2024-07-17T19:50:30.000Z","originName":"PICT0003.JPG","originSize":16147,"small":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0003_S_202407172350WKsik.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=29964fd438e1d6a79ca01ec0f6a5dd58536b96e7ac1b4f9a1490b99339d4a551","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"tag":["day"]},{"camera":"66985496c6eb10dbad5c51f6","date":"2024-07-17T23:50:54.470Z","id":"669858de7038784df54e44f9","large":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0002_202407172350OvInL.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=fed7dc39bc490aa1577f8b0e89a6eaabebfc005eee21c1f77bef88382b913f6d","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"medium":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0002_M_202407172350OvInL.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=e424782ae227acb927ef37ce5df026d13a8df32ef93020400deffebc062d4f5c","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"originDate":"2024-07-17T19:34:51.000Z","originName":"PICT0002.JPG","originSize":15504,"small":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0002_S_202407172350OvInL.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=ac02533eb070c06b79be3eff7135022e682e9d325fa01cc5166f5a7c2ad1a2e8","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"tag":["day"]},{"camera":"66985496c6eb10dbad5c51f6","date":"2024-07-17T23:50:54.470Z","id":"669858de7038784df54e44fa","large":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0001_202407172350OQVSP.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=97d41e90761cac4a92f0695d8cce0b02c4afb1fb77063abcfed5beac9672fd4c","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"medium":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0001_M_202407172350OQVSP.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=5d5d5de912f3a98529bdf552e901c756990d74f83fcedb96fa3a8ef781d5bac4","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"originDate":"2024-07-17T19:33:42.000Z","originName":"PICT0001.JPG","originSize":18409,"small":{"verb":"GET","path":"spypoint-production-account-ehcpvywr/5f145aae245c230017d3051e/66985496c6eb10dbad5c51f6/20240717/PICT0001_S_202407172350OQVSP.jpg?X-Amz-Expires=86400&X-Amz-Date=20240728T160811Z&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4TENZYKLZ2QLZNPE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-SignedHeaders=host&X-Amz-Signature=daeedee6b7b807c767d2d30b77b88a5814baec46904d75a4fb7db0bc693b2012","host":"s3.amazonaws.com","headers":[{"name":"Content-Type","value":"image/jpeg"}]},"tag":["day"]}]}
    "#;

    const CAMERA_RESPONSE: &str = r#"
    {
    "activationDate": "2020-07-19T14:37:35.058Z",
    "Config": {
    "batteryType": "AA",
    "capture": false,
    "captureMode": "photo",
    "dateFormat": "mdy",
    "delay": "1min",
    "multiShot": 1,
    "name": "Clover Field",
    "operationMode": "standard",
    "quality": "normal",
    "schedule": [
    [
    0,
    0
    ],
    [
    0,
    0
    ],
    [
    0,
    0
    ],
    [
    0,
    0
    ],
    [
    0,
    0
    ],
    [
    0,
    0
    ],
    [
    0,
    0
    ]
    ],
    "sensibility": {
    "high": 9,
    "level": "medium",
    "low": 35,
    "medium": 20
    },
    "smallPicWidth": 0,
    "stamp": true,
    "temperatureUnit": "F",
    "timeFormat": 12,
    "transmitAuto": false,
    "transmitFormat": "full",
    "transmitFreq": 12,
    "transmitTime": {
    "hour": 14,
    "minute": 52
    },
    "transmitUser": true,
    "triggerSpeed": "optimal"
    },
    "hdSince": "2020-08-20T16:15:00.000Z",
    "id": "5f145aaf173ca3001571df15",
    "status": {
    "batteries": [
    82
    ],
    "batteryType": "AA",
    "capability": {
    "hdRequest": true,
    "survivalMode": true
    },
    "installDate": "2021-09-27T17:12:00.000Z",
    "lastUpdate": "2022-01-25T14:53:00.000Z",
    "memory": {
    "size": 29568,
    "used": 2609
    },
    "model": "LINK-MICRO-V",
    "modemFirmware": "EC21VDFAR02A10M4G",
    "notifications": [],
    "serial": 0,
    "signal": {
    "bar": 2,
    "dBm": -113,
    "mcc": 311,
    "mnc": 480,
    "type": "LTE",
    "processed": {
    "percentage": 50,
    "bar": 2,
    "lowSignal": false
    }
    },
    "sim": "8944500209190478639",
    "temperature": {
    "value": 39
    },
    "version": "V1.11.06 HW:1"
    },
    "ucid": "865519047271252",
    "user": "5f145aae245c230017d3051e",
    "isCellular": true,
    "subscriptions": [
    {
    "id": "",
    "cameraId": "5f145aaf173ca3001571df15",
    "paymentStatus": "active",
    "isActive": true,
    "plan": {
    "name": "Free",
    "id": "Free",
    "isActive": true,
    "isFree": true,
    "isSelectable": true,
    "photoCountPerMonth": 100,
    "pricePerMonthIfPaidPerMonth": 0,
    "pricePerMonthIfPaidAnnually": 0,
    "pricePerYear": 0,
    "pricePerMonthIfPaidAnnuallyInsidersClub": 0,
    "pricePerMonthIfPaidPerMonthInsidersClub": 0,
    "pricePerYearInsidersClub": 0,
    "rebateIfPaidAnnually": 0,
    "rebatePercentageInsidersClub": 20,
    "showBanner": "",
    "isUpgradable": true,
    "isDowngradable": false
    },
    "currency": "USD",
    "paymentFrequency": "month_by_month",
    "isFree": true,
    "startDateBillingCycle": "2024-06-30T15:41:18.674Z",
    "endDateBillingCycle": "2024-07-30T15:41:18.674Z",
    "monthEndBillingCycle": "2024-07-30T15:41:18.674Z",
    "photoCount": 0,
    "isAutoRenew": false
    }
    ],
    "dataMatrixKey": "ER2J2V5FTM2",
    "ptpNotifications": []
    }"#;

    const ALL_CAMERAS_RESPONSE: &str = r#"[
  {
    "activationDate": "2020-07-19T14:37:35.058Z",
    "Config": {
      "batteryType": "AA",
      "capture": false,
      "captureMode": "photo",
      "dateFormat": "mdy",
      "delay": "1min",
      "multiShot": 1,
      "name": "Clover Field",
      "operationMode": "standard",
      "quality": "normal",
      "schedule": [
        [
          0,
          0
        ],
        [
          0,
          0
        ],
        [
          0,
          0
        ],
        [
          0,
          0
        ],
        [
          0,
          0
        ],
        [
          0,
          0
        ],
        [
          0,
          0
        ]
      ],
      "sensibility": {
        "high": 9,
        "level": "medium",
        "low": 35,
        "medium": 20
      },
      "smallPicWidth": 0,
      "stamp": true,
      "temperatureUnit": "F",
      "timeFormat": 12,
      "transmitAuto": false,
      "transmitFormat": "full",
      "transmitFreq": 12,
      "transmitTime": {
        "hour": 14,
        "minute": 52
      },
      "transmitUser": true,
      "triggerSpeed": "optimal"
    },
    "hdSince": "2020-08-20T16:15:00.000Z",
    "id": "5f145aaf173ca3001571df15",
    "status": {
      "batteries": [
        82
      ],
      "batteryType": "AA",
      "capability": {
        "hdRequest": true,
        "survivalMode": true
      },
      "installDate": "2021-09-27T17:12:00.000Z",
      "lastUpdate": "2022-01-25T14:53:00.000Z",
      "memory": {
        "size": 29568,
        "used": 2609
      },
      "model": "LINK-MICRO-V",
      "modemFirmware": "EC21VDFAR02A10M4G",
      "notifications": [],
      "serial": 0,
      "signal": {
        "bar": 2,
        "dBm": -113,
        "mcc": 311,
        "mnc": 480,
        "type": "LTE",
        "processed": {
          "percentage": 50,
          "bar": 2,
          "lowSignal": false
        }
      },
      "sim": "8944500209190478639",
      "temperature": {
        "value": 39
      },
      "version": "V1.11.06 HW:1"
    },
    "ucid": "865519047271252",
    "user": "5f145aae245c230017d3051e",
    "isCellular": true,
    "subscriptions": [
      {
        "id": "",
        "cameraId": "5f145aaf173ca3001571df15",
        "paymentStatus": "active",
        "isActive": true,
        "plan": {
          "name": "Free",
          "id": "Free",
          "isActive": true,
          "isFree": true,
          "isSelectable": true,
          "photoCountPerMonth": 100,
          "pricePerMonthIfPaidPerMonth": 0,
          "pricePerMonthIfPaidAnnually": 0,
          "pricePerYear": 0,
          "pricePerMonthIfPaidAnnuallyInsidersClub": 0,
          "pricePerMonthIfPaidPerMonthInsidersClub": 0,
          "pricePerYearInsidersClub": 0,
          "rebateIfPaidAnnually": 0,
          "rebatePercentageInsidersClub": 20,
          "showBanner": "",
          "isUpgradable": true,
          "isDowngradable": false
        },
        "currency": "USD",
        "paymentFrequency": "month_by_month",
        "isFree": true,
        "startDateBillingCycle": "2024-06-30T15:41:18.674Z",
        "endDateBillingCycle": "2024-07-30T15:41:18.674Z",
        "monthEndBillingCycle": "2024-07-30T15:41:18.674Z",
        "photoCount": 0,
        "isAutoRenew": false
      }
    ],
    "dataMatrixKey": "ER2J2V5FTM2",
    "ptpNotifications": []
  }
]"#;

    const CAMERA_ALL: &str = r#"
   [{"activationDate":"2024-07-17T23:43:19.162Z","config":{"batteryType":"AUTO","capture":false,"captureMode":"photo","dateFormat":"mdy","detectionSchedule":[],"factory":false,"gps":true,"image":{"jpegOptim":{"dayTargetSize":500,"nightTargetSize":500,"triggerSize":60},"quality":{"qFactorDay":50,"qFactorNight":50,"resizeMethod":1},"transmit":{"maxSize":512,"minSize":1,"maxHDSize":1024,"minHDSize":5,"maxHDVideoSize":51200,"minHDVideoSize":500}},"logLevel":"warning","motionDelay":60,"multiShot":1,"name":"FLEX-3TME","operationMode":"standard","quality":"high","schedule":[[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0]],"sensibility":{"high":15,"level":"medium","low":35,"medium":20},"smallPicWidth":720,"timeFormat":12,"timeLapse":3600,"transmitAuto":true,"transmitFormat":"full","transmitFreq":12,"transmitTime":{"hour":23,"minute":58},"transmitUser":true},"creationDate":"2024-07-17T23:43:19.162Z","dataMatrixKey":"I6M2KID3TME","hdSince":"2024-07-17T19:32:57.000Z","id":"66985496c6eb10dbad5c51f6","status":{"batteries":[100,0,0],"batteryType":"AA","activePowerSource":0,"powerSources":[{"location":"TRAY1","type":"AA","percentage":100,"voltage":12183}],"capability":{"hdRequest":true,"video":true},"coordinates":[{"dateTime":"2024-07-17T19:52:21.000Z","latitude":"N25 32.654460","longitude":"W80 26.399520","position":{"type":"Point","coordinates":[-80.439992,25.544241]},"geohash":"dhwc3d1murds"}],"installDate":"2024-07-17T19:48:54.000Z","lastUpdate":"2024-07-17T19:52:21.000Z","memory":{"size":29798,"used":1},"model":"FLEX","modemFirmware":"EG91NAFBR05A07M4G","notifications":["sd_card_one_partition"],"signal":{"bar":4,"dBm":-94,"mcc":311,"mnc":480,"type":"LTE","processed":{"percentage":100,"bar":5,"lowSignal":false}},"sim":"89148000008057211843","temperature":{"unit":"F","value":86},"version":"1.8.0-97-gd0a85c9"},"ucid":"866846054077507","user":"5f145aae245c230017d3051e","isCellular":true,"subscriptions":[{"id":"","cameraId":"66985496c6eb10dbad5c51f6","paymentStatus":"active","isActive":true,"plan":{"name":"Free","id":"Free","isActive":true,"isFree":true,"isSelectable":true,"photoCountPerMonth":100,"pricePerMonthIfPaidPerMonth":0,"pricePerMonthIfPaidAnnually":0,"pricePerYear":0,"pricePerMonthIfPaidAnnuallyInsidersClub":0,"pricePerMonthIfPaidPerMonthInsidersClub":0,"pricePerYearInsidersClub":0,"rebateIfPaidAnnually":0,"rebatePercentageInsidersClub":20,"showBanner":"","isUpgradable":true,"isDowngradable":false},"currency":"USD","paymentFrequency":"month_by_month","isFree":true,"startDateBillingCycle":"2024-07-17T23:43:19.163Z","endDateBillingCycle":"2024-08-17T23:43:19.163Z","monthEndBillingCycle":"2024-08-17T23:43:19.163Z","photoCount":4,"isAutoRenew":false}],"ptpNotifications":[]}]
    "#;
}
