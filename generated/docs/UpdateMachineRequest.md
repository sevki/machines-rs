# UpdateMachineRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**models::FlyMachineConfig**](FlyMachineConfig.md)> | An object defining the Machine configuration | [optional]
**current_version** | Option<**String**> |  | [optional]
**lease_ttl** | Option<**i32**> |  | [optional]
**lsvd** | Option<**bool**> |  | [optional]
**min_secrets_version** | Option<**i32**> |  | [optional]
**name** | Option<**String**> | Unique name for this Machine. If omitted, one is generated for you | [optional]
**region** | Option<**String**> | The target region. Omitting this param launches in the same region as your WireGuard peer connection (somewhere near you). | [optional]
**skip_launch** | Option<**bool**> |  | [optional]
**skip_secrets** | Option<**bool**> |  | [optional]
**skip_service_registration** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


