# DispelledSpell

## Client Version 2.4.3, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm#L29).
```rust,ignore
struct DispelledSpell {
    Spell spell;
    DispelMethod method;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / Little | Spell | spell |  |
| 0x04 | 1 / - | [DispelMethod](dispelmethod.md) | method |  |


Used in:
* [SMSG_SPELLDISPELLOG](smsg_spelldispellog.md)

