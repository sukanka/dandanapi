use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchRequest {
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "fileHash")]
    pub file_hash: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    #[serde(rename = "videoDuration")]
    pub video_duration: i32,
    #[serde(rename = "matchMode")]
    pub match_mode: MatchMode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MatchMode {
    #[serde(rename = "hashAndFileName")]
    HashAndFileName,
    #[serde(rename = "fileNameOnly")]
    FileNameOnly,
    #[serde(rename = "hashOnly")]
    HashOnly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestEpisodes {
    #[serde(rename = "anime")]
    pub anime: String,
    #[serde(rename = "tmdbId")]
    pub tmdb_id: Option<i32>,
    #[serde(rename = "episode")]
    pub episode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestComments {
    #[serde(rename = "from")]
    pub from: i64,
    #[serde(rename = "withRelated")]
    pub with_related: bool,
    #[serde(rename = "chConvert")]
    pub ch_convert: ChConvert,
}

#[derive(Debug, Clone)]
pub enum ChConvert {
    NONE = 0,
    SIMPLIFIED = 1,
    TRADITIONAL = 2,
}

impl serde::Serialize for ChConvert {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(match self {
            ChConvert::NONE => 0,
            ChConvert::SIMPLIFIED => 1,
            ChConvert::TRADITIONAL => 2,
        })
    }
}

impl<'de> serde::Deserialize<'de> for ChConvert {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(ChConvert::NONE),
            1 => Ok(ChConvert::SIMPLIFIED),
            2 => Ok(ChConvert::TRADITIONAL),
            _ => Err(serde::de::Error::custom(format!(
                "invalid ChConvert value: {value}"
            ))),
        }
    }
}
