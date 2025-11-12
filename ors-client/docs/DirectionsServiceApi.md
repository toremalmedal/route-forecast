# \DirectionsServiceApi

All URIs are relative to *https://api.openrouteservice.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default3**](DirectionsServiceApi.md#get_default3) | **POST** /v2/directions/{profile} | Directions Service
[**get_geo_json_route**](DirectionsServiceApi.md#get_geo_json_route) | **POST** /v2/directions/{profile}/geojson | Directions Service GeoJSON
[**get_gpx_route**](DirectionsServiceApi.md#get_gpx_route) | **POST** /v2/directions/{profile}/gpx | Directions Service GPX
[**get_json_route**](DirectionsServiceApi.md#get_json_route) | **POST** /v2/directions/{profile}/json | Directions Service JSON
[**get_simple_geo_json_route**](DirectionsServiceApi.md#get_simple_geo_json_route) | **GET** /v2/directions/{profile} | Directions Service



## get_default3

> models::GetDefault3200Response get_default3(profile, directions_service)
Directions Service

Returns a route between two or more locations for a selected profile and its settings as JSON

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |
**directions_service** | [**DirectionsService**](DirectionsService.md) |  | [required] |

### Return type

[**models::GetDefault3200Response**](getDefault_3_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_geo_json_route

> models::GetSimpleGeoJsonRoute200Response get_geo_json_route(profile, directions_service)
Directions Service GeoJSON

Returns a route between two or more locations for a selected profile and its settings as GeoJSON

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |
**directions_service** | [**DirectionsService**](DirectionsService.md) |  | [required] |

### Return type

[**models::GetSimpleGeoJsonRoute200Response**](getSimpleGeoJsonRoute_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/geo+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gpx_route

> models::GetGpxRoute200Response get_gpx_route(profile, directions_service)
Directions Service GPX

Returns a route between two or more locations for a selected profile and its settings as GPX. The schema can be found [here](https://raw.githubusercontent.com/GIScience/openrouteservice-schema/main/gpx/v1/ors-gpx.xsd)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |
**directions_service** | [**DirectionsService**](DirectionsService.md) |  | [required] |

### Return type

[**models::GetGpxRoute200Response**](getGPXRoute_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/gpx+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_json_route

> models::GetDefault3200Response get_json_route(profile, directions_service)
Directions Service JSON

Returns a route between two or more locations for a selected profile and its settings as JSON

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |
**directions_service** | [**DirectionsService**](DirectionsService.md) |  | [required] |

### Return type

[**models::GetDefault3200Response**](getDefault_3_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_simple_geo_json_route

> models::GetSimpleGeoJsonRoute200Response get_simple_geo_json_route(profile, start, end)
Directions Service

Get a basic route between two points with the profile provided. Returned response is in GeoJSON format. This method does not accept any request body or parameters other than profile, start coordinate, and end coordinate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |
**start** | **String** | Start coordinate of the route in `longitude,latitude` format. | [required] |
**end** | **String** | Destination coordinate of the route in `longitude,latitude` format. | [required] |

### Return type

[**models::GetSimpleGeoJsonRoute200Response**](getSimpleGeoJsonRoute_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/geo+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

