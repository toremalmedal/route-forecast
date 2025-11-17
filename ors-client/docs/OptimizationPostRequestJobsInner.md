# OptimizationPostRequestJobsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | an integer used as unique identifier  | [optional]
**description** | Option<**String**> | a string describing this job  | [optional]
**location** | Option<[**Vec<Vec<f32>>**](Vec.md)> | coordinates array in `[lon, lat]`  | [optional]
**location_index** | Option<[**serde_json::Value**](.md)> | index of relevant row and column in custom matrix  | [optional]
**setup** | Option<**f64**> | job setup duration (defaults to 0), in seconds  | [optional]
**service** | Option<[**serde_json::Value**](.md)> | job service duration (defaults to 0), in seconds  | [optional]
**delivery** | Option<**Vec<f64>**> | an array of integers describing multidimensional quantities for delivery  | [optional]
**pickup** | Option<**Vec<f64>**> | an array of integers describing multidimensional quantities for pickup  | [optional]
**skills** | Option<**Vec<i32>**> | Array of integers defining mandatory skills for this job  | [optional]
**priority** | Option<**f64**> | an integer in the range [0, 100] describing priority level (defaults to 0)  | [optional][default to 0]
**time_windows** | Option<[**Vec<Vec<i32>>**](Vec.md)> | Array of `time_window` arrays describing valid slots for job service start and end, in week seconds, i.e. 28800 = Mon, 8 AM.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


