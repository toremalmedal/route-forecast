# OptimizationPostRequestVehiclesInnerBreaksInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Integer used as unique identifier  | [optional]
**time_windows** | Option<[**Vec<Vec<i32>>**](Vec.md)> | Array of time_window objects describing valid slots for break start and end, in week seconds, i.e. 28800 = Mon, 8 AM.  | [optional]
**service** | Option<**f64**> | break duration in seconds (defaults to 0)  | [optional][default to 0]
**description** | Option<**String**> | a string describing this break  | [optional]
**max_load** | Option<**Vec<f64>**> | Array of integers describing the maximum vehicle load for which this break can happen. An error is reported if two break objects have the same id for the same vehicle.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


