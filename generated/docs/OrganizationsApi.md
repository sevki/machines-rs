# \OrganizationsApi

All URIs are relative to *https://api.machines.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**machines_org_list**](OrganizationsApi.md#machines_org_list) | **GET** /orgs/{org_slug}/machines | List All Machines



## machines_org_list

> models::OrgMachinesResponse machines_org_list(org_slug, include_deleted, region, state, updated_after, cursor, limit)
List All Machines

List all Machines associated with a specific organization. Machines are sorted by their `updated_at` timestamps, oldest to newest.  This API call represents \"a point in time\". Recent machine changes, including creations and destructions, may take time to propagate. When polling with `updated_after`, offset your timestamps to catch late-arriving events. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Fly Organization Slug | [required] |
**include_deleted** | Option<**bool**> | Include deleted machines |  |
**region** | Option<**String**> | Region filter |  |
**state** | Option<**String**> | Comma separated list of states to filter (created, started, stopped, suspended) |  |
**updated_after** | Option<**String**> | Only return machines updated after this time. Timestamp must be in the RFC 3339 format |  |
**cursor** | Option<**String**> | Pagination cursor from previous response (takes precedence over updated_after) |  |
**limit** | Option<**i32**> | The number of machines to fetch (max of 1000). If omitted, this is set to 500 by default |  |

### Return type

[**models::OrgMachinesResponse**](OrgMachinesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

