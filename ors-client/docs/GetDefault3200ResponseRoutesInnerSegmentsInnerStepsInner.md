# GetDefault3200ResponseRoutesInnerSegmentsInnerStepsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**distance** | Option<**f64**> | The distance for the step in metres. | [optional]
**duration** | Option<**f64**> | The duration for the step in seconds. | [optional]
**r#type** | Option<**i32**> | The [instruction](https://giscience.github.io/openrouteservice/api-reference/endpoints/directions/instruction-types) action for symbolisation purposes. | [optional]
**instruction** | Option<**String**> | The routing instruction text for the step. | [optional]
**name** | Option<**String**> | The name of the next street. | [optional]
**exit_number** | Option<**i32**> | Only for roundabouts. Contains the number of the exit to take. | [optional]
**exit_bearings** | Option<**Vec<i32>**> | Contains the bearing of the entrance and all passed exits in a roundabout. | [optional]
**way_points** | Option<**Vec<i32>**> | List containing the indices of the steps start- and endpoint corresponding to the *geometry*. | [optional]
**maneuver** | Option<[**models::GetDefault3200ResponseRoutesInnerSegmentsInnerStepsInnerManeuver**](getDefault_3_200_response_routes_inner_segments_inner_steps_inner_maneuver.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


