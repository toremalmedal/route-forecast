# \SnappingServiceApi

All URIs are relative to *https://api.openrouteservice.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default**](SnappingServiceApi.md#get_default) | **POST** /v2/snap/{profile} | Snapping Service
[**get_geo_json_snapping**](SnappingServiceApi.md#get_geo_json_snapping) | **POST** /v2/snap/{profile}/geojson | Snapping Service GeoJSON
[**get_json_snapping**](SnappingServiceApi.md#get_json_snapping) | **POST** /v2/snap/{profile}/json | Snapping Service JSON



## get_default

> models::GetDefault200Response get_default(profile, get_default_request)
Snapping Service

Returns a list of points snapped to the nearest edge in the routing graph. In case an appropriate snapping point cannot be found within the specified search radius, \"null\" is returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |
**get_default_request** | [**GetDefaultRequest**](GetDefaultRequest.md) |  | [required] |

### Return type

[**models::GetDefault200Response**](getDefault_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_geo_json_snapping

> models::GetGeoJsonSnapping200Response get_geo_json_snapping(profile, get_default_request)
Snapping Service GeoJSON

Returns a GeoJSON FeatureCollection of points snapped to the nearest edge in the routing graph. In case an appropriate snapping point cannot be found within the specified search radius, it is omitted from the features array. The features provide the 'source_id' property, to match the results with the input location array (IDs start at 0). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the profile. | [required] |
**get_default_request** | [**GetDefaultRequest**](GetDefaultRequest.md) |  | [required] |

### Return type

[**models::GetGeoJsonSnapping200Response**](getGeoJSONSnapping_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_json_snapping

> models::GetDefault200Response get_json_snapping(profile, get_default_request)
Snapping Service JSON

Returns a list of points snapped to the nearest edge in the routing graph. In case an appropriate snapping point cannot be found within the specified search radius, \"null\" is returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the profile. | [required] |
**get_default_request** | [**GetDefaultRequest**](GetDefaultRequest.md) |  | [required] |

### Return type

[**models::GetDefault200Response**](getDefault_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

