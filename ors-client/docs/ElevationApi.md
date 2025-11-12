# \ElevationApi

All URIs are relative to *https://api.openrouteservice.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**elevation_line_post**](ElevationApi.md#elevation_line_post) | **POST** /elevation/line | Elevation Line Service
[**elevation_point_get**](ElevationApi.md#elevation_point_get) | **GET** /elevation/point | Elevation Point Service
[**elevation_point_post**](ElevationApi.md#elevation_point_post) | **POST** /elevation/point | Elevation Point Service



## elevation_line_post

> models::ElevationLinePost200Response elevation_line_post(body)
Elevation Line Service

This endpoint can take planar 2D line objects and enrich them with elevation from a variety of datasets.  The input and output formats are:   * GeoJSON   * Polyline   * <a href=\"https://developers.google.com/maps/documentation/utilities/polylinealgorithm\">Google's Encoded polyline</a> with coordinate precision 5 or 6  Example: ```   # POST LineString as polyline   curl -XPOST https://api.openrouteservice.org/elevation/line     -H 'Content-Type: application/json' \\     -H 'Authorization: INSERT_YOUR_KEY     -d '{       \"format_in\": \"polyline\",       \"format_out\": \"encodedpolyline5\",       \"geometry\": [[13.349762, 38.112952],                    [12.638397, 37.645772]]         }' ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ElevationLinePostRequest**](ElevationLinePostRequest.md) | Query the elevation of a line in various formats. | [required] |

### Return type

[**models::ElevationLinePost200Response**](_elevation_line_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## elevation_point_get

> models::ElevationPointGet200Response elevation_point_get(api_key, geometry, format_out, dataset)
Elevation Point Service

This endpoint can take a 2D point and enrich it with elevation from a variety of datasets.  The output formats are:   * GeoJSON   * Point  Example: ```   # GET point   curl -XGET https://localhost:5000/elevation/point?geometry=13.349762,38.11295 ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | Insert your API Key here.  | [required] |
**geometry** | [**Vec<f64>**](f64.md) | The point to be queried, in comma-separated lon,lat values, e.g. [13.349762, 38.11295] | [required] |
**format_out** | Option<**String**> | The output format to be returned. |  |[default to geojson]
**dataset** | Option<**String**> | The elevation dataset to be used. |  |[default to srtm]

### Return type

[**models::ElevationPointGet200Response**](_elevation_point_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## elevation_point_post

> models::ElevationPointGet200Response elevation_point_post(body)
Elevation Point Service

This endpoint can take a 2D point and enrich it with elevation from a variety of datasets.  The input and output formats are:   * GeoJSON   * Point  Example: ```   # POST point as GeoJSON   # https://api.openrouteservice.org/elevation/point?api_key=YOUR-KEY   {     \"format_in\": \"geojson\",     \"format_out\": \"geojson\",     \"geometry\": {       \"coordinates\": [13.349762, 38.11295],       \"type\": \"Point\"     }   } ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ElevationPointPostRequest**](ElevationPointPostRequest.md) | Query the elevation of a point in various formats. | [required] |

### Return type

[**models::ElevationPointGet200Response**](_elevation_point_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

