# SMSG_CHAT_PLAYER_AMBIGUOUS

## Client Version 2.4.3, Client Version 3

### Comment

Never actually sent in any emulator.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_chat_player_ambiguous.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_chat_player_ambiguous.wowm#L1).
```rust,ignore
smsg SMSG_CHAT_PLAYER_AMBIGUOUS = 0x032D {
    CString player;
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
| 0x04 | - / - | CString | player |  |  |
