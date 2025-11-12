# GetDefault1Request

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locations** | [**Vec<Vec<f64>>**](Vec.md) | List of comma separated lists of `longitude,latitude` coordinates in WGS 84 (EPSG:4326) | 
**id** | Option<**String**> | Arbitrary identification string of the request reflected in the meta information. | [optional]
**sources** | Option<**Vec<String>**> | A list of indices that refers to the list of locations (starting with `0`). `{index_1},{index_2}[,{index_N} ...]` or `all` (default). example `[0,3]` for the first and fourth locations  | [optional][default to ["all"]]
**destinations** | Option<**Vec<String>**> | A list of indices that refers to the list of locations (starting with `0`). `{index_1},{index_2}[,{index_N} ...]` or `all` (default). `[0,3]` for the first and fourth locations  | [optional][default to ["all"]]
**metrics** | Option<**Vec<String>**> | Specifies a list of returned metrics. \"* `distance` - Returns distance matrix for specified points in defined `units`. * `duration` - Returns duration matrix for specified points in **seconds**.  | [optional][default to ["duration"]]
**resolve_locations** | Option<**bool**> | Specifies whether given locations are resolved or not. If the parameter value set to `true`, every element in `destinations` and `sources` will contain a `name` element that identifies the name of the closest street. Default is `false`.  | [optional][default to false]
**units** | Option<**String**> | Specifies the distance unit. Default: m. | [optional][default to M]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


