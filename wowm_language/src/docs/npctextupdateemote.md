# NpcTextUpdateEmote

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/common.wowm:825`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L825).
```rust,ignore
struct NpcTextUpdateEmote {
    u32 delay;
    u32 emote;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / Little | u32 | delay |  |
| 0x04 | 4 / Little | u32 | emote |  |


Used in:
* [NpcTextUpdate](npctextupdate.md)
* [SMSG_QUESTGIVER_OFFER_REWARD](smsg_questgiver_offer_reward.md)

