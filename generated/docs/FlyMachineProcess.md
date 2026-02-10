# FlyMachineProcess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cmd** | Option<**Vec<String>**> |  | [optional]
**entrypoint** | Option<**Vec<String>**> |  | [optional]
**env** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**env_from** | Option<[**Vec<models::FlyEnvFrom>**](FlyEnvFrom.md)> | EnvFrom can be provided to set environment variables from machine fields. | [optional]
**exec** | Option<**Vec<String>**> |  | [optional]
**ignore_app_secrets** | Option<**bool**> | IgnoreAppSecrets can be set to true to ignore the secrets for the App the Machine belongs to and only use the secrets provided at the process level. The default/legacy behavior is to use the secrets provided at the App level. | [optional]
**secrets** | Option<[**Vec<models::FlyMachineSecret>**](FlyMachineSecret.md)> | Secrets can be provided at the process level to explicitly indicate which secrets should be used for the process. If not provided, the secrets provided at the machine level will be used. | [optional]
**user** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


