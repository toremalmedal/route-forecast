# AlternativeRoutes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_count** | Option<**i32**> | Target number of alternative routes to compute. Service returns up to this number of routes that fulfill the share-factor and weight-factor constraints. | [optional]
**weight_factor** | Option<**f64**> | Maximum factor by which route weight may diverge from the optimal route. The default value of 1.4 means alternatives can be up to 1.4 times longer (costly) than the optimal route. | [optional]
**share_factor** | Option<**f64**> | Maximum fraction of the route that alternatives may share with the optimal route. The default value of 0.6 means alternatives can share up to 60% of path segments with the optimal route. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


