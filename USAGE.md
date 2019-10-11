# `slumberd`

Usage documentation for `slumberd`.

Unless `--disable-help` is passed on the command-line, this document is also available at runtime at `/_help` and
`/_usage`.

## CLI Usage

```
slumberd 0.1.0
An HTTP server which sleeps for a specific or random amount of time.

Usage information is available over HTTP at /_help or /_usage; use --disable-help to disable this endpoint.

USAGE:
    slumberd [FLAGS] [OPTIONS]

FLAGS:
        --disable-help    Disable serving usage information at /_help and /_usage. These endpoints will otherwise serve
                          markdown usage information from USAGE.md which is compiled into in the binary.
    -h, --help            Prints help information
    -j, --json            Log in line-delimited JSON format.
    -r, --random          Instead of sleeping for the default sleep time, sleep for a random duration for each request
                          by default. This random duration will be selected between the minimum and maximum sleep times.
    -V, --version         Prints version information
    -v                    Logging verbosity. By default, only INFO and above are logged. Pass once to increase verbosity
                          to DEBUG, twice for TRACE.

OPTIONS:
    -H, --host <host>                 The host to listen on for HTTP requests. [default: 127.0.0.1]
        --max-sleep <max-sleep-ms>    The maximum allowed request sleep time in milliseconds. In random mode, this will
                                      serve as the upper bound for random sleep durations. [default: 30000]
        --min-sleep <min-sleep-ms>    The minimum allowed request sleep time in milliseconds. In random mode, this will
                                      serve as the lower bound for random sleep durations. [default: 15]
    -P, --port <port>                 The port to listen for connections on. [default: 8080]
    -s, --sleep <sleep-ms>            The amount of time to sleep in milliseconds on each request by default. This value
                                      is ignored in random mode. [default: 5000]
```

All times in `slumberd` are measured in milliseconds.

## Setting the Default Mode

The default mode for generic requests is to sleep for `--sleep` milliseconds as specified. If `--random` is passed, the
default mode will be to sleep for a random amount of time between `--min-sleep` and `--max-sleep` milliseconds.

See below for more information on how to change the sleep duration and the sleep mode on a per-request basis.

## Minimum/Maximum Request Durations

The `--min-sleep` and `--max-sleep` settings passed on the command-line are the lowest and highest possible bounds for
request duration. Any requested sleep duration outside of this range will be coerced to fit in this range.

 - If the minimum sleep time is 1000ms and you request a sleep time of 500ms, `slumberd` will sleep for 1000ms.
 - If the maximum sleep time is 1000ms and you request a sleep time of 1500ms, `slumberd` will sleep for 1500ms.

For random sleep requests, these values can be changed on a per-request basis, but these global minimums and maximums
will always be respected.

## Configuration Priorities

`slumberd` makes it possible to configure sleep durations in a number of ways:

 - Path parameters such as `/sleep/{millis}` and `/random/{min_ms}/{max_ms}`.
 - Query-string parameters such as `?time={millis}`, `?min={millis}`, and `?max={millis}`.
 - Headers such as `X-Slumber-Time-Millis`, `X-Slumber-Min-Time-Millis`, and `X-Slumber-Max-Time-Millis`.
 - CLI-specified options.

The sleep mode, fixed versus random, can also be controlled by path based parameters as detailed above, as a
query-string parameter (`?type={fixed|random}`), as a header (`X-Slumber-Type`), or via the CLI.

The priority is in the order as given above:

 1. Path parameters.
 2. Query-string parameters.
 3. Headers.
 4. CLI-specified options.

In most cases, these different configuration sources can be mixed together, but path parameters will supersede
anything else specified.

Example:

```shell
curl -is -H 'X-Slumber-Type: random' http://127.0.0.1:8080/?min=1000
```

This will sleep for a random amount of time with a minimum sleep time of 1000ms, deferring to the `--max-sleep` CLI
argument for the upper bound.

`slumberd` provides all of these different methods of setting configuration per-request in order to provide
compatibility in a variety of contexts:

 - If you're only able to control the request path, you can use the path to specify what you want.
 - If you're only able to control query-string parameters, you can use them to specify what you want.
 - If you're only able to control headers, you can use them to specify what you want.

Often, load-balancer health checks only allow you to specify the request path and possibly query string parameters.
This limitation led to the inspiration of providing as many ways to specify configuration as possible.

The only thing presently not supported is using the request body to specify configuration. This is operating under the
assumption that if you're able to control the request body, you should generally be able to control other parts of the
request.

## Path-Based Routes

