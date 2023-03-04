# EngineManifest

## Properties

| Name                            | Type                                                                 | Description | Notes      |
| ------------------------------- | -------------------------------------------------------------------- | ----------- | ---------- |
| **manifest_version**            | **String**                                                           |             |            |
| **name**                        | **String**                                                           |             |            |
| **brand_name**                  | **String**                                                           |             |            |
| **uuid**                        | **String**                                                           |             |            |
| **url**                         | **String**                                                           |             |            |
| **icon**                        | **String**                                                           |             |            |
| **default_sampling_rate**       | **i32**                                                              |             |            |
| **terms_of_service**            | **String**                                                           |             |            |
| **update_infos**                | [**Vec<crate::models::UpdateInfo>**](UpdateInfo.md)                  |             |            |
| **dependency_licenses**         | [**Vec<crate::models::LicenseInfo>**](LicenseInfo.md)                |             |            |
| **downloadable_libraries_path** | Option<**String**>                                                   |             | [optional] |
| **downloadable_libraries_url**  | Option<**String**>                                                   |             | [optional] |
| **supported_features**          | Option<[**crate::models::SupportedFeatures**](SupportedFeatures.md)> |             |            |

[[Back to Model list]](../README.md#documentation-for-models)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to README]](../README.md)
