# WordADay Service

Service to power retrieving new words for users

## Installation

```bash
# CLI tool to lint sql files
$ brew install quarylabs/quary/sqruff
# CLI tool to lint proto bufs
$ brew install bufbuild/buf/buf
```

## TODOs

1. Cache Decoding Key
2. Deserialize JWT to a struct instead of a JSON value
3. Check expiration on JWT and prevent further API calls if expired