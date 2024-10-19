# Authentication
In order to be able to call routes that require an API key,
place your keys in `src/api/nonkyc/secret.rs`

example:

```js
pub const PUBLIC_KEY: &'static str = "PUBLIC_KEY_HEX";
pub const SECRET: &'static str = "SECRET_KEY_HEX";
```