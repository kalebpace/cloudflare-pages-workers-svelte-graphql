import apollo from './handlers/apollo'
import playground from './handlers/playground'
import setCors from './utils/setCors'

import { Request } from 'apollo-server-env'

const serverOptions = {
  // Set the path for the GraphQL server
  baseEndpoint: '/',

  // Set the path for the GraphQL playground
  // This option can be removed to disable the playground route
  playgroundEndpoint: '/___graphql',

  // When a request's path isn't matched, forward it to the origin
  forwardUnmatchedRequestsToOrigin: false,

  // Enable debug mode to return script errors directly in browser
  debug: true,

  // Enable CORS headers on GraphQL requests
  // Set to `true` for defaults (see `utils/setCors`),
  // or pass an object to configure each header
  cors: false,
  // cors: {
  //   allowCredentials: 'true',
  //   allowHeaders: 'Content-type',
  //   allowOrigin: '*',
  //   allowMethods: 'GET, POST, PUT',
  // },

  // Enable KV caching for external REST data source requests
  // Note that you'll need to add a KV namespace called
  // WORKERS_GRAPHQL_CACHE in your wrangler.toml file for this to
  // work! See the project README for more information.
  kvCache: false,
}

const handleRequest = async (request: Request): Promise<any> => {
  const url = new URL(request.url)
  try {
    if (url.pathname === serverOptions.baseEndpoint) {
      const response = 
        request.method === 'OPTIONS'
          ? new Response(null, { status: 204 })
          : await apollo(request, serverOptions as any)
      if (serverOptions.cors) {
        setCors(response as Response, serverOptions.cors)
      }
      return response
    } 
    else if (
      serverOptions.playgroundEndpoint &&
      url.pathname === serverOptions.playgroundEndpoint
    ) {
      return playground(request as globalThis.Request, serverOptions as any)
    } else if (serverOptions.forwardUnmatchedRequestsToOrigin) {
      return fetch(request as globalThis.Request)
    } else {
      return new Response('Not found', { status: 404 })
    }
  } catch (err) {
    return new Response(serverOptions.debug ? (err as Error).stack as any : 'Something went wrong', { status: 500 })
  }
}

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request as Request))
})
