# OptimizationPost200ResponseSummaryViolationsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cause** | Option<**String**> | string describing the cause of violation. Possible violation causes are:             - \"delay\" if actual service start does not meet a task time window and is late on a time window end             - \"lead_time\" if actual service start does not meet a task time window and is early on a time window start             - \"load\" if the vehicle load goes over its capacity             - \"max_tasks\" if the vehicle has more tasks than its max_tasks value             - \"skills\" if the vehicle does not hold all required skills for a task             - \"precedence\" if a shipment precedence constraint is not met (pickup without matching delivery, delivery before/without matching pickup)             - \"missing_break\" if a vehicle break has been omitted in its custom route             - \"max_travel_time\" if the vehicle has more travel time than its max_travel_time value             - \"max_load\" if the load during a break exceed its max_load value  | [optional]
**duration** | Option<**f64**> | Earliness (resp. lateness) if `cause` is \"lead_time\" (resp \"delay\")  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


