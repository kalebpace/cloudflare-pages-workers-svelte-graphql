import { GraphQLOptions } from "apollo-server-cloudflare";

interface CloudflareGraphQLOptions extends GraphQLOptions {
    baseEndpoint: string
    kvCache: any
}