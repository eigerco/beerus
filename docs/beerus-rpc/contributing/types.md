# JSON-RPC API Types

## Hex encoding

Every value is Hex encoded, starts with `0x` and contains at least one
hexadecimal digit

- ✅ 0x1
- ✅ 0x01
- ✅ 0x0
- ❌ 0x
- ❌ 0x4000 // no leading zeroes allowed
- ❌ ff // must be prefixed with `0x`

## Unformatted data

When encoding unformatted data (byte arrays, account addresses, hashes, bytecode
arrays): encode as hex, prefix with `0x`, two hex digits per byte.

- ❌ 0xf0f0f // not even number of digits