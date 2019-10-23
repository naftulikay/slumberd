# slumberd [![Build Status][travis.svg]][travis] [![Docker Status][docker.svg]][docker]

An HTTP server which sleeps for a configurable amount of time, largely useful as a way to test functionality of HTTP
proxies when dealing with long-lived requests.

Available on Docker Hub as [`naftulikay/slumberd`][docker].

## Usage

`slumberd` is extremely flexible and can provide static or random request durations configurable via the CLI, specific
request paths, query string parameters, and/or request headers. `slumberd` also returns a JSON response describing
what it did, and sets response headers providing similar information.

#### Sleep for 500ms

```
$ curl -i http://127.0.0.1:8080/sleep/500
HTTP/1.1 200 OK
content-length: 143
x-request-id: 4699f361-3f53-4de9-ae01-1f36e8316cf5
content-type: application/json
x-slumber-time-millis: 500
x-slumber-type: fixed
x-slumber-time: 500ms
date: Wed, 23 Oct 2019 21:17:37 GMT

{
  "slumber": {
    "type": "fixed",
    "time_millis": 500,
    "time": "500ms"
  },
  "request_id": "4699f361-3f53-4de9-ae01-1f36e8316cf5"
}
```

#### Sleep for a Random Duration Between 500-1000ms

```
$ curl -i http://127.0.0.1:8080/random/500/1000
HTTP/1.1 200 OK
content-length: 255
x-slumber-min-time-millis: 500
x-request-id: c4f2c297-f95c-4e14-9e1b-7f634ff2afdd
x-slumber-min-time: 500ms
content-type: application/json
x-slumber-max-time-millis: 1000
x-slumber-max-time: 1s
x-slumber-time-millis: 541
x-slumber-type: random
x-slumber-time: 541.962699ms
date: Wed, 23 Oct 2019 21:18:16 GMT

{
  "slumber": {
    "type": "random",
    "time_millis": 541,
    "time": "541.962699ms",
    "max_time": "1s",
    "max_time_millis": 1000,
    "min_time_millis": 500,
    "min_time": "500ms"
  },
  "request_id": "c4f2c297-f95c-4e14-9e1b-7f634ff2afdd"
}
```

**For complete usage information**, see [USAGE.md](./USAGE.md).

## Performance and Portability

`slumberd` uses [`actix-web`][actix-web] as a platform and uses Rust's zero-cost futures to "sleep" on each request.
Individual requests _do not_ block the thread that they are executing on, and as such, `slumberd` should scale fairly
linearly with the amount of requests it receives versus the available network bandwidth and number of logical CPU cores
available on the host.

 - One operating system thread per logical CPU is spawned, with each running a Tokio event loop.
 - Memory usage on cold boot is around 25MiB. I have not profiled `slumberd` under load, but similar Actix applications
   I have written in the past will usually remain in the realm of ~100-200MiB under load.
 - The `--release` stripped static binary for Linux at time of writing is 4.06MiB. The Docker image should also be of
   a similar size as it contains nothing but the binary.
 - The static `musl` binary has _zero_ system requirements other than an x86_64 architecture and any modern Linux kernel
   (I'm assuming that any kernel version >=2.6 should work just fine).

## License

Licensed at your discretion under either:

 - [Apache Software License, Version 2.0](./LICENSE-APACHE)
 - [MIT License](./LICENSE-MIT)

 [actix-web]: https://github.com/actix/actix-web
 [docker]: https://cloud.docker.com/repository/docker/naftulikay/slumberd
 [docker.svg]: https://img.shields.io/docker/cloud/build/naftulikay/slumberd.svg
 [travis]: https://travis-ci.org/naftulikay/slumberd
 [travis.svg]: https://travis-ci.org/naftulikay/slumberd.svg?branch=master
