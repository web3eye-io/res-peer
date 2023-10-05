import { defineStore } from 'pinia'

export interface Content {
  cid: string
  author: string
  title: string
  content: string
  likes: number
  dislikes: number
  accounts: Record<string, boolean>
  createdAt: number
}

export const useContentStore = defineStore('content', {
  state: () => ({
    contentsKeys: [] as Array<string>,
    recommends: new Map<string, Array<string>>(),
    comments: new Map<string, Array<string>>(),
    contents: new Map<string, Content>(),
    mutateKeys: [] as Array<string>
  }),
  getters: {
    _contents (): () => Array<Content> {
      return () => {
        return Array.from(this.contents.values()).filter((el) => {
          return !this.recommends.get(el.cid) && !this.comments.get(el.cid)
        }).sort((a: Content, b: Content) => a.createdAt < b.createdAt ? 1 : -1)
      }
    }
  },
  actions: {}
})
