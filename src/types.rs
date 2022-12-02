use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
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
