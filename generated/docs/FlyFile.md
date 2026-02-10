# FlyFile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**guest_path** | Option<**String**> | GuestPath is the path on the machine where the file will be written and must be an absolute path. For example: /full/path/to/file.json | [optional]
**image_config** | Option<**String**> | The name of an image to use the OCI image config as the file contents. | [optional]
**mode** | Option<**i32**> | Mode bits used to set permissions on this file as accepted by chmod(2). | [optional]
**raw_value** | Option<**String**> | The base64 encoded string of the file contents. | [optional]
**secret_name** | Option<**String**> | The name of the secret that contains the base64 encoded file contents. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


