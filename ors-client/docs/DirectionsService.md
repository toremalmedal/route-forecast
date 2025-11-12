# DirectionsService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**coordinates** | [**Vec<Vec<f64>>**](Vec.md) | The waypoints to use for the route as an array of `longitude/latitude` pairs in WGS 84 (EPSG:4326) | 
**id** | Option<**String**> | Arbitrary identification string of the request reflected in the meta information. | [optional]
**preference** | Option<**String**> | Specifies the route preference | [optional][default to Recommended]
**units** | Option<**String**> | Specifies the distance unit. | [optional][default to M]
**language** | Option<**String**> | Language for the route instructions. | [optional][default to En]
**geometry** | Option<**bool**> | Specifies whether to return geometry.  | [optional][default to true]
**instructions** | Option<**bool**> | Specifies whether to return instructions. | [optional][default to true]
**instructions_format** | Option<**String**> | Select html for more verbose instructions. | [optional][default to Text]
**roundabout_exits** | Option<**bool**> | Provides bearings of the entrance and all passed roundabout exits. Adds the `exit_bearings` array to the step object in the response.  | [optional][default to false]
**attributes** | Option<**Vec<String>**> | List of route attributes | [optional]
**maneuvers** | Option<**bool**> | Specifies whether the maneuver object is included into the step object or not.  | [optional][default to false]
**radiuses** | Option<**Vec<f64>**> | A list of maximum distances (measured in metres) that limit the search of nearby road segments to every given waypoint. The values must be greater than 0, the value of -1 specifies using the maximum possible search radius. The number of radiuses correspond to the number of waypoints. If only a single value is given, it will be applied to all waypoints. | [optional]
**bearings** | Option<[**Vec<Vec<f64>>**](Vec.md)> | Specifies a list of pairs (bearings and deviations) to filter the segments of the road network a waypoint can snap to. \"For example `bearings=[[45,10],[120,20]]`. \"Each pair is a comma-separated list that can consist of one or two float values, where the first value is the bearing and the second one is the allowed deviation from the bearing. \"The bearing can take values between `0` and `360` clockwise from true north. If the deviation is not set, then the default value of `100` degrees is used. \"The number of pairs must correspond to the number of waypoints. \"The number of bearings corresponds to the length of waypoints-1 or waypoints. If the bearing information for the last waypoint is given, then this will control the sector from which the destination waypoint may be reached. \"You can skip a bearing for a certain waypoint by passing an empty value for an array, e.g. `[30,20],[],[40,20]`. | [optional]
**continue_straight** | Option<**bool**> | Forces the route to keep going straight at waypoints restricting uturns there even if it would be faster. | [optional][default to false]
**elevation** | Option<**bool**> | Specifies whether to return elevation values for points. Please note that elevation also gets encoded for json response encoded polyline. | [optional]
**extra_info** | Option<**Vec<String>**> | The extra info items to include in the response | [optional]
**options** | Option<[**models::RouteOptions**](Route_Options.md)> |  | [optional]
**suppress_warnings** | Option<**bool**> | Suppress warning messages in the response | [optional]
**geometry_simplify** | Option<**bool**> | Specifies whether to simplify the geometry. Simplify geometry cannot be applied to routes with more than **one segment** and when `extra_info` is required. | [optional][default to false]
**skip_segments** | Option<**Vec<i32>**> | Specifies the segments that should be skipped in the route calculation. A segment is the connection between two given coordinates and the counting starts with 1 for the connection between the first and second coordinate. | [optional]
**alternative_routes** | Option<[**models::AlternativeRoutes**](Alternative_Routes.md)> |  | [optional]
**maximum_speed** | Option<**f64**> | The maximum speed specified by user. | [optional]
**schedule** | Option<**bool**> | If true, return a public transport schedule starting at <departure> for the next <schedule_duration> minutes. | [optional][default to false]
**schedule_duration** | Option<**String**> |  | [optional]
**schedule_rows** | Option<**i32**> | The maximum amount of entries that should be returned when requesting a schedule. | [optional]
**walking_time** | Option<**String**> |  | [optional]
**ignore_transfers** | Option<**bool**> | Specifies if transfers as criterion should be ignored. | [optional][default to false]
**custom_model** | Option<[**models::DirectionsServiceCustomModel**](Directions_Service_custom_model.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


