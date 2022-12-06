use leptos::{on_cleanup, Scope, Serializable};
use serde::{Deserialize, Serialize};
use std::env;

pub async fn fetch_leaderbord<T>() -> Option<T>
where
    T: Serializable,
{
    let client = reqwest::Client::new();
    let res = client
        .get("https://adventofcode.com/2022/leaderboard/private/view/143527.json")
        .header("SESSION", &env::var("AOC_SESSION_COOKIE").unwrap())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    T::from_json(&res).map_err(|e| log::error!("{e}")).ok()
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
    #[serde(rename = "completion_day_level")]
    pub completion_day_level: CompletionDayLevel,
    pub name: String,
    pub stars: i64,
    #[serde(rename = "last_star_ts")]
    pub last_star_ts: i64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompletionDayLevel {
    #[serde(rename = "1")]
    pub n1: n1,
    #[serde(rename = "2")]
    pub n2: n22,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct n1 {
    #[serde(rename = "1")]
    pub n1: n12,
    #[serde(rename = "2")]
    pub n2: n2,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct n12 {
    #[serde(rename = "get_star_ts")]
    pub get_star_ts: i64,
    #[serde(rename = "star_index")]
    pub star_index: i64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct n2 {
    #[serde(rename = "get_star_ts")]
    pub get_star_ts: i64,
    #[serde(rename = "star_index")]
    pub star_index: i64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct n22 {
    #[serde(rename = "1")]
    pub n1: n13,
    #[serde(rename = "2")]
    pub n2: n23,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct n13 {
    #[serde(rename = "star_index")]
    pub star_index: i64,
    #[serde(rename = "get_star_ts")]
    pub get_star_ts: i64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct n23 {
    #[serde(rename = "get_star_ts")]
    pub get_star_ts: i64,
    #[serde(rename = "star_index")]
    pub star_index: i64,
}
