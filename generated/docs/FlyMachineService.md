# FlyMachineService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autostart** | Option<**bool**> |  | [optional]
**autostop** | Option<**Autostop**> | Accepts a string (new format) or a boolean (old format). For backward compatibility with older clients, the API continues to use booleans for \"off\" and \"stop\" in responses. * \"off\" or false - Do not autostop the Machine. * \"stop\" or true - Automatically stop the Machine. * \"suspend\" - Automatically suspend the Machine, falling back to a full stop if this is not possible. (enum: off, stop, suspend) | [optional]
**checks** | Option<[**Vec<models::FlyMachineServiceCheck>**](FlyMachineServiceCheck.md)> | An optional list of service checks | [optional]
**concurrency** | Option<[**models::FlyMachineServiceConcurrency**](FlyMachineServiceConcurrency.md)> |  | [optional]
**force_instance_description** | Option<**String**> |  | [optional]
**force_instance_key** | Option<**String**> |  | [optional]
**internal_port** | Option<**i32**> |  | [optional]
**min_machines_running** | Option<**i32**> |  | [optional]
**ports** | Option<[**Vec<models::FlyMachinePort>**](FlyMachinePort.md)> |  | [optional]
**protocol** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


