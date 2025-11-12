# \GeocodeApi

All URIs are relative to *https://api.openrouteservice.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**geocode_autocomplete_get**](GeocodeApi.md#geocode_autocomplete_get) | **GET** /geocode/autocomplete | Geocode Autocomplete Service
[**geocode_reverse_get**](GeocodeApi.md#geocode_reverse_get) | **GET** /geocode/reverse | Reverse Geocode Service
[**geocode_search_get**](GeocodeApi.md#geocode_search_get) | **GET** /geocode/search | Forward Geocode Service
[**geocode_search_structured_get**](GeocodeApi.md#geocode_search_structured_get) | **GET** /geocode/search/structured | Structured Forward Geocode Service (beta)



## geocode_autocomplete_get

> models::GeocodeResponse geocode_autocomplete_get(api_key, text, focus_point_lon, focus_point_lat, boundary_rect_min_lon, boundary_rect_min_lat, boundary_rect_max_lon, boundary_rect_max_lat, boundary_country, sources, layers)
Geocode Autocomplete Service

**Requests should be throttled when using this endpoint!** *Be aware that Responses are asynchronous.* Returns a JSON formatted list of objects corresponding to the search input. `boundary.*`-parameters can be combined if they are overlapping. **The interactivity for this enpoint is experimental!** [Please refer to this external Documentation](https://github.com/pelias/documentation/blob/master/autocomplete.md) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | Insert your API Key here.  | [required] |[default to your-api-key]
**text** | **String** | Name of location, street address or postal code.  | [required] |
**focus_point_lon** | Option<**f32**> | Longitude of the `focus.point`. Specify the focus point to order results by linear distance to this point. Works for up to 100 kilometers distance.  Use with `focus.point.lat`.  |  |
**focus_point_lat** | Option<**f32**> | Latitude of the `focus.point`. Specify the focus point to order results by linear distance to this point. Works for up to 100 kilometers distance. Use with `focus.point.lon`.  |  |
**boundary_rect_min_lon** | Option<**f32**> | Left border of rectangular boundary to narrow results.  |  |
**boundary_rect_min_lat** | Option<**f32**> | Bottom border of rectangular boundary to narrow results.  |  |
**boundary_rect_max_lon** | Option<**f32**> | Right border of rectangular boundary to narrow results.  |  |
**boundary_rect_max_lat** | Option<**f32**> | Top border of rectangular boundary to narrow results.  |  |
**boundary_country** | Option<**String**> | Restrict results to single country. Possible values are [alpha-2 and alpha-3 country codes](https://en.wikipedia.org/wiki/ISO_3166-1). Example: `DEU` or `DE` for Germany.  |  |
**sources** | Option<[**Vec<String>**](String.md)> | Restrict your search to specific sources. Searches all sources by default. You can either use the normal or short name. Sources are [`openstreetmap(osm)`](http://www.openstreetmap.org/), [`openaddresses(oa)`](http://openaddresses.io/), [`whosonfirst(wof)`](https://whosonfirst.org/), [`geonames(gn)`](http://www.geonames.org/).  |  |[default to ["osm","oa","gn","wof"]]
**layers** | Option<[**Vec<String>**](String.md)> | Restrict search to layers (place type). By default all layers are searched.   layer|description|   ----|----|   `venue`|points of interest, businesses, things with walls|   `address`|places with a street address|   `street`|streets,roads,highways|   `neighbourhood`|social communities, neighbourhoods|   `borough`|a local administrative boundary, currently only used for New York City|   `localadmin`|local administrative boundaries|   `locality`|towns, hamlets, cities|   `county`|official governmental area; usually bigger than a locality, almost always smaller than a region|   `macrocounty`|a related group of counties. Mostly in Europe.|   `region`|states and provinces|   `macroregion`|a related group of regions. Mostly in Europe|   `country`|places that issue passports, nations, nation-states|   `coarse`|alias for simultaneously using all administrative layers (everything except `venue` and `address`)|  |  |

### Return type

[**models::GeocodeResponse**](Geocode_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## geocode_reverse_get

> models::GeocodeResponse geocode_reverse_get(api_key, point_lon, point_lat, boundary_circle_radius, size, layers, sources, boundary_country)
Reverse Geocode Service

Returns the next enclosing object with an address tag which surrounds the given coordinate. **The interactivity for this enpoint is experimental!** [Please refer to this external Documentation](https://github.com/pelias/documentation/blob/master/reverse.md#reverse-geocoding) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | Insert your API Key here.  | [required] |[default to your-api-key]
**point_lon** | **f32** | Longitude of the coordinate to query.  | [required] |
**point_lat** | **f32** | Latitude of the coordinate to query.  | [required] |[default to 48.858268]
**boundary_circle_radius** | Option<**f64**> | Restrict search to circular region around `point.lat/point.lon`. Value in kilometers.  |  |[default to 1]
**size** | Option<**i64**> | Set the number of returned results.  |  |[default to 10]
**layers** | Option<[**Vec<String>**](String.md)> | Restrict search to layers (place type). By default all layers are searched.   layer|description|   ----|----|   `venue`|points of interest, businesses, things with walls|   `address`|places with a street address|   `street`|streets,roads,highways|   `neighbourhood`|social communities, neighbourhoods|   `locality`|towns, hamlets, cities|   `borough`|a local administrative boundary, currently only used for New York City|   `localadmin`|local administrative boundaries|   `county`|official governmental area; usually bigger than a locality, almost always smaller than a region|   `macrocounty`|a related group of counties. Mostly in Europe.|   `region`|states and provinces|   `macroregion`|a related group of regions. Mostly in Europe|   `country`|places that issue passports, nations, nation-states|   `coarse`|alias for simultaneously using all administrative layers (everything except `venue` and `address`)|  |  |
**sources** | Option<[**Vec<String>**](String.md)> | Restrict your search to specific sources. Searches all sources by default. You can either use the normal or short name. Sources are [`openstreetmap(osm)`](http://www.openstreetmap.org/), [`openaddresses(oa)`](http://openaddresses.io/), [`whosonfirst(wof)`](https://whosonfirst.org/), [`geonames(gn)`](http://www.geonames.org/).  |  |[default to ["osm","oa","gn","wof"]]
**boundary_country** | Option<**String**> | Restrict search to country by [alpha 2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) or [alpha 3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3) codes.  |  |

### Return type

[**models::GeocodeResponse**](Geocode_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## geocode_search_get

> models::GeocodeResponse geocode_search_get(api_key, text, focus_point_lon, focus_point_lat, boundary_rect_min_lon, boundary_rect_min_lat, boundary_rect_max_lon, boundary_rect_max_lat, boundary_circle_lon, boundary_circle_lat, boundary_circle_radius, boundary_gid, boundary_country, sources, layers, size)
Forward Geocode Service

Returns a JSON formatted list of objects corresponding to the search input. `boundary.*`-parameters can be combined if they are overlapping. **The interactivity for this enpoint is experimental!** [Please refer to this external Documentation](https://github.com/pelias/documentation/blob/master/search.md#search-the-world) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | Insert your API Key here.  | [required] |[default to you-api-key]
**text** | **String** | Name of location, street address or postal code.  | [required] |
**focus_point_lon** | Option<**f32**> | Longitude of the `focus.point`. Specify the focus point to order results by linear distance to this point. Works for up to 100 kilometers distance.  Use with `focus.point.lat`.  |  |
**focus_point_lat** | Option<**f32**> | Latitude of the `focus.point`. Specify the focus point to order results by linear distance to this point. Works for up to 100 kilometers distance. Use with `focus.point.lon`.  |  |
**boundary_rect_min_lon** | Option<**f32**> | Left border of rectangular boundary to narrow results.  |  |
**boundary_rect_min_lat** | Option<**f32**> | Bottom border of rectangular boundary to narrow results.  |  |
**boundary_rect_max_lon** | Option<**f32**> | Right border of rectangular boundary to narrow results.  |  |
**boundary_rect_max_lat** | Option<**f32**> | Top border of rectangular boundary to narrow results.  |  |
**boundary_circle_lon** | Option<**f32**> | Center Longitude of circular boundary to narrow results. Use with `boundary.circle.lat` & `boundary.circle.radius`.  |  |
**boundary_circle_lat** | Option<**f32**> | Center Latitude of circular boundary to narrow results. Use with `boundary.circle.lon` & `boundary.circle.radius`.  |  |
**boundary_circle_radius** | Option<**f32**> | Radius of circular boundary around the center coordinate in kilometers. Use with `boundary.circle.lon` & `boundary.circle.lat`.  |  |[default to 50]
**boundary_gid** | Option<**String**> | Restrict results to administrative boundary using a Pelias global id [`gid`](https://github.com/pelias/documentation/blob/f1f475aa4f8c18426fb80baea636990502c08ed3/search.md#search-within-a-parent-administrative-area). `gid`s for records can be found using either the [Who's on First Spelunker](http://spelunker.whosonfirst.org/), a tool for searching Who's on First data, or from the responses of other Pelias queries. In this case a [search for Oklahoma](http://pelias.github.io/compare/#/v1/search%3Ftext=oklahoma) will return the proper `gid`.  |  |
**boundary_country** | Option<**String**> | Restrict results to single country. Possible values are [alpha-2 and alpha-3 country codes](https://en.wikipedia.org/wiki/ISO_3166-1). Example: `DEU` or `DE` for Germany.  |  |
**sources** | Option<[**Vec<String>**](String.md)> | Restrict your search to specific sources. Searches all sources by default. You can either use the normal or short name. Sources are [`openstreetmap(osm)`](http://www.openstreetmap.org/), [`openaddresses(oa)`](http://openaddresses.io/), [`whosonfirst(wof)`](https://whosonfirst.org/), [`geonames(gn)`](http://www.geonames.org/).  |  |[default to ["osm","oa","gn","wof"]]
**layers** | Option<[**Vec<String>**](String.md)> | Restrict search to layers (place type). By default all layers are searched.   layer|description|   ----|----|   `venue`|points of interest, businesses, things with walls|   `address`|places with a street address|   `street`|streets,roads,highways|   `neighbourhood`|social communities, neighbourhoods|   `borough`|a local administrative boundary, currently only used for New York City|   `localadmin`|local administrative boundaries|   `locality`|towns, hamlets, cities|   `county`|official governmental area; usually bigger than a locality, almost always smaller than a region|   `macrocounty`|a related group of counties. Mostly in Europe.|   `region`|states and provinces|   `macroregion`|a related group of regions. Mostly in Europe|   `country`|places that issue passports, nations, nation-states|   `coarse`|alias for simultaneously using all administrative layers (everything except `venue` and `address`)|  |  |
**size** | Option<**i64**> | Set the number of returned results.  |  |[default to 10]

### Return type

[**models::GeocodeResponse**](Geocode_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## geocode_search_structured_get

> models::GeocodeResponse geocode_search_structured_get(api_key, address, neighbourhood, country, postalcode, region, county, locality, borough, focus_point_lon, focus_point_lat, boundary_rect_min_lon, boundary_rect_min_lat, boundary_rect_max_lon, boundary_rect_max_lat, boundary_circle_lon, boundary_circle_lat, boundary_circle_radius, boundary_country, layers, sources, size)
Structured Forward Geocode Service (beta)

Returns a JSON formatted list of objects corresponding to the search input. **The interactivity for this enpoint is experimental!** [Please refer to this external Documentation](https://github.com/pelias/documentation/blob/master/structured-geocoding.md#structured-geocoding) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | Insert your API Key here.  | [required] |[default to your-api-key]
**address** | Option<**String**> | Search for full address with house number or only a street name.  |  |
**neighbourhood** | Option<**String**> | Search for neighbourhoods. Neighbourhoods are vernacular geographic entities that may not necessarily be official administrative divisions but are important nonetheless. Example: `Notting Hill`.  |  |
**country** | Option<**String**> | Search for full country name, [alpha 2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) or [alpha 3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3) codes.  |  |
**postalcode** | Option<**String**> | Search for postal codes. Postal codes are unique within a country so they are useful in geocoding as a shorthand for a fairly granular geographical location.  |  |
**region** | Option<**String**> | Search for regions. Regions are normally the first-level administrative divisions within countries. For US-regions [common abbreviations](https://en.wikipedia.org/wiki/List_of_U.S._state_abbreviations) can be used.  |  |
**county** | Option<**String**> | Search for counties. Counties are administrative divisions between localities and regions. Can be useful when attempting to disambiguate between localities.  |  |
**locality** | Option<**String**> | Search for localities. Localities are equivalent to what are commonly referred to as *cities*.  |  |[default to Tokyo]
**borough** | Option<**String**> | Search for boroughs. Boroughs are mostly known in the context of New York City, even though they may exist in other cities, such as Mexico City. Example: `Manhatten`.  |  |
**focus_point_lon** | Option<**f32**> | Longitude of the `focus.point`. Specify the focus point to order results by linear distance to this point. Works for up to 100 kilometers distance.  Use with `focus.point.lat`.  |  |
**focus_point_lat** | Option<**f32**> | Latitude of the `focus.point`. Specify the focus point to order results by linear distance to this point. Works for up to 100 kilometers distance. Use with `focus.point.lon`.  |  |
**boundary_rect_min_lon** | Option<**f32**> | Left border of rectangular boundary to narrow results.  |  |
**boundary_rect_min_lat** | Option<**f32**> | Bottom border of rectangular boundary to narrow results.  |  |
**boundary_rect_max_lon** | Option<**f32**> | Right border of rectangular boundary to narrow results.  |  |
**boundary_rect_max_lat** | Option<**f32**> | Top border of rectangular boundary to narrow results.  |  |
**boundary_circle_lon** | Option<**f32**> | Center Longitude of circular boundary to narrow results. Use with `boundary.circle.lat` & `boundary.circle.radius`.  |  |
**boundary_circle_lat** | Option<**f32**> | Center Latitude of circular boundary to narrow results. Use with `boundary.circle.lon` & `boundary.circle.radius`.  |  |
**boundary_circle_radius** | Option<**f32**> | Radius of circular boundary around the center coordinate in kilometers. Use with `boundary.circle.lon` & `boundary.circle.lat`.  |  |[default to 50]
**boundary_country** | Option<**String**> | Restrict results to single country. Possible values are [alpha-2 and alpha-3 country codes](https://en.wikipedia.org/wiki/ISO_3166-1). Example: `DEU` or `DE` for Germany.  |  |
**layers** | Option<[**Vec<String>**](String.md)> | Restrict search to layers (place type). By default all layers are searched.   layer|description|   ----|----|   `venue`|points of interest, businesses, things with walls|   `address`|places with a street address|   `street`|streets,roads,highways|   `neighbourhood`|social communities, neighbourhoods|   `borough`|a local administrative boundary, currently only used for New York City|   `localadmin`|local administrative boundaries|   `locality`|towns, hamlets, cities|   `county`|official governmental area; usually bigger than a locality, almost always smaller than a region|   `macrocounty`|a related group of counties. Mostly in Europe.|   `region`|states and provinces|   `macroregion`|a related group of regions. Mostly in Europe|   `country`|places that issue passports, nations, nation-states|   `coarse`|alias for simultaneously using all administrative layers (everything except `venue` and `address`)|  |  |
**sources** | Option<[**Vec<String>**](String.md)> | Restrict your search to specific sources. Searches all sources by default. You can either use the normal or short name. Sources are [`openstreetmap(osm)`](http://www.openstreetmap.org/), [`openaddresses(oa)`](http://openaddresses.io/), [`whosonfirst(wof)`](https://whosonfirst.org/), [`geonames(gn)`](http://www.geonames.org/).  |  |[default to ["osm","oa","gn","wof"]]
**size** | Option<**i64**> | Set the number of returned results.  |  |[default to 10]

### Return type

[**models::GeocodeResponse**](Geocode_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

