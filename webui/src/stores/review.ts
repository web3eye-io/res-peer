import { defineStore } from 'pinia'

export interface Review {
  reviewer: string
  approved: boolean
  reason: string
}

export interface ReviewApplication {
  reviewers: Record<string, Review>
  approved: number
  rejected: number
  createdAt: number
}

export interface Content extends ReviewApplication {
  cid: string
  commentToCid?: string
  author: string
  title: string
  content: string
}

export interface Asset extends ReviewApplication {
  cid: string
  baseUri: string
  uris: Array<string>
  author: string
  price?: number
  name: string
}

export const useReviewStore = defineStore('review', {
  state: () => ({
    contentApplicationsKeys: [] as Array<string>,
    contentApplications: new Map<string, Content>(),
    assetApplicationsKeys: [] as Array<string>,
    assetApplications: new Map<string, Content>(),
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
    },
    contentReview (): (cid: string, owner: string) => Review | undefined {
      return (cid: string, owner: string) => {
        const content = this.contentApplications.get(cid)
        return content?.reviewers[owner]
      }
    },
    asset (): (cid: string) => Content | undefined {
      return (cid: string) => {
        return this.assetApplications.get(cid)
      }
    },
    assetReview (): (cid: string, owner: string) => Review | undefined {
      return (cid: string, owner: string) => {
        const asset = this.assetApplications.get(cid)
        return asset?.reviewers[owner]
      }
    }
  },
  actions: {}
})
