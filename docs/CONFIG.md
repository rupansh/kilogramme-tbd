The `config.toml` is written in the [TOML](https://toml.io/en/) format
It has the following sections:

- [`options`](#options) - optional Bot specific options
- [`telegram`](#telegram) - telegram related config
- [`mongo`](#mongo) - mongo related configs

See [config.example.toml](../config.example.toml) for an example config

### `[options]`

- `file_log` - optional flag to enable logging (Default: `true`)

```toml
[options]
file_log = true
```

### `[telegram]`

Please obtain `api_id` & `api_hash` from [my.telegram.org](my.telegram.org) \
ensure that you don't leak the config as these are sensitive keys.

- `api_id` - telegram api id
- `api_hash` - telegram api hash
- `phone` - telegram account's phone in `+{CC}{NUMBER}` format

```toml
[telegram]
api_id = 123456
api_hash = "fffdddddddcccccccbbbbbbbaaaaa444"
phone = "+91123456790-"
```

### `[mongo]`

- `uri` - mongodb server uri

```
[mongo]
uri = "mongodb://localhost:27017"
```