`slumberd` provides a few path-based routes that can be used to specify sleep configuration without necessarily
modifying the query string or headers:

 - `/_help`, `/_usage`: Dump this usage information. This can be disabled by passing `--disable-help`.
 - `/sleep/{millis}`: Sleep for the specified amount of milliseconds. Example: `/sleep/500`.
 - `/random`: Sleep for a random amount of time bounded by query-string, header, or CLI-specified minimum and maximum
   durations.
 - `/random/{min_ms}/{max_ms}`: Sleep for a random amount of time between the specified minimum and maximum
   durations. Example: `/random/500/1000`.
 - `/*`: Anything not matching the paths specified above will get a generic handler which allows specifying values via
   the query-string, request headers, or falling back to the CLI-specified options. Example: `/foo/bar`.

## Query-String Parameters

`slumberd` understands the following query-string parameters:

 - `type`: Either `fixed` or `random` to set the sleep mode for the request.
 - `time`: In `fixed` mode, the amount of time in milliseconds to sleep for.
 - `min`: In `random` mode, the minimum amount of time in milliseconds to sleep for.
 - `max`: In `random` mode, the maximum amount of time in milliseconds to sleep for.
 
> **NOTE:** As described above, all time values are coerced to fit in the range of the minimum and maximum request time
> specified on the command-line.

Examples:

 - Fixed Request Duration: `/?type=fixed&time=1000`
 - Random Request Duration: `/?type=random&min=500&max=1000`
 
If `type` is not passed, it will default to the corresponding header value, or, failing that, to the default mode passed
on the command-line.

## Request Headers

`slumberd` understands the following request headers:

 - `X-Slumber-Type`: Either `fixed` or `random` to set the sleep mode for the request.
 - `X-Slumber-Time-Millis`: In `fixed` mode, the amount of time in milliseconds to sleep for.
 - `X-Slumber-Min-Time-Millis`: In `random` mode, the minimum amount of time in milliseconds to sleep for.
 - `X-Slumber-Max-Time-Millis`: In `random` mode, the maximum amount of time in milliseconds to sleep for.

> **NOTE:** As described above, all time values are coerced to fit in the range of the minimum and maximum request time
> specified on the command-line.

## Response Headers

`slumberd` returns metadata about the request in its response headers. 

The following headers are always returned, regardless of sleep mode:

 - `X-Request-Id`: A UUID uniquely identifying the request.
 - `X-Slumber-Type`: Either `fixed` or `random`, identifying the sleep mode for the request.
 - `X-Slumber-Time`: A pretty, human-readable representation of the sleep duration. This is essentially the value
   returned by `Debug` for `Duration`.
 - `X-Slumber-Time-Millis`: The amount of time in milliseconds that the request slept for.

The following headers are only returned in random sleep mode:

 - `X-Slumber-Min-Time`: A human-readable representation of the minimum allowed sleep duration.
 - `X-Slumber-Min-Time-Millis`: The minimum allowed sleep duration in milliseconds.
 - `X-Slumber-Max-Time`: A human-readable representation of the maximum allowed sleep duration.
 - `X-Slumber-Max-Time-Millis`: The maximum allowed sleep duration in milliseconds.
 

## Response Body

`slumberd` always returns a JSON body containing metadata about the request. All fields described here directly
correlate with the response header values described above.

The following properties are always set:

 - `request_id`: A UUID uniquely identifying the request.
 - `slumber.type`: Either `fixed` or `random`, identifying the sleep mode for the request.
 - `slumber.time`: A pretty, human-readable representation of the sleep duration. This is essentially the value
   returned by `Debug` for `Duration`.
 - `slumber.time_millis`: The amount of time in milliseconds that the request slept for.
 
The following properties are only returned in random sleep mode:

 - `slumber.min_time`: A human-readable representation of the minimum allowed sleep duration.
 - `slumber.min_time_millis`: The minimum allowed sleep duration in milliseconds.
 - `slumber.max_time`: A human-readable representation of the maximum allowed sleep duration.
 - `slumber.max_time_millis`: The maximum allowed sleep duration in milliseconds.

### Response Examples

Here is a sample response body for a fixed sleep duration:

```json
{
  "slumber": {
    "type": "fixed",
    "time_millis": 100,
    "time": "100ms"
  },
  "request_id": "1e2b0a75-855d-488f-9d57-9169818729cf"
}
```

Here is a sample response body for a random sleep duration:

```json
{
  "slumber": {
    "type": "random",
    "time_millis": 105,
    "time": "105.11339ms",
    "max_time": "200ms",
    "max_time_millis": 200,
    "min_time_millis": 100,
    "min_time": "100ms"
  },
  "request_id": "4083cabc-a1c6-4e1e-9c1a-df573ff43ae2"
}
```