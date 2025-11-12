# \MatchingServiceApi

All URIs are relative to *https://api.openrouteservice.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_matching**](MatchingServiceApi.md#get_matching) | **GET** /v2/match/{profile} | Matching Service information
[**post_matching**](MatchingServiceApi.md#post_matching) | **POST** /v2/match/{profile} | Matching Service



## get_matching

> serde_json::Value get_matching(profile)
Matching Service information

Provide information about the graph used for matching.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_matching

> serde_json::Value post_matching(profile, post_matching_request)
Matching Service

Matches point, linestring and polygon geometries to edge IDs of the graph. Note that matchings are invalidated when rebuilding the graph because the edge IDs may change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |
**post_matching_request** | [**PostMatchingRequest**](PostMatchingRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

