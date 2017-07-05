# Photo Map
_an experiment in automatic photos sorting_

## Current problem

I have several thousands pictures taken during more than 10 years. Some of them are sorted by "event" but a lot of them, coming from my phone have never been properly sorted. They are just uploaded to my NAS without any filtering done at all.

I don't want the situation to keep on going and have decided to develop a tool to see if the solution could not be automated:

- extract geo information from picture: coordinates and geo names
- around the same range of time-date, analyse the coordinates to build events around the same place, at the same time
- determine outliers by trying to find the location where there are no event (photo taken at many different time, with no clear link together)
- the UI should present a timescale and a way to create "photo groups" more or less large. The groups can then be applied to actually create folders and move the photos inside **THIS IS THE ACTUAL GOAL OF THIS PROJECT**

## Challenges

- Rust based
- Able to load 10k+ photos quickly-ish (no database)
- Work across "slow" network link
- Websocket, really?
- UI?

## How to run

```
cargo build
# load all picture in the sample folder. Feel free to put a symlink here
cargo run --bin cli
```

## What do we use

- [patched](https://github.com/octplane/rust-reverse-geocoder) version of [rust-reverse-geocoder](https://github.com/llambda/rust-reverse-geocoder)
- [rexif](https://crates.io/crates/rexif)
- [websocket](https://crates.io/crates/websocket)

## Current Work

- see [the roadmap](https://github.com/octplane/photo-map/issues/1)