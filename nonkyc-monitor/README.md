# Render nonkyc hodlings for small cap altcoins

This is an example application for my library [marketeer](https://github.com/jonas089/marketeer/tree/master)

This app will query your nonkyc account for all holdings worth more than `10 USDT` and print them to the console.

It will repeat this every 3 seconds.

You can play with the minimum value and timer in `main.rs`.

Error handling is minimal and failed queries are ignored but shouldn't cause this app to crash since they'll just return `None` for now.

# Running the app

First make sure to specify your nonkyc API keys in your environment.

Example `~/.bashrc`:

```js
export NON_KYC_PUBLIC_KEY="PUBLIC_KEY"
export NON_KYC_SECRET_KEY="SECRET_KEY"
```

Now you are ready to run the app:

```
cargo run
```