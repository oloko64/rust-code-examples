# Requester

A app that floods APIs.

## How to use

```
USAGE:
    requester [OPTIONS] --url <URL>

OPTIONS:
    -b, --body <BODY>
            Request body. JSON only

    -h, --help
            Print help information

    -p, --pool-size <POOL_SIZE>
            Pool size to be used to make the requests. Maximum value recommended is 5000 [default:
            500]

    -r, --request-number <REQUEST_NUMBER>
            Total number of requests to be sent [default: 5000]

    -t, --type-request <TYPE_REQUEST>
            Request type - [GET, POST] [default: GET]

    -u, --url <URL>
            URL to be used to make the requests. Example: https://www.example.com

    -V, --version
            Print version information
```

---

## How to build

```bash
cargo build --release
```
