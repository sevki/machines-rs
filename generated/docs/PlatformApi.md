# \PlatformApi

All URIs are relative to *https://api.machines.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**platform_placements_post**](PlatformApi.md#platform_placements_post) | **POST** /platform/placements | Get Placements
[**platform_regions_get**](PlatformApi.md#platform_regions_get) | **GET** /platform/regions | Get Regions



## platform_placements_post

> models::MainGetPlacementsResponse platform_placements_post(request)
Get Placements

Simulates placing the specified number of machines into regions, depending on available capacity and limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**MainGetPlacementsRequest**](MainGetPlacementsRequest.md) | Get placements request | [required] |

### Return type

[**models::MainGetPlacementsResponse**](main.getPlacementsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_regions_get

> models::MainRegionResponse platform_regions_get(size, cpu_kind, memory_mb, cpus, gpus, gpu_kind)
Get Regions

List all regions on the platform with their current Machine capacity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**size** | Option<**String**> | guest machine size preset. default performance-1x |  |
**cpu_kind** | Option<**String**> | guest CPU kind |  |
**memory_mb** | Option<**i32**> | guest memory in megabytes |  |
**cpus** | Option<**i32**> | guest CPU count |  |
**gpus** | Option<**i32**> | guest GPU count |  |
**gpu_kind** | Option<**String**> | guest GPU kind |  |

### Return type

[**models::MainRegionResponse**](main.regionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

