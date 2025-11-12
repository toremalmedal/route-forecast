# RouteOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avoid_features** | Option<**Vec<String>**> | List of features to avoid.  | [optional]
**avoid_borders** | Option<**String**> | Specify which type of border crossing to avoid | [optional]
**avoid_countries** | Option<**Vec<String>**> | List of countries to exclude from matrix with `driving-*` profiles. Can be used together with `'avoid_borders': 'controlled'`. `[ 11, 193 ]` would exclude Austria and Switzerland. List of countries and application examples can be found [here](https://giscience.github.io/openrouteservice/technical-details/country-list). Also, ISO standard country codes cna be used in place of the numerical ids, for example, DE or DEU for Germany.  | [optional]
**vehicle_type** | Option<**String**> | Definition of the vehicle type. | [optional][default to Hgv]
**profile_params** | Option<[**models::ProfileParameters**](Profile_Parameters.md)> |  | [optional]
**avoid_polygons** | Option<[**models::RouteOptionsAvoidPolygons**](Route_Options_avoid_polygons.md)> |  | [optional]
**round_trip** | Option<[**models::RoundTripRouteOptions**](Round_Trip_Route_Options.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


