import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  state: () => ({
    account: undefined as unknown as string
  }),
  getters: {},
  actions: {}
})
