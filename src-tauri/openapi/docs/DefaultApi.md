# \DefaultApi

All URIs are relative to _http://localhost_

| Method                                                                                                                           | HTTP request                           | Description                                                        |
| -------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------- | ------------------------------------------------------------------ |
| [**accent_phrases_accent_phrases_post**](DefaultApi.md#accent_phrases_accent_phrases_post)                                       | **POST** /accent_phrases               | テキストからアクセント句を得る                                     |
| [**add_preset_add_preset_post**](DefaultApi.md#add_preset_add_preset_post)                                                       | **POST** /add_preset                   | Add Preset                                                         |
| [**add_user_dict_word_user_dict_word_post**](DefaultApi.md#add_user_dict_word_user_dict_word_post)                               | **POST** /user_dict_word               | Add User Dict Word                                                 |
| [**audio_query_audio_query_post**](DefaultApi.md#audio_query_audio_query_post)                                                   | **POST** /audio_query                  | 音声合成用のクエリを作成する                                       |
| [**audio_query_from_preset_audio_query_from_preset_post**](DefaultApi.md#audio_query_from_preset_audio_query_from_preset_post)   | **POST** /audio_query_from_preset      | 音声合成用のクエリをプリセットを用いて作成する                     |
| [**cancellable_synthesis_cancellable_synthesis_post**](DefaultApi.md#cancellable_synthesis_cancellable_synthesis_post)           | **POST** /cancellable_synthesis        | 音声合成する（キャンセル可能）                                     |
| [**connect_waves_connect_waves_post**](DefaultApi.md#connect_waves_connect_waves_post)                                           | **POST** /connect_waves                | base64エンコードされた複数のwavデータを一つに結合する              |
| [**core_versions_core_versions_get**](DefaultApi.md#core_versions_core_versions_get)                                             | **GET** /core_versions                 | Core Versions                                                      |
| [**delete_preset_delete_preset_post**](DefaultApi.md#delete_preset_delete_preset_post)                                           | **POST** /delete_preset                | Delete Preset                                                      |
| [**delete_user_dict_word_user_dict_word_word_uuid_delete**](DefaultApi.md#delete_user_dict_word_user_dict_word_word_uuid_delete) | **DELETE** /user_dict_word/{word_uuid} | Delete User Dict Word                                              |
| [**downloadable_libraries_downloadable_libraries_get**](DefaultApi.md#downloadable_libraries_downloadable_libraries_get)         | **GET** /downloadable_libraries        | Downloadable Libraries                                             |
| [**engine_manifest_engine_manifest_get**](DefaultApi.md#engine_manifest_engine_manifest_get)                                     | **GET** /engine_manifest               | Engine Manifest                                                    |
| [**get_presets_presets_get**](DefaultApi.md#get_presets_presets_get)                                                             | **GET** /presets                       | Get Presets                                                        |
| [**get_user_dict_words_user_dict_get**](DefaultApi.md#get_user_dict_words_user_dict_get)                                         | **GET** /user_dict                     | Get User Dict Words                                                |
| [**import_user_dict_words_import_user_dict_post**](DefaultApi.md#import_user_dict_words_import_user_dict_post)                   | **POST** /import_user_dict             | Import User Dict Words                                             |
| [**initialize_speaker_initialize_speaker_post**](DefaultApi.md#initialize_speaker_initialize_speaker_post)                       | **POST** /initialize_speaker           | Initialize Speaker                                                 |
| [**is_initialized_speaker_is_initialized_speaker_get**](DefaultApi.md#is_initialized_speaker_is_initialized_speaker_get)         | **GET** /is_initialized_speaker        | Is Initialized Speaker                                             |
| [**mora_data_mora_data_post**](DefaultApi.md#mora_data_mora_data_post)                                                           | **POST** /mora_data                    | アクセント句から音高・音素長を得る                                 |
| [**mora_length_mora_length_post**](DefaultApi.md#mora_length_mora_length_post)                                                   | **POST** /mora_length                  | アクセント句から音素長を得る                                       |
| [**mora_pitch_mora_pitch_post**](DefaultApi.md#mora_pitch_mora_pitch_post)                                                       | **POST** /mora_pitch                   | アクセント句から音高を得る                                         |
| [**morphable_targets_morphable_targets_post**](DefaultApi.md#morphable_targets_morphable_targets_post)                           | **POST** /morphable_targets            | 指定した話者に対してエンジン内の話者がモーフィングが可能か判定する |
| [**multi_synthesis_multi_synthesis_post**](DefaultApi.md#multi_synthesis_multi_synthesis_post)                                   | **POST** /multi_synthesis              | 複数まとめて音声合成する                                           |
| [**rewrite_user_dict_word_user_dict_word_word_uuid_put**](DefaultApi.md#rewrite_user_dict_word_user_dict_word_word_uuid_put)     | **PUT** /user_dict_word/{word_uuid}    | Rewrite User Dict Word                                             |
| [**setting_get_setting_get**](DefaultApi.md#setting_get_setting_get)                                                             | **GET** /setting                       | Setting Get                                                        |
| [**setting_post_setting_post**](DefaultApi.md#setting_post_setting_post)                                                         | **POST** /setting                      | Setting Post                                                       |
| [**speaker_info_speaker_info_get**](DefaultApi.md#speaker_info_speaker_info_get)                                                 | **GET** /speaker_info                  | Speaker Info                                                       |
| [**speakers_speakers_get**](DefaultApi.md#speakers_speakers_get)                                                                 | **GET** /speakers                      | Speakers                                                           |
| [**supported_devices_supported_devices_get**](DefaultApi.md#supported_devices_supported_devices_get)                             | **GET** /supported_devices             | Supported Devices                                                  |
| [**synthesis_morphing_synthesis_morphing_post**](DefaultApi.md#synthesis_morphing_synthesis_morphing_post)                       | **POST** /synthesis_morphing           | 2人の話者でモーフィングした音声を合成する                          |
| [**synthesis_synthesis_post**](DefaultApi.md#synthesis_synthesis_post)                                                           | **POST** /synthesis                    | 音声合成する                                                       |
| [**update_preset_update_preset_post**](DefaultApi.md#update_preset_update_preset_post)                                           | **POST** /update_preset                | Update Preset                                                      |
| [**version_version_get**](DefaultApi.md#version_version_get)                                                                     | **GET** /version                       | Version                                                            |

## accent_phrases_accent_phrases_post

> Vec<crate::models::AccentPhrase> accent_phrases_accent_phrases_post(text,
> speaker, is_kana, core_version) テキストからアクセント句を得る

テキストからアクセント句を得ます。
is_kanaが`true`のとき、テキストは次のようなAquesTalkライクな記法に従う読み仮名として処理されます。デフォルトは`false`です。 *
全てのカナはカタカナで記述される *
アクセント句は`/`または`、`で区切る。`、`で区切った場合に限り無音区間が挿入される。 *
カナの手前に`_`を入れるとそのカナは無声化される *
アクセント位置を`'`で指定する。全てのアクセント句にはアクセント位置を1つ指定する必要がある。 *
アクセント句末に`？`(全角)を入れることにより疑問文の発音ができる。

### Parameters

| Name             | Type               | Description | Required   | Notes              |
| ---------------- | ------------------ | ----------- | ---------- | ------------------ |
| **text**         | **String**         |             | [required] |                    |
| **speaker**      | **i32**            |             | [required] |                    |
| **is_kana**      | Option<**bool**>   |             |            | [default to false] |
| **core_version** | Option<**String**> |             |            |                    |

### Return type

[**Vec<crate::models::AccentPhrase>**](AccentPhrase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## add_preset_add_preset_post

> i32 add_preset_add_preset_post(preset) Add Preset

新しいプリセットを追加します Parameters ------- preset: Preset
新しいプリセット。
プリセットIDが既存のものと重複している場合は、新規のプリセットIDが採番されます。
Returns ------- id: int 追加したプリセットのプリセットID

### Parameters

| Name       | Type                    | Description | Required   | Notes |
| ---------- | ----------------------- | ----------- | ---------- | ----- |
| **preset** | [**Preset**](Preset.md) |             | [required] |       |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## add_user_dict_word_user_dict_word_post

> String add_user_dict_word_user_dict_word_post(surface, pronunciation,
> accent_type, word_type, priority) Add User Dict Word

ユーザー辞書に言葉を追加します。 Parameters ---------- surface : str
言葉の表層形 pronunciation: str 言葉の発音（カタカナ） accent_type: int
アクセント型（音が下がる場所を指す） word_type: WordTypes, optional
PROPER_NOUN（固有名詞）、COMMON_NOUN（普通名詞）、VERB（動詞）、ADJECTIVE（形容詞）、SUFFIX（語尾）のいずれか
priority: int, optional 単語の優先度（0から10までの整数）
数字が大きいほど優先度が高くなる 1から9までの値を指定することを推奨

### Parameters

| Name              | Type                                        | Description | Required   | Notes |
| ----------------- | ------------------------------------------- | ----------- | ---------- | ----- |
| **surface**       | **String**                                  |             | [required] |       |
| **pronunciation** | **String**                                  |             | [required] |       |
| **accent_type**   | **i32**                                     |             | [required] |       |
| **word_type**     | Option<[**crate::models::WordTypes**](.md)> |             |            |       |
| **priority**      | Option<**i32**>                             |             |            |       |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## audio_query_audio_query_post

> crate::models::AudioQuery audio_query_audio_query_post(text, speaker,
> core_version) 音声合成用のクエリを作成する

クエリの初期値を得ます。ここで得られたクエリはそのまま音声合成に利用できます。各値の意味は`Schemas`を参照してください。

### Parameters

| Name             | Type               | Description | Required   | Notes |
| ---------------- | ------------------ | ----------- | ---------- | ----- |
| **text**         | **String**         |             | [required] |       |
| **speaker**      | **i32**            |             | [required] |       |
| **core_version** | Option<**String**> |             |            |       |

### Return type

[**crate::models::AudioQuery**](AudioQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## audio_query_from_preset_audio_query_from_preset_post

> crate::models::AudioQuery
> audio_query_from_preset_audio_query_from_preset_post(text, preset_id,
> core_version) 音声合成用のクエリをプリセットを用いて作成する

クエリの初期値を得ます。ここで得られたクエリはそのまま音声合成に利用できます。各値の意味は`Schemas`を参照してください。

### Parameters

| Name             | Type               | Description | Required   | Notes |
| ---------------- | ------------------ | ----------- | ---------- | ----- |
| **text**         | **String**         |             | [required] |       |
| **preset_id**    | **i32**            |             | [required] |       |
| **core_version** | Option<**String**> |             |            |       |

### Return type

[**crate::models::AudioQuery**](AudioQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## cancellable_synthesis_cancellable_synthesis_post

> std::path::PathBuf cancellable_synthesis_cancellable_synthesis_post(speaker,
> audio_query, core_version) 音声合成する（キャンセル可能）

### Parameters

| Name             | Type                            | Description | Required   | Notes |
| ---------------- | ------------------------------- | ----------- | ---------- | ----- |
| **speaker**      | **i32**                         |             | [required] |       |
| **audio_query**  | [**AudioQuery**](AudioQuery.md) |             | [required] |       |
| **core_version** | Option<**String**>              |             |            |       |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: audio/wav, application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## connect_waves_connect_waves_post

> std::path::PathBuf connect_waves_connect_waves_post(request_body)
> base64エンコードされた複数のwavデータを一つに結合する

base64エンコードされたwavデータを一纏めにし、wavファイルで返します。

### Parameters

| Name             | Type                         | Description | Required   | Notes |
| ---------------- | ---------------------------- | ----------- | ---------- | ----- |
| **request_body** | [**Vec<String>**](String.md) |             | [required] |       |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: audio/wav, application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## core_versions_core_versions_get

> Vec<String> core_versions_core_versions_get() Core Versions

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## delete_preset_delete_preset_post

> delete_preset_delete_preset_post(id) Delete Preset

既存のプリセットを削除します Parameters ------- id: int
削除するプリセットのプリセットID

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## delete_user_dict_word_user_dict_word_word_uuid_delete

> delete_user_dict_word_user_dict_word_word_uuid_delete(word_uuid) Delete User
> Dict Word

ユーザー辞書に登録されている言葉を削除します。 Parameters ---------- word_uuid:
str 削除する言葉のUUID

### Parameters

| Name          | Type       | Description | Required   | Notes |
| ------------- | ---------- | ----------- | ---------- | ----- |
| **word_uuid** | **String** |             | [required] |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## downloadable_libraries_downloadable_libraries_get

> Vec<crate::models::DownloadableLibrary>
> downloadable_libraries_downloadable_libraries_get() Downloadable Libraries

ダウンロード可能なモデル情報を返します。 Returns ------- ret_data:
List[DownloadableLibrary]

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DownloadableLibrary>**](DownloadableLibrary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## engine_manifest_engine_manifest_get

> crate::models::EngineManifest engine_manifest_engine_manifest_get() Engine
> Manifest

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EngineManifest**](EngineManifest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## get_presets_presets_get

> Vec<crate::models::Preset> get_presets_presets_get() Get Presets

エンジンが保持しているプリセットの設定を返します Returns ------- presets:
List[Preset] プリセットのリスト

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Preset>**](Preset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## get_user_dict_words_user_dict_get

> ::std::collections::HashMap<String, crate::models::UserDictWord>
> get_user_dict_words_user_dict_get() Get User Dict Words

ユーザー辞書に登録されている単語の一覧を返します。
単語の表層形(surface)は正規化済みの物を返します。 Returns ------- Dict[str,
UserDictWord] 単語のUUIDとその詳細

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::UserDictWord>**](UserDictWord.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## import_user_dict_words_import_user_dict_post

> import_user_dict_words_import_user_dict_post(_override, request_body) Import
> User Dict Words

他のユーザー辞書をインポートします。 Parameters ---------- import_dict_data:
Dict[str, UserDictWord] インポートするユーザー辞書のデータ override: bool
重複したエントリがあった場合、上書きするかどうか

### Parameters

| Name             | Type                                                                                    | Description | Required   | Notes |
| ---------------- | --------------------------------------------------------------------------------------- | ----------- | ---------- | ----- |
| **_override**    | **bool**                                                                                |             | [required] |       |
| **request_body** | [**::std::collections::HashMap<String, crate::models::UserDictWord>**](UserDictWord.md) |             | [required] |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## initialize_speaker_initialize_speaker_post

> initialize_speaker_initialize_speaker_post(speaker, skip_reinit, core_version)
> Initialize Speaker

指定されたspeaker_idの話者を初期化します。
実行しなくても他のAPIは使用できますが、初回実行時に時間がかかることがあります。

### Parameters

| Name             | Type               | Description                                          | Required   | Notes              |
| ---------------- | ------------------ | ---------------------------------------------------- | ---------- | ------------------ |
| **speaker**      | **i32**            |                                                      | [required] |                    |
| **skip_reinit**  | Option<**bool**>   | 既に初期化済みの話者の再初期化をスキップするかどうか |            | [default to false] |
| **core_version** | Option<**String**> |                                                      |            |                    |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## is_initialized_speaker_is_initialized_speaker_get

> bool is_initialized_speaker_is_initialized_speaker_get(speaker, core_version)
> Is Initialized Speaker

指定されたspeaker_idの話者が初期化されているかどうかを返します。

### Parameters

| Name             | Type               | Description | Required   | Notes |
| ---------------- | ------------------ | ----------- | ---------- | ----- |
| **speaker**      | **i32**            |             | [required] |       |
| **core_version** | Option<**String**> |             |            |       |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## mora_data_mora_data_post

> Vec<crate::models::AccentPhrase> mora_data_mora_data_post(speaker,
> accent_phrase, core_version) アクセント句から音高・音素長を得る

### Parameters

| Name              | Type                                                    | Description | Required   | Notes |
| ----------------- | ------------------------------------------------------- | ----------- | ---------- | ----- |
| **speaker**       | **i32**                                                 |             | [required] |       |
| **accent_phrase** | [**Vec<crate::models::AccentPhrase>**](AccentPhrase.md) |             | [required] |       |
| **core_version**  | Option<**String**>                                      |             |            |       |

### Return type

[**Vec<crate::models::AccentPhrase>**](AccentPhrase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## mora_length_mora_length_post

> Vec<crate::models::AccentPhrase> mora_length_mora_length_post(speaker,
> accent_phrase, core_version) アクセント句から音素長を得る

### Parameters

| Name              | Type                                                    | Description | Required   | Notes |
| ----------------- | ------------------------------------------------------- | ----------- | ---------- | ----- |
| **speaker**       | **i32**                                                 |             | [required] |       |
| **accent_phrase** | [**Vec<crate::models::AccentPhrase>**](AccentPhrase.md) |             | [required] |       |
| **core_version**  | Option<**String**>                                      |             |            |       |

### Return type

[**Vec<crate::models::AccentPhrase>**](AccentPhrase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## mora_pitch_mora_pitch_post

> Vec<crate::models::AccentPhrase> mora_pitch_mora_pitch_post(speaker,
> accent_phrase, core_version) アクセント句から音高を得る

### Parameters

| Name              | Type                                                    | Description | Required   | Notes |
| ----------------- | ------------------------------------------------------- | ----------- | ---------- | ----- |
| **speaker**       | **i32**                                                 |             | [required] |       |
| **accent_phrase** | [**Vec<crate::models::AccentPhrase>**](AccentPhrase.md) |             | [required] |       |
| **core_version**  | Option<**String**>                                      |             |            |       |

### Return type

[**Vec<crate::models::AccentPhrase>**](AccentPhrase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## morphable_targets_morphable_targets_post

> Vec<::std::collections::HashMap<String, crate::models::MorphableTargetInfo>>
> morphable_targets_morphable_targets_post(request_body, core_version)
> 指定した話者に対してエンジン内の話者がモーフィングが可能か判定する

指定されたベース話者に対してエンジン内の各話者がモーフィング機能を利用可能か返します。
モーフィングの許可/禁止は`/speakers`の`speaker.supported_features.synthesis_morphing`に記載されています。
プロパティが存在しない場合は、モーフィングが許可されているとみなします。
返り値の話者はstring型なので注意。

### Parameters

| Name             | Type                   | Description | Required   | Notes |
| ---------------- | ---------------------- | ----------- | ---------- | ----- |
| **request_body** | [**Vec<i32>**](i32.md) |             | [required] |       |
| **core_version** | Option<**String**>     |             |            |       |

### Return type

[**Vec<::std::collections::HashMap<String, crate::models::MorphableTargetInfo>>**](map.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## multi_synthesis_multi_synthesis_post

> std::path::PathBuf multi_synthesis_multi_synthesis_post(speaker, audio_query,
> core_version) 複数まとめて音声合成する

### Parameters

| Name             | Type                                                | Description | Required   | Notes |
| ---------------- | --------------------------------------------------- | ----------- | ---------- | ----- |
| **speaker**      | **i32**                                             |             | [required] |       |
| **audio_query**  | [**Vec<crate::models::AudioQuery>**](AudioQuery.md) |             | [required] |       |
| **core_version** | Option<**String**>                                  |             |            |       |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/zip, application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## rewrite_user_dict_word_user_dict_word_word_uuid_put

> rewrite_user_dict_word_user_dict_word_word_uuid_put(word_uuid, surface,
> pronunciation, accent_type, word_type, priority) Rewrite User Dict Word

ユーザー辞書に登録されている言葉を更新します。 Parameters ---------- surface :
str 言葉の表層形 pronunciation: str 言葉の発音（カタカナ） accent_type: int
アクセント型（音が下がる場所を指す） word_uuid: str 更新する言葉のUUID
word_type: WordTypes, optional
PROPER_NOUN（固有名詞）、COMMON_NOUN（普通名詞）、VERB（動詞）、ADJECTIVE（形容詞）、SUFFIX（語尾）のいずれか
priority: int, optional 単語の優先度（0から10までの整数）
数字が大きいほど優先度が高くなる 1から9までの値を指定することを推奨

### Parameters

| Name              | Type                                        | Description | Required   | Notes |
| ----------------- | ------------------------------------------- | ----------- | ---------- | ----- |
| **word_uuid**     | **String**                                  |             | [required] |       |
| **surface**       | **String**                                  |             | [required] |       |
| **pronunciation** | **String**                                  |             | [required] |       |
| **accent_type**   | **i32**                                     |             | [required] |       |
| **word_type**     | Option<[**crate::models::WordTypes**](.md)> |             |            |       |
| **priority**      | Option<**i32**>                             |             |            |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## setting_get_setting_get

> String setting_get_setting_get() Setting Get

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## setting_post_setting_post

> String setting_post_setting_post(cors_policy_mode, allow_origin) Setting Post

### Parameters

| Name                 | Type               | Description | Required | Notes |
| -------------------- | ------------------ | ----------- | -------- | ----- |
| **cors_policy_mode** | Option<**String**> |             |          |       |
| **allow_origin**     | Option<**String**> |             |          |       |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/html, application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## speaker_info_speaker_info_get

> crate::models::SpeakerInfo speaker_info_speaker_info_get(speaker_uuid,
> core_version) Speaker Info

指定されたspeaker_uuidに関する情報をjson形式で返します。
画像や音声はbase64エンコードされたものが返されます。 Returns ------- ret_data:
SpeakerInfo

### Parameters

| Name             | Type               | Description | Required   | Notes |
| ---------------- | ------------------ | ----------- | ---------- | ----- |
| **speaker_uuid** | **String**         |             | [required] |       |
| **core_version** | Option<**String**> |             |            |       |

### Return type

[**crate::models::SpeakerInfo**](SpeakerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## speakers_speakers_get

> Vec<crate::models::Speaker> speakers_speakers_get(core_version) Speakers

### Parameters

| Name             | Type               | Description | Required | Notes |
| ---------------- | ------------------ | ----------- | -------- | ----- |
| **core_version** | Option<**String**> |             |          |       |

### Return type

[**Vec<crate::models::Speaker>**](Speaker.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## supported_devices_supported_devices_get

> crate::models::SupportedDevicesInfo
> supported_devices_supported_devices_get(core_version) Supported Devices

### Parameters

| Name             | Type               | Description | Required | Notes |
| ---------------- | ------------------ | ----------- | -------- | ----- |
| **core_version** | Option<**String**> |             |          |       |

### Return type

[**crate::models::SupportedDevicesInfo**](SupportedDevicesInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## synthesis_morphing_synthesis_morphing_post

> std::path::PathBuf synthesis_morphing_synthesis_morphing_post(base_speaker,
> target_speaker, morph_rate, audio_query, core_version)
> 2人の話者でモーフィングした音声を合成する

指定された2人の話者で音声を合成、指定した割合でモーフィングした音声を得ます。
モーフィングの割合は`morph_rate`で指定でき、0.0でベースの話者、1.0でターゲットの話者に近づきます。

### Parameters

| Name               | Type                            | Description | Required   | Notes |
| ------------------ | ------------------------------- | ----------- | ---------- | ----- |
| **base_speaker**   | **i32**                         |             | [required] |       |
| **target_speaker** | **i32**                         |             | [required] |       |
| **morph_rate**     | **f32**                         |             | [required] |       |
| **audio_query**    | [**AudioQuery**](AudioQuery.md) |             | [required] |       |
| **core_version**   | Option<**String**>              |             |            |       |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: audio/wav, application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## synthesis_synthesis_post

> std::path::PathBuf synthesis_synthesis_post(speaker, audio_query,
> enable_interrogative_upspeak, core_version) 音声合成する

### Parameters

| Name                             | Type                            | Description                                      | Required   | Notes             |
| -------------------------------- | ------------------------------- | ------------------------------------------------ | ---------- | ----------------- |
| **speaker**                      | **i32**                         |                                                  | [required] |                   |
| **audio_query**                  | [**AudioQuery**](AudioQuery.md) |                                                  | [required] |                   |
| **enable_interrogative_upspeak** | Option<**bool**>                | 疑問系のテキストが与えられたら語尾を自動調整する |            | [default to true] |
| **core_version**                 | Option<**String**>              |                                                  |            |                   |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: audio/wav, application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## update_preset_update_preset_post

> i32 update_preset_update_preset_post(preset) Update Preset

既存のプリセットを更新します Parameters ------- preset: Preset
更新するプリセット。 プリセットIDが更新対象と一致している必要があります。
Returns ------- id: int 更新したプリセットのプリセットID

### Parameters

| Name       | Type                    | Description | Required   | Notes |
| ---------- | ----------------------- | ----------- | ---------- | ----- |
| **preset** | [**Preset**](Preset.md) |             | [required] |       |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

## version_version_get

> serde_json::Value version_version_get() Version

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
