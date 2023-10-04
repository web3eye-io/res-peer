import { defineStore } from 'pinia'

export interface Review {
  reviewer: string
  approved: boolean
  reason: string
}

export interface Content {
  cid: string
  comment_to_cid?: string
  author: string
  title: string
  content: string
  reviewers: Map<string, Review>
  approved: number
  rejected: number
}

export const useReviewStore = defineStore('review', {
  state: () => ({
    contentApplicationsKeys: [] as Array<string>,
    contentApplications: new Map<string, Content>()
  }),
  getters: {},
  actions: {}
})
