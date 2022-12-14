use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::slice::GroupBy;

pub async fn fetch_leaderbord() -> Option<Vec<Vec<Score>>> {
    let client = reqwest::Client::new();

    let session_cookie = format!("session={}", &env::var("AOC_SESSION_COOKIE").unwrap());

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", HeaderValue::from_str(&session_cookie).unwrap());

    let request = client
        .request(
            reqwest::Method::GET,
            "https://adventofcode.com/2022/leaderboard/private/view/558189.json",
        )
        .headers(headers);

    let res = request.send().await.map_err(|e| log::error!("{e}")).ok()?;
    let json = res.text().await.ok()?;

    let parsed_json: serde_json::Value = serde_json::from_str(&json).unwrap();
    let members = parsed_json.get("members").unwrap();

    let mut scores: Vec<Score> = members
        .as_object()
        .unwrap()
        .into_iter()
        .map(|x| {
            let value = x.1;

            Score {
                id: value.get("id").unwrap().as_i64().unwrap(),
                global_score: value.get("global_score").unwrap().as_i64().unwrap(),
                local_score: value.get("local_score").unwrap().as_i64().unwrap(),
                last_star_ts: value.get("last_star_ts").unwrap().as_i64().unwrap(),
                name: value.get("name").unwrap().to_string(),
                stars: value.get("stars").unwrap().as_i64().unwrap(),
            }
        })
        .collect();

    scores.sort_by_key(|x| x.stars);
    scores.reverse();

    let mut grouped: Vec<Vec<Score>> = Vec::new();
    for (index, group) in scores.group_by(|a, b| a.stars == b.stars).enumerate() {
        grouped.push(group.to_vec());
    }

    Some(grouped)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdventOfCodeResponse {
    pub event: String,
    pub members: Members,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Members {
    #[serde(rename = "1789")]
    pub n1789: Score,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: i64,
    #[serde(rename = "global_score")]
    pub global_score: i64,
    #[serde(rename = "local_score")]
    pub local_score: i64,
    pub name: String,
    pub stars: i64,
    #[serde(rename = "last_star_ts")]
    pub last_star_ts: i64,
}
