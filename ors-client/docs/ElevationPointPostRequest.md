# ElevationPointPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format_in** | **String** | The input format the API has to expect. | 
**format_out** | Option<**String**> | The output format to be returned. | [optional][default to Geojson]
**dataset** | Option<**String**> | The elevation dataset to be used. | [optional][default to Srtm]
**geometry** | [**serde_json::Value**](.md) | * geojson: A geometry object of a Point GeoJSON, e.g.          {\"type\": \"Point\",           \"coordinates\": [13.331273, 38.10849]          } * point: A coordinate list, e.g.          [13.331273, 38.10849]  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


