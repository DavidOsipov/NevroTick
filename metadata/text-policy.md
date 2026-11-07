# Metadata UTF-8 and character policy

**Owner instruction:** 7 November 2026, Asia/Tbilisi. The product metadata file
must be UTF-8 without emoji or hidden/nonprintable junk. This policy applies to
raw `product.toml`, including comments, and to decoded authored strings and keys.
It is separate from the GUI localization policy and does not change that policy.

## Selected repertoire

The pipeline's [text policy](pipeline/text_policy.py) requires strict, BOM-free
UTF-8. It rejects malformed, overlong, truncated and surrogate encodings rather
than replacing them. It does not strip, normalize or repair accepted text.

The code-owned table pins Unicode **18.0.0** and rejects:

- `Emoji_Presentation` and `Extended_Pictographic` code points.
- `Default_Ignorable_Code_Point`, including variation selectors, joiners, tags,
  bidi formatting and invisible fillers.
- General categories `Cc`, `Cf`, `Cs`, `Co`, `Cn`, `Zl` and `Zp`: controls,
  formatting, surrogates, private-use/unassigned values and Unicode line/paragraph
  separators. ASCII HT/LF/CR remain document-syntax exceptions; field rules still
  control their permitted locations.
- Combining keycap U+20E3 and every Unicode noncharacter, including U+FDD0..FDEF
  and all plane-ending U+FFFE/U+FFFF values.

Ordinary assigned international letters, marks, numbers and punctuation remain
available, including accents, Cyrillic, CJK and non-BMP letters. Plain `#`, `*`
and decimal digits remain valid; banning the broader `Emoji` property would
incorrectly ban those ordinary characters. U+FFFD is allowed when deliberately
present as text; malformed UTF-8 is never converted into it.

This is a strict no-pictograph policy: bare copyright, registered and trademark
symbols are also `Extended_Pictographic`. Use plain words such as `Copyright`,
`registered` and `trademark` in metadata. Existing notices in other source/data
files retain their original spelling and permission notices.

A TOML escape such as `\U0001F600` is ASCII in the raw source but represents a
forbidden character after parsing. Both raw and decoded admission are mandatory.
The table is independent of Python's host `unicodedata` version. New characters
assigned after the pinned version remain rejected until the policy is reviewed
and updated. This restriction does not detect all confusable spellings, prove
translation truth, authenticate publishers or establish build sandbox isolation.

## Pinned primary sources and provenance

The current [UTS #51 revision 31](https://www.unicode.org/reports/tr51/tr51-31.html)
describes Unicode Emoji 18.0. The exact fixed release data below was fetched as
inert research input over HTTPS on 7 November 2026; the pipeline does not fetch
or parse these files at runtime.

| Source | Locator | Bytes | SHA-256 |
| --- | --- | --- | --- |
| `emoji-data.txt` | [Unicode source](https://www.unicode.org/Public/18.0.0/ucd/emoji/emoji-data.txt) | 108719 | `80d00f8e616a0ef27fd6b8de3b758c06383b5d917e2977709578e68baf733bf1` |
| `DerivedCoreProperties.txt` | [Unicode source](https://www.unicode.org/Public/18.0.0/ucd/DerivedCoreProperties.txt) | 1159889 | `09c928886a178fcafd93c29e4bd59073a058e5a100b716d425cb563ab50f68c9` |
| `DerivedGeneralCategory.txt` | [Unicode source](https://www.unicode.org/Public/18.0.0/ucd/extracted/DerivedGeneralCategory.txt) | 282143 | `d6b151d2d40ee9b1876d26f417980f45ffae47b6055ccf7203cb31f07a030f94` |
| `license.txt` | [Unicode source](https://www.unicode.org/license.txt) | 1995 | `e7a93b009565cfce55919a381437ac4db883e9da2126fa28b91d12732bc53d96` |

The existing GUI Unicode data is version 15.1.0; its scope and hashes remain
unchanged. The metadata policy uses the separately pinned current release.

## Deterministic derivation

1. Strip each source line's `#` comment, then split semicolon-delimited members.
2. Parse each singleton or inclusive hexadecimal range.
3. Select the emoji properties `Emoji_Presentation`, `Extended_Pictographic`;
   core property `Default_Ignorable_Code_Point`; and general categories
   `Cc`, `Cf`, `Cs`, `Co`, `Cn`, `Zl`, `Zp`.
4. Add the singleton U+20E3. Sort all intervals and coalesce overlapping/adjacent
   intervals. The explicit 66-noncharacter check and HT/LF/CR exceptions are
   enforced by code at lookup.
5. Embed the resulting immutable intervals in `text_policy.py`. No candidate
   metadata value can add ranges, redefine this policy or change its version.

The merged table has **867 intervals**, covering
**943179 code points before the HT/LF/CR syntax exceptions**.
Encoding the interval list as ASCII JSON with `separators=(",", ":")` has SHA-256
`2822c400f5939bc78e2807a15975ec71ceee6e791b083672b705948c64da448f`.

The interval structure and digest are checked by independent local tests alongside
literal, escaped, malformed and valid-neighbor controls. Those source-tool checks
prove no native rendering, Windows package, release or complete Unicode security
qualification. Changing the pin requires a reviewed source/digest/range update
and affected controls; never update it automatically during a build.

## Retained Unicode data permission notice

The tables are derived from Unicode data. Preserve this associated permission
notice when distributing this module or its derived data. Original data headers
identify Copyright 2026 Unicode, Inc.; the following is the fetched license text.

```text
UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2026 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.
```
