export default {
  Query: {
    pokemon: async (_source: unknown, { id }: any, { dataSources }: any): Promise<unknown> => {
      return dataSources.pokemonAPI.getPokemon(id)
    },
  },
}