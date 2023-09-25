import { defineStore } from 'pinia'

export const useChainStore = defineStore('chain', {
  state: () => ({
    chains: [] as Array<string>,
    defaultChain: undefined as unknown as string
  }),
  getters: {},
  actions: {}
})
