# \DefaultApi

All URIs are relative to *https://api.kartverket.no/stedsnavn/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**navn_get**](DefaultApi.md#navn_get) | **GET** /navn | Søk etter et spesifikt navn (skrivemåte/stedsnavn).
[**navneobjekttyper_get**](DefaultApi.md#navneobjekttyper_get) | **GET** /navneobjekttyper | Vis alle navneobjekttyper (f.eks by, elv, kirke) som et sted kan ha.
[**punkt_get**](DefaultApi.md#punkt_get) | **GET** /punkt | Finn steder nær et gitt geografisk punkt.
[**sprak_get**](DefaultApi.md#sprak_get) | **GET** /sprak | Vis alle språk som et navn kan ha.
[**sted_get**](DefaultApi.md#sted_get) | **GET** /sted | Søk etter et sted. Et sted kan ha flere navn og hvert navn kan ha flere skrivemåter.



## navn_get

> models::ReturSkrivemate navn_get(sok, fuzzy, fnr, knr, kommunenavn, fylkesnavn, sprak, utkoordsys, treff_per_side, side, filtrer)
Søk etter et spesifikt navn (skrivemåte/stedsnavn).

Søk etter et spesifikt navn (skrivemåte/stedsnavn). Returen er sortert etter stedets kategorisering innenfor Sentralt stedsnavnsregister (et fylke kommer f.eks. før en bekk). Innenfor hver kategori er så returen sortert etter hvor godt det matcher søkebegrepet. <p>For eksempel ?sok=Trondheim*&fuzzy=false</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sok** | Option<**String**> | Søk etter stedsnavn. Støtter wildcard-søk (*) |  |
**fuzzy** | Option<**bool**> | Sett til \"true\" for å utføre et fuzzy-søk. Søket vil da prøve å finne alle navn som ligner på det du søker etter. Treffene vil sorteres etter hvor mye de ligner på søkebegrepet. Kan være litt tregere enn et vanlig søk. |  |[default to false]
**fnr** | Option<[**Vec<String>**](String.md)> | Søk innenfor et fylkesnummer. Bestående av two tegn med ledende 0 om nødvendig. Kan søke i flere fylker ved å gjenta parameteret, f.eks. fnr=50&fnr=03 |  |
**knr** | Option<[**Vec<String>**](String.md)> | Søk innenfor en kommune ved å skrive inn kommunenummeret. Bestående av fire tegn med ledende 0 om nødvendig. Kan søke i flere kommuner ved å gjenta parameteret, f.eks. knr=5001&knr=0301 |  |
**kommunenavn** | Option<**String**> | Søk etter treff i kommunen med oppgitt navn. Vær obs på at flere kommuner har samme navn, det anbefales derfor å søke etter kommunenummer hvis mulig. |  |
**fylkesnavn** | Option<**String**> | Søk etter treff i fylket med oppgitt navn. |  |
**sprak** | Option<**String**> | Filtrer skrivemåte etter språk, vis kun de skrivemåtene som er i det angitte språk. Se i \"/sprak\"-endepunktet for å se mulige valg for språkkoder. |  |
**utkoordsys** | Option<**i32**> | Angi det koordinatsystemet som du ønsker at geometrien i returen skal transformeres til, oppgis som en SRID (altså tallene i en EPSG-kode, f.eks. 4258 eller 25833). Standard er 4258. |  |[default to 4258]
**treff_per_side** | Option<**i32**> | Antall treff per side. Minimum 1, maksimum 500. |  |[default to 10]
**side** | Option<**i32**> | Sidenummeret som skal returneres. Minimum 1, maksimum 500. |  |[default to 1]
**filtrer** | Option<**String**> | Vis kun de elementene du vil ha i returen. Kommaseparert liste med nøkler. For å hente ut underobjekter bruk \".\"-notasjon, f.eks. &filtrer=navn.stedsnummer,metadata |  |

### Return type

[**models::ReturSkrivemate**](ReturSkrivemate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## navneobjekttyper_get

> Vec<models::Navneobjekttype> navneobjekttyper_get()
Vis alle navneobjekttyper (f.eks by, elv, kirke) som et sted kan ha.

Vis alle navneobjekttyper (f.eks by, elv, kirke) som et sted kan ha. Man kan filtrere på navneobjekttypekoder i noen søk, dette er en oversikt over de gyldige alternativene.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Navneobjekttype>**](Navneobjekttype.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## punkt_get

> models::ReturPunkt punkt_get(nord, ost, koordsys, radius, utkoordsys, treff_per_side, side, filtrer)
Finn steder nær et gitt geografisk punkt.

Gjør et geografisk søk etter de nærmeste stedene. Vær obs på at et sted kan ha flere stedsnavn og at hvert stedsnavn kan ha flere likestilte skrivemåter. Returen er sortert etter stedets kategorisering innenfor Sentralt stedsnavnsregister slik at det mest \"nærliggende\" stedet kommer først (en bekk kommer f.eks. før et fylke). Innenfor hver kategori er så returen sortert etter distanse til punktet som man søkte etter.  <p>For eksempel ?nord=60.5&ost=11.12&koordsys=4258&radius=1000</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nord** | **f64** | Nord-koordinaten/Latitude | [required] |
**ost** | **f64** | Øst-koordinaten/Longitude | [required] |
**koordsys** | **i32** | Koordinatsystemet (EPSG) til koordinatene du søker med. Angis som en SRID, for eksempel 4258 eller 25833. | [required] |
**radius** | Option<**i32**> | Radius i antall meter som søket leter etter stedsnavn i. Maksimum er 5000m. |  |[default to 500]
**utkoordsys** | Option<**i32**> | Angi det koordinatsystemet som du ønsker at geometrien i returen skal transformeres til, oppgis som en SRID (altså tallene i en EPSG-kode, f.eks. 4258 eller 25833). Standard er 4258. |  |[default to 4258]
**treff_per_side** | Option<**i32**> | Antall treff per side. Minimum 1, maksimum 500. |  |[default to 10]
**side** | Option<**i32**> | Sidenummeret som skal returneres. Minimum 1, maksimum 500. |  |[default to 1]
**filtrer** | Option<**String**> | Vis kun de elementene du vil ha i returen. Kommaseparert liste med nøkler. For å hente ut underobjekter bruk \".\"-notasjon, f.eks. &filtrer=navn.stedsnummer,metadata |  |

### Return type

[**models::ReturPunkt**](ReturPunkt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sprak_get

> Vec<models::Sprak> sprak_get()
Vis alle språk som et navn kan ha.

Man kan filtrere på språkkoder i noen søk, dette er en oversikt over de gyldige alternativene.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Sprak>**](Sprak.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sted_get

> models::ReturSted sted_get(sok, fuzzy, fnr, knr, kommunenavn, fylkesnavn, stedsnummer, navneobjekttype, utkoordsys, treff_per_side, side, filtrer)
Søk etter et sted. Et sted kan ha flere navn og hvert navn kan ha flere skrivemåter.

Søk etter et sted. Et sted kan ha flere stedsnavn, hvert navn kan ha flere skrivemåter. Returen er sortert etter stedets kategorisering innenfor Sentralt stedsnavnsregister (et fylke kommer f.eks. før en bekk). Innenfor hver kategori er så returen sortert etter hvor godt det matcher søkebegrepet. <p>For eksempel ?sok=Trondheim*&knr=5001</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sok** | Option<**String**> | Søk etter stedsnavn. Støtter wildcard-søk (*) |  |
**fuzzy** | Option<**bool**> | Sett til \"true\" for å utføre et fuzzy-søk. Søket vil da prøve å finne alle navn som ligner på det du søker etter. Treffene vil sorteres etter hvor mye de ligner på søkebegrepet. Kan være litt tregere enn et vanlig søk. |  |[default to false]
**fnr** | Option<[**Vec<String>**](String.md)> | Søk innenfor et fylkesnummer. Bestående av two tegn med ledende 0 om nødvendig. Kan søke i flere fylker ved å gjenta parameteret, f.eks. fnr=50&fnr=03 |  |
**knr** | Option<[**Vec<String>**](String.md)> | Søk innenfor en kommune ved å skrive inn kommunenummeret. Bestående av fire tegn med ledende 0 om nødvendig. Kan søke i flere kommuner ved å gjenta parameteret, f.eks. knr=5001&knr=0301 |  |
**kommunenavn** | Option<**String**> | Søk etter treff i kommunen med oppgitt navn. Vær obs på at flere kommuner har samme navn, det anbefales derfor å søke etter kommunenummer hvis mulig. |  |
**fylkesnavn** | Option<**String**> | Søk etter treff i fylket med oppgitt navn. |  |
**stedsnummer** | Option<**i32**> | Finn stedsnavn med et gitt stedsnummer. |  |
**navneobjekttype** | Option<[**Vec<String>**](String.md)> | Filtrer returen etter navneobjekttypekode, viser kun de forekomstene som har angitt navneobjekttype. Sjekk endepunktet /navneobjekttyper for å se mulige valg for navneobjekttypekoder. Det er mulig å filtrere på flere navneobjekttypekoder ved å gjenta parameteret. |  |
**utkoordsys** | Option<**i32**> | Angi det koordinatsystemet som du ønsker at geometrien i returen skal transformeres til, oppgis som en SRID (altså tallene i en EPSG-kode, f.eks. 4258 eller 25833). Standard er 4258. |  |[default to 4258]
**treff_per_side** | Option<**i32**> | Antall treff per side. Minimum 1, maksimum 500. |  |[default to 10]
**side** | Option<**i32**> | Sidenummeret som skal returneres. Minimum 1, maksimum 500. |  |[default to 1]
**filtrer** | Option<**String**> | Vis kun de elementene du vil ha i returen. Kommaseparert liste med nøkler. For å hente ut underobjekter bruk \".\"-notasjon, f.eks. &filtrer=navn.stedsnummer,metadata |  |

### Return type

[**models::ReturSted**](ReturSted.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

