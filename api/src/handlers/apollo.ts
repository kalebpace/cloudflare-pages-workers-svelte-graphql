import { ApolloServer, Config } from 'apollo-server-cloudflare'
import { graphqlCloudflare } from 'apollo-server-cloudflare/dist/cloudflareApollo'
import { Request } from 'apollo-server-core/node_modules/apollo-server-env/dist/fetch'
import { CloudflareGraphQLOptions } from '../types'

import KVCache from '../kv-cache'
import PokemonAPI  from '../datasources/pokeapi'
import resolvers from '../resolvers'
import typeDefs from '../schema'


const dataSources = () => ({
  pokemonAPI: new PokemonAPI(),
})

const kvCache = { cache: new KVCache() }

const createServer = (graphQLOptions: CloudflareGraphQLOptions) =>
  new ApolloServer({
    typeDefs,
    resolvers,
    introspection: true,
    dataSources,
    ...(graphQLOptions.kvCache ? kvCache : {}),
  } as Config)

const handler = (request: Request, graphQLOptions: CloudflareGraphQLOptions) => {
  const server = createServer(graphQLOptions)
  return graphqlCloudflare(() => server.createGraphQLServerOptions(request))(request)
}

export default handler
