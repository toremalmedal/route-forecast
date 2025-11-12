# JsonIndividualRouteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**summary** | Option<[**models::GetDefault3200ResponseRoutesInnerSummary**](getDefault_3_200_response_routes_inner_summary.md)> |  | [optional]
**segments** | Option<[**Vec<models::GetDefault3200ResponseRoutesInnerSegmentsInner>**](getDefault_3_200_response_routes_inner_segments_inner.md)> | List containing the segments and its corresponding steps which make up the route. | [optional]
**bbox** | Option<**Vec<f64>**> | A bounding box which contains the entire route | [optional]
**geometry** | Option<**String**> | The geometry of the route. For JSON route responses this is an encoded polyline. | [optional]
**way_points** | Option<**Vec<i32>**> | List containing the indices of way points corresponding to the *geometry*. | [optional]
**warnings** | Option<[**Vec<models::GetDefault2200ResponseWarning>**](getDefault_2_200_response_warning.md)> | List of warnings that have been generated for the route | [optional]
**legs** | Option<[**Vec<models::GetDefault3200ResponseRoutesInnerLegsInner>**](getDefault_3_200_response_routes_inner_legs_inner.md)> | List containing the legs the route consists of. | [optional]
**extras** | Option<[**std::collections::HashMap<String, models::GetDefault3200ResponseRoutesInnerExtrasValue>**](getDefault_3_200_response_routes_inner_extras_value.md)> | List of extra info objects representing the extra info items that were requested for the route. | [optional]
**departure** | Option<**String**> | Departure date and time | [optional]
**arrival** | Option<**String**> | Arrival date and time | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


