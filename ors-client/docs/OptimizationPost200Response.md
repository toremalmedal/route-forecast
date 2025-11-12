# OptimizationPost200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**i32**> | status code. Possible values:   Value         | Status |  :-----------: | :-----------: |  `0` | no error raised |  `1` | internal error |  `2` | input error |  `3` | routing error |  | [optional]
**error** | Option<**String**> | error message (present if `code` is different from `0`)  | [optional]
**summary** | Option<[**models::OptimizationPost200ResponseSummary**](_optimization_post_200_response_summary.md)> |  | [optional]
**unassigned** | Option<[**Vec<models::OptimizationPost200ResponseUnassignedInner>**](_optimization_post_200_response_unassigned_inner.md)> | array of objects describing unassigned jobs with their `id` and `location` (if provided)  | [optional]
**routes** | Option<[**Vec<models::OptimizationPost200ResponseRoutesInner>**](_optimization_post_200_response_routes_inner.md)> | array of `route` objects  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


