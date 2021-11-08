# Jukejitsu: Cloudflare Edition

Demo (wip, beware: it ugly): https://cloudflare-pages-workers-vue3-graphql.pages.dev/

### Intro
Jukejitsu is the name of a project I started while attending university and have since used it as a reference implementation in new frameworks/technologies.
The main functionality of jukejitsu is to implement real-time sync'ing of a music/video queue which supports multi-client voting and submission.
The top of the queue is then played by a video/audio player which syncs current playback state between all clients.

### Project specifics
In my effort to experiement with the Cloudflare Workers platform, I aimed to acheive a few items below:
- experiment with the Cloudflare Pages platform
- create a juniper wrapper, named `juniper_cf_workers`, around the `workers-rs` crate to support running GraphQL servers and merge it upstream with the juniper project
- implement a Durable Object which manages sessions via websockets (sadly `workers-rs` does not have an interface for this yet)
- implement a GraphQL API as a go between for the client application and durable object
- learn more Rust (definitely a newbie), evaluate Svelte (used to be a Vue3 project, hence the title, oops) as a front end framework, and learn GraphQL

### State of this project
Well. I bit off more than I could chew (seems to be my M.O.) for this Hacktober. I intend to submit it to Cloudflare Developer Challenge, but it is in a heavily experimental and broken state.
At this time, I do not know if the full functionality can be implemented until `worker-rs` exposes a websocket interface. This usually only gets partial attention on the weekends, due to fulltime work for @Zenus-Inc, so progress has been slow. 
