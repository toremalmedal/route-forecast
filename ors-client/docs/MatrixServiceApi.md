# \MatrixServiceApi

All URIs are relative to *https://api.openrouteservice.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default1**](MatrixServiceApi.md#get_default1) | **POST** /v2/matrix/{profile} | Matrix Service



## get_default1

> models::GetDefault1200Response get_default1(profile, get_default1_request)
Matrix Service

Returns duration or distance matrix for multiple source and destination points. By default a square duration matrix is returned where every point in locations is paired with each other. The result is null if a value can’t be determined.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the matrix profile. | [required] |
**get_default1_request** | [**GetDefault1Request**](GetDefault1Request.md) |  | [required] |

### Return type

[**models::GetDefault1200Response**](getDefault_1_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json;charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

