use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubEvent {
    pub action: String,
    pub sponsorship: Sponsorship,
}

#[derive(Debug, Deserialize)]
pub struct Sponsorship {
    pub sponsor: Sponsor,
    pub tier: Tier,
}

#[derive(Debug, Deserialize)]
pub struct Sponsor {
    pub login: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Tier {
    pub monthly_price_in_dollars: i64,
}
