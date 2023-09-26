import { defineStore } from 'pinia'

export const useApplicationStore = defineStore('application', {
  state: () => ({
    feedApp: undefined as unknown as string,
    creditApp: undefined as unknown as string,
    marketApp: undefined as unknown as string
  }),
  getters: {},
  actions: {}
})
