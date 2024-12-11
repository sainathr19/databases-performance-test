use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use crate::helpers::utils::{parse_f64_or_skip,serialize_f64};

#[derive(Serialize, Deserialize, FromRow,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpmuHistoryInterval {
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub count: f64,
    
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub end_time: f64,
    
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub start_time: f64,
    
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub units: f64,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpmuHistoryMeta {
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub end_count: f64,
    
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub end_time: f64,
    
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub end_units: f64,
    
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub start_count: f64,
    
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub start_time: f64,
    
    #[serde(deserialize_with = "parse_f64_or_skip", 
            serialize_with = "serialize_f64")]
    pub start_units: f64,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct RpmuHistoryResponse {
    pub intervals: Vec<RpmuHistoryInterval>,
    pub meta: RpmuHistoryMeta,
}