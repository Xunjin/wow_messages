# MiniMoveMessage

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm#L11).
```rust,ignore
struct MiniMoveMessage {
    u8 size;
    MiniMoveOpcode opcode;
    PackedGuid guid;
    u32 movement_counter;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 1 / - | u8 | size |  |  |
| 0x01 | 2 / - | [MiniMoveOpcode](minimoveopcode.md) | opcode |  |  |
| 0x03 | - / - | [PackedGuid](../spec/packed-guid.md) | guid |  |  |
| - | 4 / Little | u32 | movement_counter |  |  |
