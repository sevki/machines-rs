# \AppsApi

All URIs are relative to *https://api.machines.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_create_deploy_token**](AppsApi.md#app_create_deploy_token) | **POST** /apps/{app_name}/deploy_token | Create App deploy token
[**app_ip_assignments_create**](AppsApi.md#app_ip_assignments_create) | **POST** /apps/{app_name}/ip_assignments | Assign new IP address to app
[**app_ip_assignments_delete**](AppsApi.md#app_ip_assignments_delete) | **DELETE** /apps/{app_name}/ip_assignments/{ip} | Remove IP assignment from app
[**app_ip_assignments_list**](AppsApi.md#app_ip_assignments_list) | **GET** /apps/{app_name}/ip_assignments | List IP assignments for app
[**apps_create**](AppsApi.md#apps_create) | **POST** /apps | Create App
[**apps_delete**](AppsApi.md#apps_delete) | **DELETE** /apps/{app_name} | Destroy App
[**apps_list**](AppsApi.md#apps_list) | **GET** /apps | List Apps
[**apps_show**](AppsApi.md#apps_show) | **GET** /apps/{app_name} | Get App



## app_create_deploy_token

> models::CreateAppResponse app_create_deploy_token(app_name, request)
Create App deploy token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**request** | [**CreateAppDeployTokenRequest**](CreateAppDeployTokenRequest.md) | Request body | [required] |

### Return type

[**models::CreateAppResponse**](CreateAppResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_ip_assignments_create

> models::IpAssignment app_ip_assignments_create(request)
Assign new IP address to app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**AssignIpRequest**](AssignIpRequest.md) | Assign IP request | [required] |

### Return type

[**models::IpAssignment**](IPAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_ip_assignments_delete

> app_ip_assignments_delete()
Remove IP assignment from app

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_ip_assignments_list

> models::ListIpAssignmentsResponse app_ip_assignments_list()
List IP assignments for app

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListIpAssignmentsResponse**](listIPAssignmentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_create

> apps_create(request)
Create App

Create an app with the specified details in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**CreateAppRequest**](CreateAppRequest.md) | App body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_delete

> apps_delete(app_name)
Destroy App

Delete an app by its name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list

> models::ListAppsResponse apps_list(org_slug, app_role)
List Apps

List all apps with the ability to filter by organization slug. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | The org slug, or 'personal', to filter apps | [required] |
**app_role** | Option<**String**> | Filter apps by role |  |

### Return type

[**models::ListAppsResponse**](ListAppsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_show

> models::App apps_show(app_name)
Get App

Retrieve details about a specific app by its name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |

### Return type

[**models::App**](App.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

