/*
 * VOICEVOX Engine
 *
 * VOICEVOXの音声合成エンジンです。
 *
 * The version of the OpenAPI document: 0.14.0-preview.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WordTypes :      fastapiでword_type引数を検証する時に使用するクラス     

///      fastapiでword_type引数を検証する時に使用するクラス     
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WordTypes {
    #[serde(rename = "PROPER_NOUN")]
    PROPERNOUN,
    #[serde(rename = "COMMON_NOUN")]
    COMMONNOUN,
    #[serde(rename = "VERB")]
    VERB,
    #[serde(rename = "ADJECTIVE")]
    ADJECTIVE,
    #[serde(rename = "SUFFIX")]
    SUFFIX,

}

impl ToString for WordTypes {
    fn to_string(&self) -> String {
        match self {
            Self::PROPERNOUN => String::from("PROPER_NOUN"),
            Self::COMMONNOUN => String::from("COMMON_NOUN"),
            Self::VERB => String::from("VERB"),
            Self::ADJECTIVE => String::from("ADJECTIVE"),
            Self::SUFFIX => String::from("SUFFIX"),
        }
    }
}




