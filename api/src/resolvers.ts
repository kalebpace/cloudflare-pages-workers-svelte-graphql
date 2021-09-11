export default {
  Query: {
    pokemon: async (_source: any, { id }: any, { dataSources }: any): Promise<any> => {
      return dataSources.pokemonAPI.getPokemon(id)
    },
  },
}
