import { defineStore } from 'pinia'
import { Reviewer } from './review'

export interface AgeAmount {
  amount: string
  expired: number
}

export const useUserStore = defineStore('user', {
  state: () => ({
    account: undefined as unknown as string,
    spendable: '0.',
    amounts: [] as Array<AgeAmount>,
    reviewer: false,
    reviewerApplication: undefined as unknown as Reviewer
  }),
  getters: {},
  actions: {}
})
