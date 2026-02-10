# FlyMachineRestart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gpu_bid_price** | Option<**f64**> | GPU bid price for spot Machines. | [optional]
**max_retries** | Option<**i32**> | When policy is on-failure, the maximum number of times to attempt to restart the Machine before letting it stop. | [optional]
**policy** | Option<**Policy**> | * no - Never try to restart a Machine automatically when its main process exits, whether that’s on purpose or on a crash. * always - Always restart a Machine automatically and never let it enter a stopped state, even when the main process exits cleanly. * on-failure - Try up to MaxRetries times to automatically restart the Machine if it exits with a non-zero exit code. Default when no explicit policy is set, and for Machines with schedules. * spot-price - Starts the Machine only when there is capacity and the spot price is less than or equal to the bid price. (enum: no, always, on-failure, spot-price) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


