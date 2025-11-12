# Skrivemate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fylker** | Option<[**Vec<models::Fylker>**](Fylker.md)> |  | [optional]
**kommuner** | Option<[**Vec<models::Kommuner>**](Kommuner.md)> |  | [optional]
**navneobjekttype** | Option<**String**> | Objekttypen som et sted kan tilhøre, f.eks en by, elv eller kirke | [optional]
**navnestatus** | Option<**String**> | Angir statusen til navnet, f.eks. om det er et hovednavn eller sidenavn. | [optional]
**representasjonspunkt** | Option<[**models::Representasjonspunkt**](Representasjonspunkt.md)> | Et konstruert representasjonspunkt for stedsnavnet. | [optional]
**skrivemte** | Option<**String**> | En spesifikk måte å skrive et stedsnavn på. | [optional]
**skrivemtestatus** | Option<**String**> | Hvilken status denne skrivemåten har, f.eks. om den er historisk eller godkjent. | [optional]
**sprk** | Option<**String**> | Språket til skrivemåten/navnet. | [optional]
**stedsnummer** | Option<**i32**> | Identifikator for et sted. | [optional]
**stedstatus** | Option<**String**> | Status for selve stedet. Et sted kan være et relikt (historisk), aktivt eller planlagt sted. En kommune som ikke lenger eksisterer er f.eks. relikt, mens en bru som skal bygges er planlagt. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


