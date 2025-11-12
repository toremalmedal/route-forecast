# JsonLeg

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of the leg, possible values are currently 'walk' and 'pt'. | [optional]
**departure_location** | Option<**String**> | The departure location of the leg. | [optional]
**trip_headsign** | Option<**String**> | The headsign of the public transport vehicle of the leg. | [optional]
**route_long_name** | Option<**String**> | The public transport route name of the leg. | [optional]
**route_short_name** | Option<**String**> | The public transport route name (short version) of the leg. | [optional]
**route_desc** | Option<**String**> | The route description of the leg (if provided in the GTFS data set). | [optional]
**route_type** | Option<**i32**> | The route type of the leg (if provided in the GTFS data set). | [optional]
**distance** | Option<**f64**> | The distance for the leg in metres. | [optional]
**duration** | Option<**f64**> | The duration for the leg in seconds. | [optional]
**departure** | Option<**String**> | Departure date and time | [optional]
**arrival** | Option<**String**> | Arrival date and time | [optional]
**feed_id** | Option<**String**> | The feed ID this public transport leg based its information from. | [optional]
**trip_id** | Option<**String**> | The trip ID of this public transport leg. | [optional]
**route_id** | Option<**String**> | The route ID of this public transport leg. | [optional]
**is_in_same_vehicle_as_previous** | Option<**bool**> | Whether the legs continues in the same vehicle as the previous one. | [optional]
**geometry** | Option<**String**> | The geometry of the leg. This is an encoded polyline. | [optional]
**instructions** | Option<[**Vec<models::GetDefault3200ResponseRoutesInnerSegmentsInnerStepsInner>**](getDefault_3_200_response_routes_inner_segments_inner_steps_inner.md)> | List containing the specific steps the segment consists of. | [optional]
**stops** | Option<[**Vec<models::GetDefault3200ResponseRoutesInnerLegsInnerStopsInner>**](getDefault_3_200_response_routes_inner_legs_inner_stops_inner.md)> | List containing the stops the along the leg. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


