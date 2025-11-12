# RouteRequestCustomModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**distance_influence** | Option<**f64**> | Parameter determining the influence of the distance between two points on the edge weight | [optional]
**speed** | Option<[**Vec<models::DirectionsServiceCustomModelSpeedInner>**](Directions_Service_custom_model_speed_inner.md)> | Array of objects describing rules to be applied to the speed of edges | [optional]
**priority** | Option<[**Vec<models::DirectionsServiceCustomModelSpeedInner>**](Directions_Service_custom_model_speed_inner.md)> | Array of objects describing rules to be applied to the priority of edges | [optional]
**areas** | Option<[**std::collections::HashMap<String, models::DirectionsServiceCustomModelAreasValue>**](Directions_Service_custom_model_areas_value.md)> | Map of areas that can be referenced in speed and priority rules | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


