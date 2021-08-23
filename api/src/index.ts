// import { handleRequest, handleOptions } from './handlers/handler'

// addEventListener('fetch', (event) => {
//   const request = event.request
//   if (request.method === "OPTIONS") {
//     event.respondWith(handleOptions(request))
//   }
//   else if(
//     request.method === "GET" ||
//     request.method === "HEAD" ||
//     request.method === "POST"
//   ){
//     event.respondWith(handleRequest(request))
//   }
//   else {
//     event.respondWith(
//       new Response(null, {
//         status: 405,
//         statusText: "Method Not Allowed",
//       }),
//     )
//   }
// })

import apollo from './handlers/apollo'
import playground from './handlers/playground'
import setCors from './utils/setCors'

const graphQLOptions: any = {
  baseEndpoint: '/',
  playgroundEndpoint: '/___graphql',
  forwardUnmatchedRequestsToOrigin: false,
  debug: true,
  cors: true,
  kvCache: false,
}

const handleRequest = async (request: Request) => {
  const url = new URL(request.url)
  try {
    if (url.pathname === graphQLOptions.baseEndpoint) {
      const response: any =
        request.method === 'OPTIONS'
          ? new Response('', { status: 204 })
          : await apollo(request, graphQLOptions)
      if (graphQLOptions.cors) {
        setCors(response, graphQLOptions.cors)
      }
      return response
    } else if (
      graphQLOptions.playgroundEndpoint &&
      url.pathname === graphQLOptions.playgroundEndpoint
    ) {
      return playground(request, graphQLOptions)
    } else if (graphQLOptions.forwardUnmatchedRequestsToOrigin) {
      return fetch(request)
    } else {
      return new Response('Not found', { status: 404 })
    }
  } catch (err) {
    return new Response(graphQLOptions.debug ? err : 'Something went wrong', { status: 500 })
  }
}

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})