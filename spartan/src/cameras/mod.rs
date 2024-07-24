use bson::DateTime;
use mongodb::{bson, Collection, Database};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::spypoint;

pub mod pictures;

const COLLECTION: &str = "cameras";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    last_transmission_timestamp: i64,
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    last_transmission: DateTime,
    memory: f64,
    temperature: f64,
    memory_limit: f64,
    signal: i64,
    battery_level: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GPS {
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    last_updated_timestamp: DateTime,
    longitude: String,
    latitude: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    stored_photos: i64,
    photos: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Camera {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub camera_id: String,
    pub name: String,
    pub r#type: String,
    pub updated_by: String,
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    pub last_updated_timestamp: DateTime,
    pub registration_status: String,
    pub created_timestamp: String,
    pub usage: Usage,
    pub status_file: String,
    pub phone_carrier: String,
    pub account_id: String,
    #[serde(rename = "iccid")]
    pub icc_id: String,
    pub hardware_version: String,
    pub location: String,
    pub firmware_version: String,
    pub status: Status,
    pub photo_count: i64,
    pub sd_card: String,
    pub gps: GPS,
    pub zip: String,
}

impl From<spypoint::Camera> for Camera {
    fn from(value: spypoint::Camera) -> Self {
        // last update
        let last_update = bson::DateTime::parse_rfc3339_str(value.status.last_update)
            .unwrap_or(bson::DateTime::now());

        // Subscription
        let subscription = value.subscriptions.into_iter().next();
        let mut reg_status = String::from("");
        let mut photo_count = 0;
        if subscription.is_some() {
            let s = subscription.unwrap();
            reg_status = s.payment_status;
            photo_count = s.photo_count;
        }

        // Battery Level
        let batteries = value.status.batteries.into_iter().next().unwrap_or(0);

        let status = Status {
            last_transmission_timestamp: 0, //Redo
            last_transmission: last_update,
            memory: value.status.memory.used as f64,
            temperature: value.status.temperature.value as f64,
            memory_limit: value.status.memory.size as f64,
            signal: value.status.signal.processed.bar,
            battery_level: batteries,
        };

        let mut lat = 0.00;
        let mut lng = 0.00;

        if !value.status.coordinates.is_empty()
            && value.status.coordinates[0].position.coordinates.len() == 2
        {
            lat = value.status.coordinates[0].position.coordinates[1];
            lng = value.status.coordinates[0].position.coordinates[0];
        }

        let gps = GPS {
            last_updated_timestamp: last_update,
            latitude: lat.to_string(),
            longitude: lng.to_string(),
        };

        let usage = Usage {
            stored_photos: photo_count,
            photos: photo_count,
        };

        Camera {
            id: None,
            camera_id: value.id,
            name: value.config.clone().name,
            r#type: String::from("spypoint"),
            updated_by: String::from(""),
            last_updated_timestamp: last_update,
            registration_status: reg_status.clone(),
            created_timestamp: value.activation_date,
            status_file: String::from(""),
            phone_carrier: String::from(""),
            account_id: value.user,
            icc_id: value.ucid,
            hardware_version: value.status.version,
            location: value.config.name,
            firmware_version: value.status.modem_firmware,
            status,
            photo_count,
            usage,
            sd_card: String::from(""),
            gps,
            zip: String::from(""),
        }
    }
}

impl Camera {
    pub async fn save(&self, db: &Database) -> crate::Result<()> {
        let coll: Collection<Camera> = db.collection(COLLECTION);
        let filter = doc! {
            "camera_id": &self.camera_id,
        };

        let _res = coll
            .find_one_and_update(filter, bson::to_document(self).unwrap_or_default())
            .upsert(true)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mongodb::bson;
    use serde_json;

    use crate::cameras::Camera;
    use crate::spypoint;

    #[test]
    fn parse_date() {
        bson::DateTime::parse_rfc3339_str("2022-01-25T14:53:00.000Z").unwrap();
    }

    #[test]
    fn from_spypoint_camera() {
        let sp_camera: spypoint::Camera = serde_json::from_str(SPY_CAMERA_JSON).unwrap();

        let camera: Camera = Camera::from(sp_camera);

        let json = serde_json::to_string(&camera).unwrap();
        println!("{json}");
    }

    const SPY_CAMERA_JSON: &str = r#"{
    "activationDate": "2024-07-17T23:43:19.162Z",
    "config": {
      "batteryType": "AUTO",
      "capture": false,
      "captureMode": "photo",
      "dateFormat": "mdy",
      "detectionSchedule": [],
      "factory": false,
      "image": {
        "jpegOptim": {
          "dayTargetSize": 500,
          "nightTargetSize": 500,
          "triggerSize": 60
        },
        "quality": {
          "qFactorDay": 50,
          "qFactorNight": 50,
          "resizeMethod": 1
        },
        "transmit": {
          "maxSize": 512,
          "minSize": 1,
          "maxHDSize": 1024,
          "minHDSize": 5,
          "maxHDVideoSize": 51200,
          "minHDVideoSize": 500
        }
      },
      "logLevel": "warning",
      "motionDelay": 60,
      "multiShot": 1,
      "name": "FLEX-3TME",
      "operationMode": "standard",
      "quality": "high",
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
        "high": 15,
        "level": "medium",
        "low": 35,
        "medium": 20
      },
      "smallPicWidth": 720,
      "timeFormat": 12,
      "timeLapse": 3600,
      "transmitAuto": true,
      "transmitFormat": "full",
      "transmitFreq": 12,
      "transmitTime": {
        "hour": 23,
        "minute": 58
      },
      "transmitUser": true,
      "gps": true
    },
    "creationDate": "2024-07-17T23:43:19.162Z",
    "dataMatrixKey": "I6M2KID3TME",
    "hdSince": "2024-07-17T19:32:57.000Z",
    "id": "66985496c6eb10dbad5c51f6",
    "status": {
      "batteries": [
        100,
        0,
        0
      ],
      "batteryType": "AA",
      "activePowerSource": 0,
      "powerSources": [
        {
          "location": "TRAY1",
          "type": "AA",
          "percentage": 100,
          "voltage": 12183
        }
      ],
      "capability": {
        "hdRequest": true,
        "video": true
      },
      "coordinates": [
        {
          "dateTime": "2024-07-17T19:52:21.000Z",
          "latitude": "N25 32.654460",
          "longitude": "W80 26.399520",
          "position": {
            "type": "Point",
            "coordinates": [
              -80.439992,
              25.544241
            ]
          },
          "geohash": "dhwc3d1murds"
        }
      ],
      "installDate": "2024-07-17T19:48:54.000Z",
      "lastUpdate": "2024-07-17T19:52:21.000Z",
      "memory": {
        "size": 29798,
        "used": 1
      },
      "model": "FLEX",
      "modemFirmware": "EG91NAFBR05A07M4G",
      "notifications": [
        "sd_card_one_partition"
      ],
      "signal": {
        "bar": 4,
        "dBm": -94,
        "mcc": 311,
        "mnc": 480,
        "type": "LTE",
        "processed": {
          "percentage": 100,
          "bar": 5,
          "lowSignal": false
        }
      },
      "sim": "89148000008057211843",
      "temperature": {
        "unit": "F",
        "value": 86
      },
      "version": "1.8.0-97-gd0a85c9"
    },
    "ucid": "866846054077507",
    "user": "5f145aae245c230017d3051e",
    "isCellular": true,
    "subscriptions": [
      {
        "id": "",
        "cameraId": "66985496c6eb10dbad5c51f6",
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
        "startDateBillingCycle": "2024-07-17T23:43:19.163Z",
        "endDateBillingCycle": "2024-08-17T23:43:19.163Z",
        "monthEndBillingCycle": "2024-08-17T23:43:19.163Z",
        "photoCount": 4,
        "isAutoRenew": false
      }
    ],
    "ptpNotifications": []
  }"#;
}
