# Relation

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L19).
```rust,ignore
struct Relation {
    Guid guid;
    RelationType relation_mask;
    CString note;
    if (relation_mask & FRIEND) {
        FriendStatus status;
        if (status == ONLINE) {
            Area area;
            u32 level;
            Class class;
        }
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x08 | 4 / - | [RelationType](relationtype.md) | relation_mask |  |  |
| 0x0C | - / - | CString | note |  |  |

If relation_mask contains `FRIEND`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 1 / - | [FriendStatus](friendstatus.md) | status |  |  |

If status is equal to `ONLINE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / - | [Area](area.md) | area |  |  |
| - | 4 / Little | u32 | level |  |  |
| - | 4 / - | [Class](class.md) | class |  |  |
