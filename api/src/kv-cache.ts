export default class KVCache {
  get(key: string): any {
    return WORKERS_GRAPHQL_CACHE.get(key)
  }

  set(key: string, value: string, options: any): any {
    const opts = {}
    const ttl = options && options.ttl
    if (ttl) {
      (opts as any).expirationTtl = ttl
    }
    return WORKERS_GRAPHQL_CACHE.put(key, value, opts)
  }
}
