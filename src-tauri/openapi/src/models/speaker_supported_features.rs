/*
 * VOICEVOX Engine
 *
 * VOICEVOXの音声合成エンジンです。
 *
 * The version of the OpenAPI document: 0.14.0-preview.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SpeakerSupportedFeatures : 話者の対応機能の情報



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpeakerSupportedFeatures {
    #[serde(rename = "permitted_synthesis_morphing", skip_serializing_if = "Option::is_none")]
    pub permitted_synthesis_morphing: Option<Box<crate::models::SpeakerSupportPermittedSynthesisMorphing>>,
}

impl SpeakerSupportedFeatures {
    /// 話者の対応機能の情報
    pub fn new() -> SpeakerSupportedFeatures {
        SpeakerSupportedFeatures {
            permitted_synthesis_morphing: None,
        }
    }
}

