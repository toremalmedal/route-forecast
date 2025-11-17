# ProfileWeightings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**steepness_difficulty** | Option<**i32**> | Specifies the fitness level for `cycling-*` profiles.   level: 0 = Novice, 1 = Moderate, 2 = Amateur, 3 = Pro. The prefered gradient increases with level. | [optional]
**green** | Option<**f32**> | Specifies the Green factor for `foot-*` profiles.  factor: Multiplication factor range from 0 to 1. 0 is the green routing base factor without multiplying it by the manual factor and is already different from normal routing. 1 will prefer ways through green areas over a shorter route. | [optional]
**quiet** | Option<**f32**> | Specifies the Quiet factor for foot-* profiles.  factor: Multiplication factor range from 0 to 1. 0 is the quiet routing base factor without multiplying it by the manual factor and is already different from normal routing. 1 will prefer quiet ways over a shorter route. | [optional]
**shadow** | Option<**f32**> | Specifies the shadow factor for `foot-*` profiles.  factor: Multiplication factor range from 0 to 1. 0 is the shadow routing base factor without multiplying it by the manual factor and is already different from normal routing. 1 will prefer ways through shadow areas over a shorter route. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


