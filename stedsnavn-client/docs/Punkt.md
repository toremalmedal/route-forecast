# Punkt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meter_fra_punkt** | Option<**i32**> | Distanse i meter til punktet det ble søkt etter. | [optional]
**navneobjekttype** | Option<**String**> | Objekttypen som et sted kan tilhøre, f.eks en by, elv eller kirke | [optional]
**representasjonspunkt** | Option<[**models::Representasjonspunkt**](Representasjonspunkt.md)> | Et konstruert representasjonspunkt for stedsnavnet. | [optional]
**stedsnavn** | Option<[**Vec<models::SkrivemateJson>**](SkrivemateJson.md)> |  | [optional]
**stedsnummer** | Option<**i32**> | Identifikator for et sted. | [optional]
**stedstatus** | Option<**String**> | Status for selve stedet. Et sted kan være et relikt (historisk), aktivt eller planlagt sted. En kommune som ikke lenger eksisterer er f.eks. relikt, mens en bru som skal bygges er planlagt. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


