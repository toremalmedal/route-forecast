# \DataApi

All URIs are relative to *https://api.met.no/weatherapi/locationforecast/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**classic_format_get**](DataApi.md#classic_format_get) | **GET** /classic.{format} | 
[**classic_get**](DataApi.md#classic_get) | **GET** /classic | 
[**compact_format_get**](DataApi.md#compact_format_get) | **GET** /compact.{format} | 
[**compact_get**](DataApi.md#compact_get) | **GET** /compact | 
[**complete_format_get**](DataApi.md#complete_format_get) | **GET** /complete.{format} | 
[**complete_get**](DataApi.md#complete_get) | **GET** /complete | 
[**status_format_get**](DataApi.md#status_format_get) | **GET** /status.{format} | 
[**status_get**](DataApi.md#status_get) | **GET** /status | 



## classic_format_get

> String classic_format_get(lat, lon, format, altitude)


Weather forecasts for any location on earth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f32** | Latitude | [required] |
**lon** | **f32** | Longitude | [required] |
**format** | **String** | format code (file extension) | [required] |
**altitude** | Option<**i32**> | Whole meters above sea level |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## classic_get

> String classic_get(lat, lon, altitude)


Weather forecasts for any location on earth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f32** | Latitude | [required] |
**lon** | **f32** | Longitude | [required] |
**altitude** | Option<**i32**> | Whole meters above sea level |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compact_format_get

> models::MetjsonForecast compact_format_get(lat, lon, format, altitude)


Weather forecasts for any location on earth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f32** | Latitude | [required] |
**lon** | **f32** | Longitude | [required] |
**format** | **String** | format code (file extension) | [required] |
**altitude** | Option<**i32**> | Whole meters above sea level |  |

### Return type

[**models::MetjsonForecast**](METJSONForecast.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compact_get

> models::MetjsonForecast compact_get(lat, lon, altitude)


Weather forecasts for any location on earth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f32** | Latitude | [required] |
**lon** | **f32** | Longitude | [required] |
**altitude** | Option<**i32**> | Whole meters above sea level |  |

### Return type

[**models::MetjsonForecast**](METJSONForecast.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_format_get

> models::MetjsonForecast complete_format_get(lat, lon, format, altitude)


Weather forecasts for any location on earth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f32** | Latitude | [required] |
**lon** | **f32** | Longitude | [required] |
**format** | **String** | format code (file extension) | [required] |
**altitude** | Option<**i32**> | Whole meters above sea level |  |

### Return type

[**models::MetjsonForecast**](METJSONForecast.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_get

> models::MetjsonForecast complete_get(lat, lon, altitude)


Weather forecasts for any location on earth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f32** | Latitude | [required] |
**lon** | **f32** | Longitude | [required] |
**altitude** | Option<**i32**> | Whole meters above sea level |  |

### Return type

[**models::MetjsonForecast**](METJSONForecast.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_format_get

> String status_format_get(format)


Weather forecasts for any location on earth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** | format code (file extension) | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_get

> String status_get()


Weather forecasts for any location on earth

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

