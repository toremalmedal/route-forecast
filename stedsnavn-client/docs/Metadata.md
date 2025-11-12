# Metadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**side** | Option<**i32**> | Sidenummeret som skal returneres. Minimum 1, maksimum 500. | [optional][default to 1]
**soke_streng** | Option<**String**> | Søkestrengen som ble sendt inn til API-et. | [optional]
**totalt_antall_treff** | Option<**i32**> | Antall treff som søket returnerte. Hvis det er et tungt søk er antallet ofte et grovestimat, ikke et nøyaktig antall. | [optional]
**treff_per_side** | Option<**i32**> | Antall treff per side. Minimum 1, maksimum 500. | [optional][default to 10]
**utkoordsys** | Option<**i32**> | Angi det koordinatsystemet som du ønsker at geometrien i returen skal transformeres til, oppgis som en SRID (altså tallene i en EPSG-kode, f.eks. 4258 eller 25833). Standard er 4258. | [optional][default to 4258]
**viser_fra** | Option<**i32**> | Viser treff fra og medobjekt nummer X i responsen. | [optional]
**viser_til** | Option<**i32**> | Viser treff til og med objekt nummer X i responsen. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


