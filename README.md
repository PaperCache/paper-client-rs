# paper-client-rs

The Rust [PaperCache](https://papercache.io) client. The client supports all commands described in the wire protocol on the homepage.

## Example
```
use paper_client::PaperClient;

let mut client = PaperClient::new("paper://127.0.0.1:3145")?;

client.set("hello", "world", None)?;
let got = client.get("hello")?;
```
