# CreateVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_backup_enabled** | Option<**bool**> | enable scheduled automatic snapshots. Defaults to `true` | [optional]
**compute** | Option<[**models::FlyMachineGuest**](FlyMachineGuest.md)> |  | [optional]
**compute_image** | Option<**String**> |  | [optional]
**encrypted** | Option<**bool**> |  | [optional]
**fstype** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**region** | Option<**String**> |  | [optional]
**require_unique_zone** | Option<**bool**> |  | [optional]
**size_gb** | Option<**i32**> |  | [optional]
**snapshot_id** | Option<**String**> | restore from snapshot | [optional]
**snapshot_retention** | Option<**i32**> |  | [optional]
**source_volume_id** | Option<**String**> | fork from remote volume | [optional]
**unique_zone_app_wide** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


