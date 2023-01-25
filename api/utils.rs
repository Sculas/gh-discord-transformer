pub struct Error(pub &'static str);

pub type Result<T> = std::result::Result<T, Error>;

pub async fn notify_discord(username: &str, profile_url: &str, usd_donated: f64) -> Result<()> {
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL").map_err(|_| Error("no-webhook-url"))?;
    let thread_id = std::env::var("DISCORD_WEBHOOK_TID").ok();
    let client = reqwest::Client::builder()
        .build()
        .map_err(|_| Error("bad-client"))?;
    let mut req = client.post(&webhook_url);
    if let Some(thread_id) = thread_id {
        req = req.query(&[("thread_id", thread_id)]);
    }
    let res = req
        .json(&serde_json::json!({
            "embeds": [
                {
                    "title": "New Sponsor!",
                    "description": format!("[{}]({}) just donated ${}!", username, profile_url, usd_donated),
                    "color": 0x00ff00,
                    "footer": {
                        "text": "Sponsorship Notifications"
                    }
                }
            ],
            "allowed_mentions": {
              "parse": []
            }
        }))
        .send()
        .await
        .map_err(|_| Error("discord-bad-request"))?;
    if res.status() != http::StatusCode::NO_CONTENT {
        return Err(Error("bad-discord-status"));
    }
    Ok(())
}

#[macro_export]
macro_rules! wrap_handler {
    ($handler:ident => $wrapped_handler:ident) => {
        async fn $wrapped_handler(
            req: vercel_lambda::Request,
        ) -> std::result::Result<
            vercel_lambda::Response<vercel_lambda::Body>,
            vercel_lambda::error::VercelError,
        > {
            match $handler(req).await {
                Ok(res) => Ok(res.into_response()),
                Err(err) => Ok(vercel_lambda::Response::builder()
                    .status(http::StatusCode::BAD_REQUEST)
                    .body(err.0.into())
                    .unwrap()),
            }
        }
    };
}
