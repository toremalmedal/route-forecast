# OptimizationPost200ResponseRoutesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vehicle** | Option<**i32**> | id of the vehicle assigned to this route | [optional]
**steps** | Option<[**Vec<models::OptimizationPost200ResponseRoutesInnerStepsInner>**](_optimization_post_200_response_routes_inner_steps_inner.md)> | array of `step` objects | [optional]
**cost** | Option<**f32**> | cost for this route | [optional]
**service** | Option<**f32**> | total service time for this route | [optional]
**duration** | Option<**f32**> | total travel time for this route | [optional]
**waiting_time** | Option<**f32**> | total waiting time for this route | [optional]
**delivery** | Option<**Vec<i32>**> | Total delivery for tasks in this route | [optional]
**pickup** | Option<**Vec<i32>**> | total pickup for tasks in this route | [optional]
**description** | Option<**String**> | vehicle description, if provided in input  | [optional]
**geometry** | Option<**String**> | polyline encoded route geometry. Only provided when using the `-g` flag | [optional]
**distance** | Option<**f32**> | total route distance. Only provided when using the `-g` flag | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


