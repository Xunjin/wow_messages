# AllowedClass

## Client Version 1, Client Version 2

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/enums/allowed_races.wowm:69`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/allowed_races.wowm#L69).

```rust,ignore
flag AllowedClass : u32 {
    ALL = 0;
    WARRIOR = 1;
    PALADIN = 2;
    HUNTER = 4;
    ROGUE = 8;
    PRIEST = 16;
    SHAMAN = 64;
    MAGE = 128;
    WARLOCK = 256;
    DRUID = 1024;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `ALL` | 0 (0x00) |  |  |
| `WARRIOR` | 1 (0x01) |  |  |
| `PALADIN` | 2 (0x02) |  |  |
| `HUNTER` | 4 (0x04) |  |  |
| `ROGUE` | 8 (0x08) |  |  |
| `PRIEST` | 16 (0x10) |  |  |
| `SHAMAN` | 64 (0x40) |  |  |
| `MAGE` | 128 (0x80) |  |  |
| `WARLOCK` | 256 (0x100) |  |  |
| `DRUID` | 1024 (0x400) |  |  |

Used in:
* [SMSG_ITEM_QUERY_SINGLE_RESPONSE](smsg_item_query_single_response.md)
* [SMSG_ITEM_QUERY_SINGLE_RESPONSE](smsg_item_query_single_response.md)
* [SMSG_PETITION_QUERY_RESPONSE](smsg_petition_query_response.md)
# AllowedClass

## Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/enums/allowed_races.wowm:84`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/allowed_races.wowm#L84).

```rust,ignore
flag AllowedClass : u32 {
    ALL = 0;
    WARRIOR = 1;
    PALADIN = 2;
    HUNTER = 4;
    ROGUE = 8;
    PRIEST = 16;
    DEATH_KNIGHT = 32;
    SHAMAN = 64;
    MAGE = 128;
    WARLOCK = 256;
    DRUID = 1024;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `ALL` | 0 (0x00) |  |  |
| `WARRIOR` | 1 (0x01) |  |  |
| `PALADIN` | 2 (0x02) |  |  |
| `HUNTER` | 4 (0x04) |  |  |
| `ROGUE` | 8 (0x08) |  |  |
| `PRIEST` | 16 (0x10) |  |  |
| `DEATH_KNIGHT` | 32 (0x20) |  |  |
| `SHAMAN` | 64 (0x40) |  |  |
| `MAGE` | 128 (0x80) |  |  |
| `WARLOCK` | 256 (0x100) |  |  |
| `DRUID` | 1024 (0x400) |  |  |

Used in:
* [SMSG_ITEM_QUERY_SINGLE_RESPONSE](smsg_item_query_single_response.md)