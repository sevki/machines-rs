# Machine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checks** | Option<[**Vec<models::CheckStatus>**](CheckStatus.md)> |  | [optional]
**config** | Option<[**models::FlyMachineConfig**](FlyMachineConfig.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**events** | Option<[**Vec<models::MachineEvent>**](MachineEvent.md)> |  | [optional]
**host_status** | Option<**HostStatus**> |  (enum: ok, unknown, unreachable) | [optional]
**id** | Option<**String**> |  | [optional]
**image_ref** | Option<[**models::ImageRef**](ImageRef.md)> |  | [optional]
**incomplete_config** | Option<[**models::FlyMachineConfig**](FlyMachineConfig.md)> |  | [optional]
**instance_id** | Option<**String**> | InstanceID is unique for each version of the machine | [optional]
**name** | Option<**String**> |  | [optional]
**nonce** | Option<**String**> | Nonce is only every returned on machine creation if a lease_duration was provided. | [optional]
**private_ip** | Option<**String**> | PrivateIP is the internal 6PN address of the machine. | [optional]
**region** | Option<**String**> |  | [optional]
**state** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


