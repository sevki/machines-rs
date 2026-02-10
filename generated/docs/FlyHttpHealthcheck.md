# FlyHttpHealthcheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**headers** | Option<[**Vec<models::FlyMachineHttpHeader>**](FlyMachineHTTPHeader.md)> | Additional headers to send with the request | [optional]
**method** | Option<**String**> | The HTTP method to use to when making the request | [optional]
**path** | Option<**String**> | The path to send the request to | [optional]
**port** | Option<**i32**> | The port to connect to, often the same as internal_port | [optional]
**scheme** | Option<[**models::FlyContainerHealthcheckScheme**](FlyContainerHealthcheckScheme.md)> | Whether to use http or https | [optional]
**tls_server_name** | Option<**String**> | If the protocol is https, the hostname to use for TLS certificate validation | [optional]
**tls_skip_verify** | Option<**bool**> | If the protocol is https, whether or not to verify the TLS certificate | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


