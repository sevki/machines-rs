# FlyContainerHealthcheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exec** | Option<[**models::FlyExecHealthcheck**](FlyExecHealthcheck.md)> |  | [optional]
**failure_threshold** | Option<**i32**> | The number of times the check must fail before considering the container unhealthy. | [optional]
**grace_period** | Option<**i32**> | The time in seconds to wait after a container starts before checking its health. | [optional]
**http** | Option<[**models::FlyHttpHealthcheck**](FlyHTTPHealthcheck.md)> |  | [optional]
**interval** | Option<**i32**> | The time in seconds between executing the defined check. | [optional]
**kind** | Option<[**models::FlyContainerHealthcheckKind**](FlyContainerHealthcheckKind.md)> | Kind of healthcheck (readiness, liveness) | [optional]
**name** | Option<**String**> | The name of the check. Must be unique within the container. | [optional]
**success_threshold** | Option<**i32**> | The number of times the check must succeeed before considering the container healthy. | [optional]
**tcp** | Option<[**models::FlyTcpHealthcheck**](FlyTCPHealthcheck.md)> |  | [optional]
**timeout** | Option<**i32**> | The time in seconds to wait for the check to complete. | [optional]
**unhealthy** | Option<[**models::FlyUnhealthyPolicy**](FlyUnhealthyPolicy.md)> | Unhealthy policy that determines what action to take if a container is deemed unhealthy | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


