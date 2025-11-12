# GetDefaultIsochronesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Arbitrary identification string of the request reflected in the meta information. | [optional]
**locations** | [**Vec<Vec<f64>>**](Vec.md) | The locations to use for the route as an array of `longitude/latitude` pairs in WGS 84 (EPSG:4326) | 
**location_type** | Option<**String**> | `start` treats the location(s) as starting point, `destination` as goal. | [optional][default to Start]
**range** | **Vec<f64>** | Maximum range value of the analysis in **seconds** for time and **metres** for distance.Alternatively a comma separated list of specific range values. Ranges will be the same for all locations. | 
**range_type** | Option<**String**> | Specifies the isochrones reachability type. | [optional][default to Time]
**units** | Option<**String**> | Specifies the distance units only if `range_type` is set to distance. Default: m.  | [optional][default to M]
**options** | Option<[**models::RouteOptions**](Route_Options.md)> |  | [optional]
**area_units** | Option<**String**> | Specifies the area unit. Default: m.  | [optional][default to M]
**intersections** | Option<**bool**> | Specifies whether to return intersecting polygons.  | [optional][default to false]
**attributes** | Option<**Vec<String>**> | List of isochrones attributes - `area` - area of the isochrone polygon - `reachfactor` - reachability score between 0 and 1 This factor is calculated as the ratio between the area of a circle with a radius based on the average speed and the area of the isochrone polygon. - `total_pop` - total population within the polygon The data used for this is the [Global Human Settlement Layer (GHSL)](https://ghsl.jrc.ec.europa.eu/ghs_pop2023.php) from the European Commission. Note that while the dataset was published in 2023, the most recent data contained is from 2020. This is used by the openrouteservice at a resolution of 100m.  | [optional]
**interval** | Option<**f64**> | Interval of isochrones or equidistants. This is only used if a single range value is given. Value in **seconds** for time and **meters** for distance. | [optional]
**smoothing** | Option<**f64**> | Applies a level of generalisation to the isochrone polygons generated as a `smoothing_factor` between `0` and `100.0`. Generalisation is produced by determining a maximum length of a connecting line between two points found on the outside of a containing polygon. If the distance is larger than a threshold value, the line between the two points is removed and a smaller connecting line between other points is used. Note that the minimum length of this connecting line is ~1333m, and so when the `smoothing_factor` results in a distance smaller than this, the minimum value is used. The threshold value is determined as `(maximum_radius_of_isochrone / 100) * smoothing_factor`. Therefore, a value closer to 100 will result in a more generalised shape. The polygon generation algorithm is based on Duckham and al. (2008) `\"Efficient generation of simple polygons for characterizing the shape of a set of points in the plane.\"` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


