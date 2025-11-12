# Sok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fylker** | Option<[**Vec<models::Fylker>**](Fylker.md)> |  | [optional]
**geojson** | Option<[**models::GeoJsonGeometry**](GeoJsonGeometry.md)> | Stedsnavnet sin fullstendige geometri i en geojson-struktur. | [optional]
**kommuner** | Option<[**Vec<models::Kommuner>**](Kommuner.md)> |  | [optional]
**navneobjekttype** | Option<**String**> | Objekttypen som et sted kan tilhøre, f.eks en by, elv eller kirke | [optional]
**oppdateringsdato** | Option<**String**> | Det som vises er den nyeste av de tre oppdateringsdatoene for stedet, skrivemåten og stedsnavnet. | [optional]
**representasjonspunkt** | Option<[**models::Representasjonspunkt**](Representasjonspunkt.md)> | Et konstruert representasjonspunkt for stedsnavnet. | [optional]
**stedsnavn** | Option<[**Vec<models::SkrivemateJson>**](SkrivemateJson.md)> |  | [optional]
**stedsnummer** | Option<**i32**> | Identifikator for et sted. | [optional]
**stedstatus** | Option<**String**> | Status for selve stedet. Et sted kan være et relikt (historisk), aktivt eller planlagt sted. En kommune som ikke lenger eksisterer er f.eks. relikt, mens en bru som skal bygges er planlagt. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


