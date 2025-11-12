# \OptimizationApi

All URIs are relative to *https://api.openrouteservice.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**optimization_post**](OptimizationApi.md#optimization_post) | **POST** /optimization | Optimization Service



## optimization_post

> models::OptimizationPost200Response optimization_post(body)
Optimization Service

The optimization endpoint solves [Vehicle Routing Problems](https://en.wikipedia.org/wiki/Vehicle_routing_problem) and can be used to schedule multiple vehicles and jobs, respecting time windows, capacities and required skills.  This service is based on the excellent [Vroom project](https://github.com/VROOM-Project/vroom). Please also consult its [API documentation](https://github.com/VROOM-Project/vroom/blob/master/docs/API.md).  General Info: - The expected order for all coordinates arrays is `[lon, lat]` - All timings are in seconds - All distances are in meters - A `time_window` object is a pair of timestamps in the form `[start, end]` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OptimizationPostRequest**](OptimizationPostRequest.md) | The request body of the optimization request. | [required] |

### Return type

[**models::OptimizationPost200Response**](_optimization_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

