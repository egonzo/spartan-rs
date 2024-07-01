use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::client::Client;
use crate::Result;

pub(crate) const PATH_LOGIN:&str = "/api/v3/user/login";
pub(crate) const PATH_CAMERAS_ALL: &str = "/api/v3/camera/all";
pub(crate) const PATH_CAMERA :&str ="/api/v3/camera/";


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



/// Login logs in to the api. If successful it sets the auth token and uuid on the client.
pub(crate) async fn login(client: &Client, login: Login) -> Result<()> {
    let result:LoginResponse = client.send_request(&login, Method::POST, PATH_LOGIN, false).await?;

    client.set_auth(result.token);
    client.set_uuid(result.uuid);

    Ok(())
}

// ***** Cameras

pub(crate) async fn cameras(client: &Client) -> Result<Cameras> {
   let result:Cameras = client.get_request(PATH_CAMERAS_ALL, true).await?;

   Ok(result)
}

pub(crate) async fn camera(client: &Client, camera_id: String) -> Result<Camera> {
    let path = format!("{}{}",PATH_CAMERA, camera_id);

    let result:Camera = client.get_request(path.as_str(), true).await?;

    Ok(result)
}

pub type Cameras = Vec<Camera>;

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Camera {
    #[serde(rename = "activationDate")]
    activation_date: String,

    #[serde(rename = "config")]
    config: Config,

    #[serde(rename = "hdSince")]
    hd_since: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "status")]
    status: Status,

    #[serde(rename = "ucid")]
    ucid: String,

    #[serde(rename = "user")]
    user: String,

    #[serde(rename = "isCellular")]
    is_cellular: bool,

    #[serde(rename = "subscriptions")]
    subscriptions: Vec<Subscription>,

    #[serde(rename = "dataMatrixKey")]
    data_matrix_key: String,

    #[serde(rename = "ptpNotifications")]
    ptp_notifications: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(rename = "batteryType")]
    battery_type: String,

    #[serde(rename = "capture")]
    capture: bool,

    #[serde(rename = "captureMode")]
    capture_mode: String,

    #[serde(rename = "dateFormat")]
    date_format: String,

    #[serde(rename = "delay")]
    delay: String,

    #[serde(rename = "multiShot")]
    multi_shot: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "operationMode")]
    operation_mode: String,

    #[serde(rename = "quality")]
    quality: String,

    #[serde(rename = "schedule")]
    schedule: Vec<Vec<i64>>,

    #[serde(rename = "sensibility")]
    sensibility: Sensibility,

    #[serde(rename = "smallPicWidth")]
    small_pic_width: i64,

    #[serde(rename = "stamp")]
    stamp: bool,

    #[serde(rename = "temperatureUnit")]
    temperature_unit: String,

    #[serde(rename = "timeFormat")]
    time_format: i64,

    #[serde(rename = "transmitAuto")]
    transmit_auto: bool,

    #[serde(rename = "transmitFormat")]
    transmit_format: String,

    #[serde(rename = "transmitFreq")]
    transmit_freq: i64,

    #[serde(rename = "transmitTime")]
    transmit_time: TransmitTime,

    #[serde(rename = "transmitUser")]
    transmit_user: bool,

    #[serde(rename = "triggerSpeed")]
    trigger_speed: String,
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

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Status {
    #[serde(rename = "batteries")]
    batteries: Vec<i64>,

    #[serde(rename = "batteryType")]
    battery_type: String,

    #[serde(rename = "capability")]
    capability: Capability,

    #[serde(rename = "installDate")]
    install_date: String,

    #[serde(rename = "lastUpdate")]
    last_update: String,

    #[serde(rename = "memory")]
    memory: Memory,

    #[serde(rename = "model")]
    model: String,

    #[serde(rename = "modemFirmware")]
    modem_firmware: String,

    #[serde(rename = "notifications")]
    notifications: Vec<Option<serde_json::Value>>,

    #[serde(rename = "serial")]
    serial: i64,

    #[serde(rename = "signal")]
    signal: Signal,

    #[serde(rename = "sim")]
    sim: String,

    #[serde(rename = "temperature")]
    temperature: Temperature,

    #[serde(rename = "version")]
    version: String,
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
    size: i64,

    #[serde(rename = "used")]
    used: i64,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Signal {
    #[serde(rename = "bar")]
    bar: i64,

    #[serde(rename = "dBm")]
    d_bm: i64,

    #[serde(rename = "mcc")]
    mcc: i64,

    #[serde(rename = "mnc")]
    mnc: i64,

    #[serde(rename = "type")]
    signal_type: String,

    #[serde(rename = "processed")]
    processed: Processed,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Processed {
    #[serde(rename = "percentage")]
    percentage: i64,

    #[serde(rename = "bar")]
    bar: i64,

    #[serde(rename = "lowSignal")]
    low_signal: bool,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Temperature {
    #[serde(rename = "value")]
    value: i64,
}

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Subscription {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "cameraId")]
    camera_id: String,

    #[serde(rename = "paymentStatus")]
    payment_status: String,

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
    photo_count: i64,

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

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use crate::{client, spypoint};
    use crate::client::Server;
    use crate::spypoint::{Login, LoginResponse, PATH_CAMERA, PATH_CAMERAS_ALL, PATH_LOGIN};

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
            password: client.user_password()
        };

        tokio_test::block_on(async {
            let result = spypoint::login(&client, l).await;

            login_mock.assert();

            assert!(!result.is_err());
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
            when.method(GET).path(path)
                .header("Authorization", auth);
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
            when.method(GET).path(PATH_CAMERAS_ALL)
                .header("Authorization", auth);
            then.status(200).body(ALL_CAMERAS_RESPONSE);
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

            assert!(c.len() > 0);
            assert_ne!(c[0].config.name, "");
        });
    }

    const LOGIN_RESPONSE: &str = r#"{
  "uuid": "5f14230017d3051e",
  "token": "eyJyIjp7Il9pZCI6IjVmMTQ1YWFlMjQ1YzIzMDAxN2QzMDUxZSJ9LCJzZXNzaW9uIjp7ImlkIjoiOWM5Nzc2YmEtNjIwYS00YWYyLTljNDItMmQzOGU5NTIzODJhIn0sImlhdCI6MTcxOTc5NTI0NSwiZXhwIjoxNzE5ODgxNjQ1fQ.xDrO__0U5aVjFXdYyVE2GuAh_vniuuJrGqqHjzwcKJw"
}"#;

    const CAMERA_RESPONSE: &str = r#"
    {
    "activationDate": "2020-07-19T14:37:35.058Z",
    "config": {
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
    "config": {
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
}