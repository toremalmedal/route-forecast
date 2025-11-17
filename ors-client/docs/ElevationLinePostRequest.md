# ElevationLinePostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format_in** | **String** | The input format the API has to expect. | 
**format_out** | Option<**String**> | The output format to be returned. | [optional][default to Geojson]
**dataset** | Option<**String**> | The elevation dataset to be used. | [optional][default to Srtm]
**geometry** | [**serde_json::Value**](.md) | * geojson: A geometry object of a LineString GeoJSON, e.g.          {\"type\": \"LineString\",           \"coordinates\": [[13.331302, 38.108433],[13.331273, 38.10849]]          } * polyline: A list of coordinate lists, e.g.          [[13.331302, 38.108433], [13.331273, 38.10849]]  * encodedpolyline5: A <a href=\"https://developers.google.com/maps/documentation/utilities/polylinealgorithm\">Google encoded polyline</a> with a coordinate precision of 5, e.g.          u`rgFswjpAKD  * encodedpolyline6: A <a href=\"https://developers.google.com/maps/documentation/utilities/polylinealgorithm\">Google encoded polyline</a> with a coordinate precision of 6, e.g.          ap}tgAkutlXqBx@  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


