use leptos::{on_cleanup, Scope, Serializable};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
pub async fn fetch_leaderbord() -> Vec<Score> {
    let headers = Headers::new();
    headers.set("SESSION", &env::var("AOC_SESSION_COOKIE").unwrap());

    let res = Request::get("https://adventofcode.com/2022/leaderboard/private/view/143527.json")
        .headers(headers)
        .send()
        .await;

    match res {
        Ok(data) => {
            let json = data.json::<crate::types::AdventOfCodeResponse>().await;

            match json {
                Ok(json_data) => {
                    let members = json!(&json_data.members);
                    let members_slice = members.as_array();
                    println!("{:?}", members_slice);
                    vec![]
                }
                Err(..) => vec![],
            }
        }
        Err(..) => vec![],
    }
}

#[cfg(not(feature = "ssr"))]
pub async fn fetch_leaderbord() -> Vec<Score> {
    vec![]
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdventOfCodeResponse {
    pub event: String,
    pub members: Members,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Members {
    #[serde(rename = "1789")]
    pub n1789: Score,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: i64,
    #[serde(rename = "global_score")]
    pub global_score: i64,
    #[serde(rename = "local_score")]
    pub local_score: i64,
    #[serde(rename = "completion_day_level")]
    pub completion_day_level: CompletionDayLevel,
    pub name: String,
    pub stars: i64,
    #[serde(rename = "last_star_ts")]
    pub last_star_ts: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionDayLevel {
    #[serde(rename = "1")]
    pub n1: n1,
    #[serde(rename = "2")]
    pub n2: n22,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1 {
    #[serde(rename = "1")]
    pub n1: n12,
    #[serde(rename = "2")]
    pub n2: n2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n12 {
    #[serde(rename = "get_star_ts")]
    pub get_star_ts: i64,
    #[serde(rename = "star_index")]
    pub star_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2 {
    #[serde(rename = "get_star_ts")]
    pub get_star_ts: i64,
    #[serde(rename = "star_index")]
    pub star_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22 {
    #[serde(rename = "1")]
    pub n1: n13,
    #[serde(rename = "2")]
    pub n2: n23,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n13 {
    #[serde(rename = "star_index")]
    pub star_index: i64,
    #[serde(rename = "get_star_ts")]
    pub get_star_ts: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n23 {
    #[serde(rename = "get_star_ts")]
    pub get_star_ts: i64,
    #[serde(rename = "star_index")]
    pub star_index: i64,
}
