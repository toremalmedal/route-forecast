# OptimizationPost200ResponseSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cost** | Option<**f64**> | total cost for all routes | [optional]
**routes** | Option<**f64**> | Number of routes in the solution  | [optional]
**unassigned** | Option<**i32**> | number of jobs that could not be served | [optional]
**setup** | Option<**f64**> | Total setup time for all routes  | [optional]
**service** | Option<**f32**> | total service time for all routes | [optional]
**duration** | Option<**f32**> | total travel time for all routes | [optional]
**waiting_time** | Option<**f32**> | total waiting time for all routes | [optional]
**priority** | Option<**f64**> | total priority sum for all assigned tasks | [optional]
**violations** | Option<[**Vec<models::OptimizationPost200ResponseSummaryViolationsInner>**](_optimization_post_200_response_summary_violations_inner.md)> | array of violation objects for all routes | [optional]
**delivery** | Option<**f64**> | Total delivery for all routes | [optional]
**pickup** | Option<**f64**> | Total pickup for all routes | [optional]
**distance** | Option<**f32**> | total distance for all routes. Only provided when using the `-g` flag with `OSRM` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


