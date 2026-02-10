# \SecretsApi

All URIs are relative to *https://api.machines.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secret_create**](SecretsApi.md#secret_create) | **POST** /apps/{app_name}/secrets/{secret_name} | Create or update Secret
[**secret_delete**](SecretsApi.md#secret_delete) | **DELETE** /apps/{app_name}/secrets/{secret_name} | Delete an app secret
[**secret_get**](SecretsApi.md#secret_get) | **GET** /apps/{app_name}/secrets/{secret_name} | Get an app secret
[**secretkey_decrypt**](SecretsApi.md#secretkey_decrypt) | **POST** /apps/{app_name}/secretkeys/{secret_name}/decrypt | Decrypt with a secret key
[**secretkey_delete**](SecretsApi.md#secretkey_delete) | **DELETE** /apps/{app_name}/secretkeys/{secret_name} | Delete an app's secret key
[**secretkey_encrypt**](SecretsApi.md#secretkey_encrypt) | **POST** /apps/{app_name}/secretkeys/{secret_name}/encrypt | Encrypt with a secret key
[**secretkey_generate**](SecretsApi.md#secretkey_generate) | **POST** /apps/{app_name}/secretkeys/{secret_name}/generate | Generate a random secret key
[**secretkey_get**](SecretsApi.md#secretkey_get) | **GET** /apps/{app_name}/secretkeys/{secret_name} | Get an app's secret key
[**secretkey_set**](SecretsApi.md#secretkey_set) | **POST** /apps/{app_name}/secretkeys/{secret_name} | Create or update a secret key
[**secretkey_sign**](SecretsApi.md#secretkey_sign) | **POST** /apps/{app_name}/secretkeys/{secret_name}/sign | Sign with a secret key
[**secretkey_verify**](SecretsApi.md#secretkey_verify) | **POST** /apps/{app_name}/secretkeys/{secret_name}/verify | Verify with a secret key
[**secretkeys_list**](SecretsApi.md#secretkeys_list) | **GET** /apps/{app_name}/secretkeys | List secret keys belonging to an app
[**secrets_list**](SecretsApi.md#secrets_list) | **GET** /apps/{app_name}/secrets | List app secrets belonging to an app
[**secrets_update**](SecretsApi.md#secrets_update) | **POST** /apps/{app_name}/secrets | Update app secrets belonging to an app



## secret_create

> models::SetAppSecretResponse secret_create(app_name, secret_name, request)
Create or update Secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | App secret name | [required] |
**request** | [**SetAppSecretRequest**](SetAppSecretRequest.md) | Create app secret request | [required] |

### Return type

[**models::SetAppSecretResponse**](SetAppSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_delete

> models::DeleteAppSecretResponse secret_delete(app_name, secret_name)
Delete an app secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | App secret name | [required] |

### Return type

[**models::DeleteAppSecretResponse**](DeleteAppSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_get

> models::AppSecret secret_get(app_name, secret_name, min_version, show_secrets)
Get an app secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | App secret name | [required] |
**min_version** | Option<**String**> | Minimum secrets version to return. Returned when setting a new secret |  |
**show_secrets** | Option<**bool**> | Show the secret value. |  |

### Return type

[**models::AppSecret**](AppSecret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkey_decrypt

> models::DecryptSecretkeyResponse secretkey_decrypt(app_name, secret_name, request, min_version)
Decrypt with a secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | Secret key name | [required] |
**request** | [**DecryptSecretkeyRequest**](DecryptSecretkeyRequest.md) | Decrypt with secret key request | [required] |
**min_version** | Option<**String**> | Minimum secrets version to return. Returned when setting a new secret |  |

### Return type

[**models::DecryptSecretkeyResponse**](DecryptSecretkeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkey_delete

> models::DeleteSecretkeyResponse secretkey_delete(app_name, secret_name)
Delete an app's secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | Secret key name | [required] |

### Return type

[**models::DeleteSecretkeyResponse**](DeleteSecretkeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkey_encrypt

> models::EncryptSecretkeyResponse secretkey_encrypt(app_name, secret_name, request, min_version)
Encrypt with a secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | Secret key name | [required] |
**request** | [**EncryptSecretkeyRequest**](EncryptSecretkeyRequest.md) | Encrypt with secret key request | [required] |
**min_version** | Option<**String**> | Minimum secrets version to return. Returned when setting a new secret |  |

### Return type

[**models::EncryptSecretkeyResponse**](EncryptSecretkeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkey_generate

> models::SetSecretkeyResponse secretkey_generate(app_name, secret_name, request)
Generate a random secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | Secret key name | [required] |
**request** | [**SetSecretkeyRequest**](SetSecretkeyRequest.md) | generate secret key request | [required] |

### Return type

[**models::SetSecretkeyResponse**](SetSecretkeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkey_get

> models::SecretKey secretkey_get(app_name, secret_name, min_version)
Get an app's secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | Secret key name | [required] |
**min_version** | Option<**String**> | Minimum secrets version to return. Returned when setting a new secret |  |

### Return type

[**models::SecretKey**](SecretKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkey_set

> models::SetSecretkeyResponse secretkey_set(app_name, secret_name, request)
Create or update a secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | Secret key name | [required] |
**request** | [**SetSecretkeyRequest**](SetSecretkeyRequest.md) | Create secret key request | [required] |

### Return type

[**models::SetSecretkeyResponse**](SetSecretkeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkey_sign

> models::SignSecretkeyResponse secretkey_sign(app_name, secret_name, request, min_version)
Sign with a secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | Secret key name | [required] |
**request** | [**SignSecretkeyRequest**](SignSecretkeyRequest.md) | Sign with secret key request | [required] |
**min_version** | Option<**String**> | Minimum secrets version to return. Returned when setting a new secret |  |

### Return type

[**models::SignSecretkeyResponse**](SignSecretkeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkey_verify

> secretkey_verify(app_name, secret_name, request, min_version)
Verify with a secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**secret_name** | **String** | Secret key name | [required] |
**request** | [**VerifySecretkeyRequest**](VerifySecretkeyRequest.md) | Verify with secret key request | [required] |
**min_version** | Option<**String**> | Minimum secrets version to return. Returned when setting a new secret |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretkeys_list

> models::SecretKeys secretkeys_list(app_name, min_version, types)
List secret keys belonging to an app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**min_version** | Option<**String**> | Minimum secrets version to return. Returned when setting a new secret |  |
**types** | Option<**String**> | Comma-seperated list of secret keys to list |  |

### Return type

[**models::SecretKeys**](SecretKeys.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_list

> models::AppSecrets secrets_list(app_name, min_version, show_secrets)
List app secrets belonging to an app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**min_version** | Option<**String**> | Minimum secrets version to return. Returned when setting a new secret |  |
**show_secrets** | Option<**bool**> | Show the secret values. |  |

### Return type

[**models::AppSecrets**](AppSecrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_update

> models::AppSecretsUpdateResp secrets_update(app_name, request)
Update app secrets belonging to an app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | Fly App Name | [required] |
**request** | [**AppSecretsUpdateRequest**](AppSecretsUpdateRequest.md) | Update app secret request, with values to set, or nil to unset | [required] |

### Return type

[**models::AppSecretsUpdateResp**](AppSecretsUpdateResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

