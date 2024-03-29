# Jukejitsu: Cloudflare Edition

### Intro
Jukejitsu is the name of a project I started while attending university and have since used it as a reference implementation in new frameworks/technologies.
The main functionality of jukejitsu is to implement real-time sync'ing of a music/video queue which supports multi-client voting and submission.
The top of the queue is then played by a video/audio player which syncs current playback state between all clients.

### Project specifics
In my effort to experiement with the Cloudflare Workers platform, I had a few objectives:
- experiment with the Cloudflare Pages platform
- create a juniper wrapper, named `juniper_cf_workers`, around the `workers-rs` crate to support running GraphQL servers and merge it upstream with the juniper project
- implement a Durable Object which manages sessions via websockets (sadly `workers-rs` does not have an interface for this yet)
- implement a GraphQL API as a go between for the client application and durable object
- learn more Rust, evaluate Svelte as a front end framework, and learn GraphQL
