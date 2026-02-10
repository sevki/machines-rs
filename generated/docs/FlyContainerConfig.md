# FlyContainerConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cmd** | Option<**Vec<String>**> | CmdOverride is used to override the default command of the image. | [optional]
**depends_on** | Option<[**Vec<models::FlyContainerDependency>**](FlyContainerDependency.md)> | DependsOn can be used to define dependencies between containers. The container will only be started after all of its dependent conditions have been satisfied. | [optional]
**entrypoint** | Option<**Vec<String>**> | EntrypointOverride is used to override the default entrypoint of the image. | [optional]
**env** | Option<**std::collections::HashMap<String, String>**> | ExtraEnv is used to add additional environment variables to the container. | [optional]
**env_from** | Option<[**Vec<models::FlyEnvFrom>**](FlyEnvFrom.md)> | EnvFrom can be provided to set environment variables from machine fields. | [optional]
**exec** | Option<**Vec<String>**> | Image Config overrides - these fields are used to override the image configuration. If not provided, the image configuration will be used. ExecOverride is used to override the default command of the image. | [optional]
**files** | Option<[**Vec<models::FlyFile>**](FlyFile.md)> | Files are files that will be written to the container file system. | [optional]
**healthchecks** | Option<[**Vec<models::FlyContainerHealthcheck>**](FlyContainerHealthcheck.md)> | Healthchecks determine the health of your containers. Healthchecks can use HTTP, TCP or an Exec command. | [optional]
**image** | Option<**String**> | Image is the docker image to run. | [optional]
**name** | Option<**String**> | Name is used to identify the container in the machine. | [optional]
**restart** | Option<[**models::FlyMachineRestart**](FlyMachineRestart.md)> | Restart is used to define the restart policy for the container. NOTE: spot-price is not supported for containers. | [optional]
**secrets** | Option<[**Vec<models::FlyMachineSecret>**](FlyMachineSecret.md)> | Secrets can be provided at the process level to explicitly indicate which secrets should be used for the process. If not provided, the secrets provided at the machine level will be used. | [optional]
**stop** | Option<[**models::FlyStopConfig**](FlyStopConfig.md)> | Stop is used to define the signal and timeout for stopping the container. | [optional]
**user** | Option<**String**> | UserOverride is used to override the default user of the image. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


