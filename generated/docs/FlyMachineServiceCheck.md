# FlyMachineServiceCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**grace_period** | Option<[**models::FlyDuration**](FlyDuration.md)> | The time to wait after a VM starts before checking its health | [optional]
**headers** | Option<[**Vec<models::FlyMachineHttpHeader>**](FlyMachineHTTPHeader.md)> |  | [optional]
**interval** | Option<[**models::FlyDuration**](FlyDuration.md)> | The time between connectivity checks | [optional]
**method** | Option<**String**> | For http checks, the HTTP method to use to when making the request | [optional]
**path** | Option<**String**> | For http checks, the path to send the request to | [optional]
**port** | Option<**i32**> | The port to connect to, often the same as internal_port | [optional]
**protocol** | Option<**String**> | For http checks, whether to use http or https | [optional]
**timeout** | Option<[**models::FlyDuration**](FlyDuration.md)> | The maximum time a connection can take before being reported as failing its health check | [optional]
**tls_server_name** | Option<**String**> | If the protocol is https, the hostname to use for TLS certificate validation | [optional]
**tls_skip_verify** | Option<**bool**> | For http checks with https protocol, whether or not to verify the TLS certificate | [optional]
**r#type** | Option<**String**> | tcp or http | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


