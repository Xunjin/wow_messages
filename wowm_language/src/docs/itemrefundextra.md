# ItemRefundExtra

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/item/smsg_item_refund_info_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_refund_info_response.wowm#L1).
```rust,ignore
struct ItemRefundExtra {
    Item item;
    u32 amount;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / Little | Item | item |  |
| 0x04 | 4 / Little | u32 | amount |  |


Used in:
* [SMSG_ITEM_REFUND_INFO_RESPONSE](smsg_item_refund_info_response.md)
* [SMSG_ITEM_REFUND_RESULT](smsg_item_refund_result.md)

