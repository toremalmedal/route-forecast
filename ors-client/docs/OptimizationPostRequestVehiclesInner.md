# OptimizationPostRequestVehiclesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Integer used as unique identifier  | [optional]
**profile** | Option<**String**> | The ORS routing profile for the vehicle.  | [optional]
**description** | Option<**String**> | a string describing this vehicle  | [optional]
**start** | Option<**Vec<f32>**> | Start coordinates array in `[lon, lat]` format. If left blank, the optimization engine will identify the optimal start point.  | [optional]
**start_index** | Option<[**serde_json::Value**](.md)> | Index of relevant row and column in custom matrix.  | [optional]
**end** | Option<**Vec<f32>**> | End coordinates array in `[lon, lat]` format. If left blank, the optimization engine will identify the optimal end point.  | [optional]
**end_index** | Option<[**serde_json::Value**](.md)> | Index of relevant row and column in custom matrix.  | [optional]
**capacity** | Option<**Vec<i32>**> | Array of integers describing multidimensional quantities.  | [optional]
**costs** | Option<[**models::OptimizationPostRequestVehiclesInnerCosts**](_optimization_post_request_vehicles_inner_costs.md)> |  | [optional]
**skills** | Option<**Vec<i32>**> | Array of integers defining skills for this vehicle  | [optional]
**time_window** | Option<**Vec<i32>**> | A `time_window` array describing working hours for this vehicle, in week seconds, i.e. 28800 = Mon, 8 AM.  | [optional]
**breaks** | Option<[**Vec<models::OptimizationPostRequestVehiclesInnerBreaksInner>**](_optimization_post_request_vehicles_inner_breaks_inner.md)> | An array of `break` objects  | [optional]
**speed_factor** | Option<**f32**> | A double value in the range (0, 5] used to scale all vehicle travel times (defaults to 1.). The respected precision is limited to two digits after the decimal point.  | [optional]
**max_tasks** | Option<**f64**> | an integer defining the maximum number of tasks in a route for this vehicle  | [optional]
**max_travel_time** | Option<**f64**> | an integer defining the maximum travel time for this vehicle  | [optional]
**max_distance** | Option<**f64**> | an integer defining the maximum distance for this vehicle  | [optional]
**steps** | Option<[**Vec<models::OptimizationPostRequestVehiclesInnerStepsInner>**](_optimization_post_request_vehicles_inner_steps_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


