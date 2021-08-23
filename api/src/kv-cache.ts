// eslint-disable-next-line @typescript-eslint/ban-ts-comment
//@ts-ignore
const cache: any = WORKERS_GRAPHQL_CACHE

export default class KVCache {
  get(key: string): unknown {
    return cache.get(key)
  }

  set(key: string, value: any, options: any): unknown {
    const opts = {} as any
    const ttl = options && options.ttl
    if (ttl) {
      opts.expirationTtl = ttl
    }
    return cache.put(key, value, opts)
  }
}