# Hash delivery network cache server

This server respresents node in between user and [data server](https://github.com/MetaGigachad/hdn-data-server/). 
It has it's own 
[database](https://docs.rs/sled/latest/sled/) of
hashes which is updated from data server only on demand.

This project uses [tokio](https://docs.rs/tokio/latest/tokio/) as a runtime and
[sled](https://docs.rs/sled/latest/sled/) as it's database to be blazingly fast.

> ATTENTION: This server doesn't support standalone mode. It means that [data server](https://github.com/MetaGigachad/hdn-data-server/) should be
> accessable for this server to run.

# Configuration

By default server will create configuration file in usual directory for your OS 
(on *nix it will be `$XDG_CONFIG_HOME/hdn-cache-server/default-config.toml`).
If you wish to use another config you can provide its path through `--config` parameter.
Note that if none such file exists it will be created with default parameters.

Database will also auto create it's files if none exist.

## Default config
```toml
listener_addr = '127.0.0.1:9002'
data_server_addr = '127.0.0.1:9001'
db_dir = 'data' # Any path can be provided here
```

# Network Deploy

Shortcut: defaults allow you to run one data and one cache server locally without any 
configuration.

1. Run [data server](https://github.com/MetaGigachad/hdn-data-server/) with appropriate configuration
2. Configure [cache servers](https://github.com/MetaGigachad/hdn-cache-server/) with address of data server
3. Run [cache servers](https://github.com/MetaGigachad/hdn-cache-server/) with their configuration

# Communication with user

Cache server supports two types of requests in form of json's:
## Store
Request
```json
{
 "request_type": "store",
 "key": "some_key",
 "hash": "0b672dd94fd3da6a8d404b66ee3f0c83"
}
```
Response
```json
{
 "response_status": "success"
}
```

## Load
Request
```json
{
 "request_type": "load",
 "key": "some_key"
}
```
Responses
```json
{
 "response_status": "success",
 "requested_key": "some_key",
 "requested_hash": "0b672dd94fd3da6a8d404b66ee3f0c83",
}
{
 "response_status": "key not found",
}
```

# Communication with data server

Communication with data server uses same types of requests in more optimal scheme and uses
[postcard](https://docs.rs/postcard/latest/postcard/) format. See [`crate::messages::data_server`](crate::message::data_server) for more details.
