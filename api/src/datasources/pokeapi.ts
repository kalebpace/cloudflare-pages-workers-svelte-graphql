import { HTTPDataSource } from 'apollo-datasource-http'

export default class PokemonAPI extends HTTPDataSource {
  constructor() {
    super('https://pokeapi.co/api/v2/')
  }

  async getPokemon(id: string): Promise<unknown> {
    return this.get(`pokemon/${id}`)
  }
}