# \VolumesApi

All URIs are relative to *https://api.machines.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume_snapshot**](VolumesApi.md#create_volume_snapshot) | **POST** /apps/{app_name}/volumes/{volume_id}/snapshots | Create Snapshot
[**volume_delete**](VolumesApi.md#volume_delete) | **DELETE** /apps/{app_name}/volumes/{volume_id} | Destroy Volume
[**volumes_create**](VolumesApi.md#volumes_create) | **POST** /apps/{app_name}/volumes | Create Volume
[**volumes_extend**](VolumesApi.md#volumes_extend) | **PUT** /apps/{app_name}/volumes/{volume_id}/extend | Extend Volume
[**volumes_get_by_id**](VolumesApi.md#volumes_get_by_id) | **GET** /apps/{app_name}/volumes/{volume_id} | Get Volume
[**volumes_list**](VolumesApi.md#volumes_list) | **GET** /apps/{app_name}/volumes | List Volumes
[**volumes_list_snapshots**](VolumesApi.md#volumes_list_snapshots) | **GET** /apps/{app_name}/volumes/{volume_id}/snapshots | List Snapshots
[**volumes_update**](VolumesApi.md#volumes_update) | **PUT** /apps/{app_name}/volumes/{volume_id} | Update Volume



## create_volume_snapshot

> create_volume_snapshot(app_name, volume_id)
Create Snapshot

Create a snapshot for a specific volume within an app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**volume_id** | **String** | Volume ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_delete

> models::Volume volume_delete(app_name, volume_id)
Destroy Volume

Delete a specific volume within an app by volume ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**volume_id** | **String** | Volume ID | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_create

> models::Volume volumes_create(app_name, request)
Create Volume

Create a volume for a specific app using the details provided in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**request** | [**CreateVolumeRequest**](CreateVolumeRequest.md) | Request body | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_extend

> models::ExtendVolumeResponse volumes_extend(app_name, volume_id, request)
Extend Volume

Extend a volume's size within an app using the details provided in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**volume_id** | **String** | Volume ID | [required] |
**request** | [**ExtendVolumeRequest**](ExtendVolumeRequest.md) | Request body | [required] |

### Return type

[**models::ExtendVolumeResponse**](ExtendVolumeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_get_by_id

> models::Volume volumes_get_by_id(app_name, volume_id)
Get Volume

Retrieve details about a specific volume by its ID within an app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**volume_id** | **String** | Volume ID | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_list

> Vec<models::Volume> volumes_list(app_name, summary)
List Volumes

List all volumes associated with a specific app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**summary** | Option<**bool**> | Only return summary info about volumes (omit blocks, block size, etc) |  |

### Return type

[**Vec<models::Volume>**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_list_snapshots

> Vec<models::VolumeSnapshot> volumes_list_snapshots(app_name, volume_id)
List Snapshots

List all snapshots for a specific volume within an app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**volume_id** | **String** | Volume ID | [required] |

### Return type

[**Vec<models::VolumeSnapshot>**](VolumeSnapshot.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_update

> models::Volume volumes_update(app_name, volume_id, request)
Update Volume

Update a volume's configuration using the details provided in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**volume_id** | **String** | Volume ID | [required] |
**request** | [**UpdateVolumeRequest**](UpdateVolumeRequest.md) | Request body | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

