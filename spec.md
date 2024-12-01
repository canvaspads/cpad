# The CPad file specification

- **Version**: 1
- **Author**: `Hasegawa, Raku <raku@exhibition.jp>`

## Header section

| Field | Type | Size | Value | Value | Description |
| :-- | :-- | :-- | :-- | :-- | :-- |
| Signature | byte | 8 | `CpadFile` | `[ 43 70 61 64 46 69 6C 65 ]` | Begining of the header section. |
| Version | i32 | 4 | `1` | `[ 00 00 00 01 ]` | The file spec version. |
| IsPreviewAvailable | i8 | 1 | `0`: False, `1`: True | |

## Preview section

| Field | Type | Size | Value | Value | Description |
| :-- | :-- | :-- | :-- | :-- | :-- |
| Signature | byte | 8 | `CpadPrev` | `[ 43 70 61 64 50 72 65 76 ]` | Begining of the preview section. |
| PreviewImageSize | u32 | 4 | | | A size of `PreviewImage` |
| PreviewImage | byte | `PreviewImageSize` | | | A small preview image formatted with PNG |

## Sqlite3 section

| Field | Type | Size | Value | Value | Description |
| :-- | :-- | :-- | :-- | :-- | :-- |
| Signature | byte | 8 | `CpSqlite` | `[ 43 70 53 71 6C 69 74 65 ]` | Begining of the Sqlite3 section. |
| SQLite3FileSize | u32 | 4 | | | A size of `SQLite3File` |
| SQLite3File | byte | `SQLite3FileSize` | | | SQLite file |
