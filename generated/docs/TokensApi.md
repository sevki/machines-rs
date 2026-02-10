# \TokensApi

All URIs are relative to *https://api.machines.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**current_token_show**](TokensApi.md#current_token_show) | **GET** /v1/tokens/current | Get Current Token Information
[**tokens_request_kms**](TokensApi.md#tokens_request_kms) | **POST** /tokens/kms | Request a Petsem token for accessing KMS
[**tokens_request_oidc**](TokensApi.md#tokens_request_oidc) | **POST** /tokens/oidc | Request an OIDC token



## current_token_show

> models::CurrentTokenResponse current_token_show()
Get Current Token Information

Get information about the current macaroon token(s), including organizations, apps, and whether each token is from a user or machine

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CurrentTokenResponse**](CurrentTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_request_kms

> String tokens_request_kms()
Request a Petsem token for accessing KMS

This site hosts documentation generated from the Fly.io Machines API OpenAPI specification. Visit our complete [Machines API docs](https://fly.io/docs/machines/api/apps-resource/) for details about using the Apps resource.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_request_oidc

> String tokens_request_oidc(request)
Request an OIDC token

Request an Open ID Connect token for your machine. Customize the audience claim with the `aud` parameter. This returns a JWT token. Learn more about [using OpenID Connect](/docs/reference/openid-connect/) on Fly.io. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**CreateOidcTokenRequest**](CreateOidcTokenRequest.md) | Optional request body | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

