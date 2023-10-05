import { defineStore } from 'pinia'

export interface Review {
  reviewer: string
  approved: boolean
  reason: string
}

export interface Content {
  cid: string
  commentToCid?: string
  author: string
  title: string
  content: string
  reviewers: Record<string, Review>
  approved: number
  rejected: number
  createdAt: number
}

export const useReviewStore = defineStore('review', {
  state: () => ({
    contentApplicationsKeys: [] as Array<string>,
    contentApplications: new Map<string, Content>(),
    contentApprovedThreshold: 0,
    contentRejectedThreshold: 0,
    assetApprovedThreshold: 0,
    assetRejectedThreshold: 0,
    reviewerApprovedThreshold: 0,
    reviewerRejectedThreshold: 0
  }),
  getters: {
    reviewed (): (cid: string, owner: string) => boolean {
      return (cid: string, owner: string) => {
        return Object.keys(this.contentApplications.get(cid)?.reviewers || {})?.find((el) => el === owner) !== undefined
      }
    },
    content (): (cid: string) => Content | undefined {
      return (cid: string) => {
        return this.contentApplications.get(cid)
      }
    }
  },
  actions: {}
})
