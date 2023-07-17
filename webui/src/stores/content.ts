import { defineStore } from 'pinia'

export const useContentStore = defineStore('content', {
  state: () => ({
    contentsKeys: [] as Array<string>,
    contents: new Map<string, string>()
  }),
  getters: {},
  actions: {}
})
