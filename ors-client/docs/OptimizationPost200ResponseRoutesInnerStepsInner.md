# OptimizationPost200ResponseRoutesInnerStepsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | string that is either `start`, `job` or `end` | [optional]
**arrival** | Option<**f32**> | estimated time of arrival at this step in seconds | [optional]
**duration** | Option<**f32**> | cumulated travel time upon arrival at this step in seconds | [optional]
**setup** | Option<**f64**> | setup time at this step  | [optional]
**service** | Option<**f64**> | service time at this step  | [optional]
**waiting_time** | Option<**f32**> | waiting time upon arrival at this step, only provided if `type` value is `job` | [optional]
**violations** | Option<[**Vec<models::OptimizationPost200ResponseSummaryViolationsInner>**](_optimization_post_200_response_summary_violations_inner.md)> | array of violation objects for this step | [optional]
**description** | Option<**String**> | step description, if provided in input  | [optional]
**location** | Option<**Vec<f32>**> | coordinates array for this step (if provided in input) | [optional]
**id** | Option<**i32**> | id of the task performed at this step, only provided if type value is `job`, `pickup`, `delivery` or `break`  | [optional]
**load** | Option<**i32**> | vehicle load after step completion (with capacity constraints)  | [optional]
**distance** | Option<**f32**> | traveled distance upon arrival at this step. Only provided when using the `-g` flag | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


