import { defineStore } from 'pinia'

export interface Content {
  cid: string
  content: string
  title: string
  likes: number
  dislikes: number
  accounts: Map<string, boolean>
}

export const useContentStore = defineStore('content', {
  state: () => ({
    contentsKeys: [] as Array<string>,
    contents: new Map<string, Content>()
  }),
  getters: {},
  actions: {}
})
