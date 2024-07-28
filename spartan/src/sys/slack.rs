// slackMsg := fmt.Sprintf("Wildcat Reveal Job Error: %+v", err)
// slack.APIError(config.SlackURL, slackMsg)

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Message {
    text: String,
    attachments: Vec<Attachment>,
}

#[derive(Serialize, Default)]
pub struct Attachment {
    title: String,
    color: String,
    fields: Vec<Field>,
}

#[derive(Serialize, Default)]
pub struct Field {
    name: String,
    value: String,
}

pub async fn save_error(
    client: reqwest::Client,
    url: String,
    msg: String,
    title: String,
) -> crate::Result<()> {
    if url.is_empty() {
        return Ok(());
    }

    let t = format!("{} API Error", title);
    let m = Message {
        text: msg,
        attachments: vec![Attachment {
            title: t,
            color: String::from("#f00"),
            ..Default::default()
        }],
    };

    client.post(url).json(&m).send().await?;

    Ok(())
}

pub async fn send_message(client: reqwest::Client, url: String, msg: String) -> crate::Result<()> {
    if url.is_empty() {
        return Ok(());
    }

    let m = format!("```{}```", msg);

    let message = Message {
        text: m,
        ..Default::default()
    };

    client.post(url).json(&message).send().await?;

    Ok(())
}
