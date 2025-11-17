# OptimizationPostRequestVehiclesInnerStepsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | step type (either start, job, pickup, delivery, break or end)]  | [optional]
**id** | Option<**f64**> | id of the task to be performed at this step if `type` value is not `start` or `end`  | [optional]
**service_at** | Option<**f64**> | hard constraint on service time (as absolute or relative timestamp)  | [optional]
**service_after** | Option<**f64**> | hard constraint on service time lower bound (as absolute or relative timestamp)  | [optional]
**service_before** | Option<**f64**> | hard constraint on service time upper bound (as absolute or relative timestamp)  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


