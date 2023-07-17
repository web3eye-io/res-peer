import { defineStore } from 'pinia'

export const useBlockStore = defineStore('block', {
  state: () => ({
    blockHeight: 0,
    blockHash: ''
  }),
  getters: {},
  actions: {}
})
