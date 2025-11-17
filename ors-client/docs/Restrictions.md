# Restrictions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**length** | Option<**f32**> | Length restriction in metres. | [optional]
**width** | Option<**f32**> | Width restriction in metres. | [optional]
**height** | Option<**f32**> | Height restriction in metres.  | [optional]
**axleload** | Option<**f32**> | Axleload restriction in tons. | [optional]
**weight** | Option<**f32**> | Weight restriction in tons.  | [optional]
**hazmat** | Option<**bool**> | Specifies whether to use appropriate routing for delivering hazardous goods and avoiding water protected areas. Default is `false`.  | [optional][default to false]
**surface_type** | Option<**String**> | Specifies the minimum surface type. Default is `sett`.  | [optional][default to sett]
**track_type** | Option<**String**> | Specifies the minimum grade of the route. Default is `grade1`.  | [optional][default to grade1]
**smoothness_type** | Option<**String**> | Specifies the minimum smoothness of the route. Default is `good`. | [optional][default to Good]
**maximum_sloped_kerb** | Option<**f32**> | Specifies the maximum height of the sloped curb in metres. Values are `0.03`, `0.06` (default), `0.1`. | [optional][default to 0.6]
**maximum_incline** | Option<**i32**> | Specifies the maximum incline as a percentage. `3`, `6` (default), `10`, `15. | [optional][default to 6]
**minimum_width** | Option<**f32**> | Specifies the minimum width of the footway in metres. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


