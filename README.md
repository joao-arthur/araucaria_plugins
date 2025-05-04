# Araucaria Plugins

A set of optional plugins to make
[Araucaria](https://github.com/joao-arthur/araucaria) usable.

## Installation

```toml
araucaria = { git = "https://github.com/joao-arthur/araucaria", rev = "531f50bd7954db138ff7dcdbc61d03ff6702cd7d" }
araucaria_plugins = { git = "https://github.com/joao-arthur/araucaria_plugins", rev = "4f06fd55c8ab5d0f09602e567f6b509b821e7a37" }
```

## 🚧 TODO

- `Schema::Arr`
- Segregate modules by features
- Create a function that receives `serde_json::Value`, `T: Deserialize`, and
  returns a `T` instance
- `value_from_json_and_schema`
  - parse `.0` float as integer
  - parse number as `date_time` (unixtime)
- Swagger integration
