import { defineStore } from 'pinia'

export const useFoundationStore = defineStore('foundation', {
  state: () => ({
    foundationBalance: '0.',
    reviewRewardPercent: 0,
    reviewRewardBalance: '0.',
    reviewRewardFactor: 0,
    authorRewardPercent: 0,
    authorRewardBalance: '0.',
    authorRewardFactor: 0,
    activityRewardPercent: 0,
    activityRewardBalance: '0.',
    activityOwners: new Map<number, string>(),
    activityLockFunds: new Map<number, string>(),
    userLineraBalance: '0.'
  }),
  getters: {},
  actions: {}
})
