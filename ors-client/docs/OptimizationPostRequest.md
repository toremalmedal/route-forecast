# OptimizationPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jobs** | [**Vec<models::OptimizationPostRequestJobsInner>**](_optimization_post_request_jobs_inner.md) | Array of `job` objects describing the places to visit. For a detailed object description visit the [VROOM api description](https://github.com/VROOM-Project/vroom/blob/master/docs/API.md#jobs)  | 
**vehicles** | [**Vec<models::OptimizationPostRequestVehiclesInner>**](_optimization_post_request_vehicles_inner.md) | Array of `vehicle` objects describing the available vehicles. For a detailed object description visit the [VROOM API description](https://github.com/VROOM-Project/vroom/blob/master/docs/API.md#vehicles)  | 
**matrices** | Option<[**models::OptimizationPostRequestMatrices**](_optimization_post_request_matrices.md)> |  | [optional]
**options** | Option<[**models::OptimizationPostRequestOptions**](_optimization_post_request_options.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


