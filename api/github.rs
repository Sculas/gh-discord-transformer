use hmac_sha256::HMAC;
use http::StatusCode;
use serde_json::from_slice;
use std::env::var;
use std::result::Result as StdResult;
use utils::{Error, Result};
use vercel_lambda::{lambda_async, IntoResponse, Request, Response};

mod types;
mod utils;

async fn handler(req: Request) -> Result<impl IntoResponse> {
    verify_sig(&req)?;
    if !is_ping_event(&req) {
        let event: types::GithubEvent = from_slice(req.body()).map_err(|_| Error("bad-json"))?;
        if event.action == "created" {
            utils::notify_discord(
                &event.sponsorship.sponsor.login,
                &event.sponsorship.sponsor.html_url,
                event.sponsorship.tier.monthly_price_in_dollars,
            )
            .await?;
        }
    } else {
        println!("ping event");
    }
    Ok(Response::builder().status(StatusCode::OK).body(()).unwrap())
}

fn is_ping_event(req: &Request) -> bool {
    req.headers().get("X-GitHub-Event").map(|s| s.as_bytes()) == Some(b"ping")
}

fn verify_sig(req: &Request) -> Result<()> {
    let req_sig = match req
        .headers()
        .get("X-Hub-Signature-256")
        .map(|s| s.as_bytes())
    {
        Some(sig) => sig,
        None => return Err(Error("no-sig")),
    };
    let secret = var("GITHUB_SECRET").map_err(|_| Error("no-secret"))?;
    let sig = format!("sha256={}", hex::encode(HMAC::mac(req.body(), &secret)));
    // I know this is unsafe, but I don't really care for this project
    if sig.as_bytes() != req_sig {
        return Err(Error("bad-sig"));
    }
    Ok(())
}

// Start the runtime with the handler
fn main() -> StdResult<(), Box<dyn std::error::Error>> {
    wrap_handler!(handler => wrapped_handler);
    Ok(lambda_async!(wrapped_handler))
}
