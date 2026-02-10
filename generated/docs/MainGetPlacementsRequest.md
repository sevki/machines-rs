# MainGetPlacementsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**compute** | Option<[**models::FlyMachineGuest**](FlyMachineGuest.md)> | Resource requirements for the Machine to simulate. Defaults to a performance-1x machine | [optional]
**count** | Option<**i32**> | Number of machines to simulate placement. Defaults to 0, which returns the org-specific limit for each region. | [optional]
**org_slug** | **String** |  | 
**region** | Option<**String**> | Region expression for placement as a comma-delimited set of regions or aliases. Defaults to \"[region],any\", to prefer the API endpoint's local region with any other region as fallback. | [optional]
**volume_name** | Option<**String**> |  | [optional]
**volume_size_bytes** | Option<**i32**> |  | [optional]
**weights** | Option<**std::collections::HashMap<String, i32>**> | Optional weights to override default placement preferences. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


