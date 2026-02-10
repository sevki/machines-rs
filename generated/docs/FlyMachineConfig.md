# FlyMachineConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_destroy** | Option<**bool**> | Optional boolean telling the Machine to destroy itself once it’s complete (default false) | [optional]
**checks** | Option<[**std::collections::HashMap<String, models::FlyMachineCheck>**](FlyMachineCheck.md)> | An optional object that defines one or more named top-level checks. The key for each check is the check name. | [optional]
**containers** | Option<[**Vec<models::FlyContainerConfig>**](FlyContainerConfig.md)> | Containers are a list of containers that will run in the machine. Currently restricted to only specific organizations. | [optional]
**disable_machine_autostart** | Option<**bool**> | Deprecated: use Service.Autostart instead | [optional]
**dns** | Option<[**models::FlyDnsConfig**](FlyDNSConfig.md)> |  | [optional]
**env** | Option<**std::collections::HashMap<String, String>**> | An object filled with key/value pairs to be set as environment variables | [optional]
**files** | Option<[**Vec<models::FlyFile>**](FlyFile.md)> |  | [optional]
**guest** | Option<[**models::FlyMachineGuest**](FlyMachineGuest.md)> |  | [optional]
**image** | Option<**String**> | The docker image to run | [optional]
**init** | Option<[**models::FlyMachineInit**](FlyMachineInit.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**metrics** | Option<[**models::FlyMachineMetrics**](FlyMachineMetrics.md)> |  | [optional]
**mounts** | Option<[**Vec<models::FlyMachineMount>**](FlyMachineMount.md)> |  | [optional]
**processes** | Option<[**Vec<models::FlyMachineProcess>**](FlyMachineProcess.md)> |  | [optional]
**restart** | Option<[**models::FlyMachineRestart**](FlyMachineRestart.md)> |  | [optional]
**rootfs** | Option<[**models::FlyMachineRootfs**](FlyMachineRootfs.md)> |  | [optional]
**schedule** | Option<**String**> |  | [optional]
**services** | Option<[**Vec<models::FlyMachineService>**](FlyMachineService.md)> |  | [optional]
**size** | Option<**String**> | Deprecated: use Guest instead | [optional]
**standbys** | Option<**Vec<String>**> | Standbys enable a machine to be a standby for another. In the event of a hardware failure, the standby machine will be started. | [optional]
**statics** | Option<[**Vec<models::FlyStatic>**](FlyStatic.md)> |  | [optional]
**stop_config** | Option<[**models::FlyStopConfig**](FlyStopConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


