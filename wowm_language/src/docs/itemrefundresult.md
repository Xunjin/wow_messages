# ItemRefundResult

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/item/smsg_item_refund_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_refund_result.wowm#L1).

```rust,ignore
enum ItemRefundResult : u8 {
    SUCCESS = 0;
    FAILURE = 10;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SUCCESS` | 0 (0x00) |  |  |
| `FAILURE` | 10 (0x0A) |  |  |

Used in:
* [SMSG_ITEM_REFUND_RESULT](smsg_item_refund_result.md)
