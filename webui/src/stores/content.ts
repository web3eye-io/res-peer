import { defineStore } from 'pinia'

export interface Content {
  cid: string
  commentToCid?: string
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
    content (): (cid: string) => Content | undefined {
      return (cid: string) => {
        return this.contents.get(cid)
      }
    },
    _contents (): () => Array<Content> {
      return () => {
        return Array.from(this.contents.values()).filter((el) => {
          return !el.commentToCid
        }).sort((a: Content, b: Content) => a.createdAt < b.createdAt ? 1 : -1)
      }
    },
    _recommends (): (cid: string) => Array<Content> {
      return (cid: string) => {
        return Array.from(this.contents.values()).filter((el) => this.recommends.get(cid)?.findIndex((el1) => el1 === el.cid)) || []
      }
    },
    _comments (): (cid: string) => Array<Content> {
      return (cid: string) => {
        return Array.from(this.contents.values()).filter((el) => this.comments.get(cid)?.findIndex((el1) => el1 === el.cid)) || []
      }
    }
  },
  actions: {}
})
