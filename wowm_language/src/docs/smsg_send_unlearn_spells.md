# SMSG_SEND_UNLEARN_SPELLS

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_send_unlearn_spells.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_send_unlearn_spells.wowm#L1).
```rust,ignore
smsg SMSG_SEND_UNLEARN_SPELLS = 0x041D {
    u32 amount_of_spells;
    u32[amount_of_spells] spells;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | amount_of_spells |  |  |
| 0x08 | ? / - | u32[amount_of_spells] | spells |  |  |

# SMSG_SEND_UNLEARN_SPELLS

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_send_unlearn_spells.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_send_unlearn_spells.wowm#L8).
```rust,ignore
smsg SMSG_SEND_UNLEARN_SPELLS = 0x041E {
    u32 amount_of_spells;
    u32[amount_of_spells] spells;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | amount_of_spells |  |  |
| 0x08 | ? / - | u32[amount_of_spells] | spells |  |  |
