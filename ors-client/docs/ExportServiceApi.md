# \ExportServiceApi

All URIs are relative to *https://api.openrouteservice.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default2**](ExportServiceApi.md#get_default2) | **POST** /v2/export/{profile} | Export Service
[**get_json_export**](ExportServiceApi.md#get_json_export) | **POST** /v2/export/{profile}/json | Export Service JSON
[**get_topo_json_export**](ExportServiceApi.md#get_topo_json_export) | **POST** /v2/export/{profile}/topojson | Export Service TopoJSON



## get_default2

> models::GetDefault2200Response get_default2(profile, graph_export_service)
Export Service

Returns a list of points, edges and weights within a given bounding box for a selected profile as JSON. This method does not accept any request body or parameters other than profile, start coordinate, and end coordinate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the route profile. | [required] |
**graph_export_service** | [**GraphExportService**](GraphExportService.md) |  | [required] |

### Return type

[**models::GetDefault2200Response**](getDefault_2_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/geo+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_json_export

> models::GetDefault2200Response get_json_export(profile, graph_export_service)
Export Service JSON

Returns a list of points, edges and weights within a given bounding box for a selected profile as JSON.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the profile. | [required] |
**graph_export_service** | [**GraphExportService**](GraphExportService.md) |  | [required] |

### Return type

[**models::GetDefault2200Response**](getDefault_2_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/geo+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_topo_json_export

> models::GetTopoJsonExport200Response get_topo_json_export(profile, graph_export_service)
Export Service TopoJSON

Returns a list of edges, edge properties, and their topology within a given bounding box for a selected profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** | Specifies the profile. | [required] |
**graph_export_service** | [**GraphExportService**](GraphExportService.md) |  | [required] |

### Return type

[**models::GetTopoJsonExport200Response**](getTopoJsonExport_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

