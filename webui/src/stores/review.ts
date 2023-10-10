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

export interface Reviewer extends ReviewApplication {
  chainId: string
  reviewer: string
  resume: string
}

export const useReviewStore = defineStore('review', {
  state: () => ({
    contentApplicationsKeys: [] as Array<string>,
    contentApplications: new Map<string, Content>(),
    assetApplicationsKeys: [] as Array<string>,
    assetApplications: new Map<string, Asset>(),
    reviewerApplicationsKeys: [] as Array<string>,
    reviewerApplications: new Map<string, Reviewer>(),
    contentMutateKeys: [] as Array<string>,
    assetMutateKeys: [] as Array<string>,
    reviewerMutateKeys: [] as Array<string>,
    contentApprovedThreshold: 0,
    contentRejectedThreshold: 0,
    assetApprovedThreshold: 0,
    assetRejectedThreshold: 0,
    reviewerApprovedThreshold: 0,
    reviewerRejectedThreshold: 0
  }),
  getters: {
    contentReviewed (): (cid: string, owner: string) => boolean {
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
    assetReviewed (): (cid: string, owner: string) => boolean {
      return (cid: string, owner: string) => {
        return Object.keys(this.assetApplications.get(cid)?.reviewers || {})?.find((el) => el === owner) !== undefined
      }
    },
    asset (): (cid: string) => Asset | undefined {
      return (cid: string) => {
        return this.assetApplications.get(cid)
      }
    },
    assetReview (): (cid: string, owner: string) => Review | undefined {
      return (cid: string, owner: string) => {
        const asset = this.assetApplications.get(cid)
        return asset?.reviewers[owner]
      }
    },
    reviewerReviewed (): (candidate: string, owner: string) => boolean {
      return (cid: string, owner: string) => {
        return Object.keys(this.reviewerApplications.get(cid)?.reviewers || {})?.find((el) => el === owner) !== undefined
      }
    },
    reviewer (): (candidate: string) => Reviewer | undefined {
      return (cid: string) => {
        return this.reviewerApplications.get(cid)
      }
    },
    reviewerReview (): (candidate: string, owner: string) => Review | undefined {
      return (cid: string, owner: string) => {
        const asset = this.reviewerApplications.get(cid)
        return asset?.reviewers[owner]
      }
    }
  },
  actions: {}
})
