# QuestItemReward

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/quest/quest_common.wowm:134`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L134).
```rust,ignore
struct QuestItemReward {
    Item item;
    u32 item_count;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / Little | Item | item |  |
| 0x04 | 4 / Little | u32 | item_count |  |


Used in:
* [SMSG_QUESTGIVER_QUEST_COMPLETE](smsg_questgiver_quest_complete.md)
* [SMSG_QUESTGIVER_QUEST_DETAILS](smsg_questgiver_quest_details.md)
* [SMSG_QUEST_QUERY_RESPONSE](smsg_quest_query_response.md)

