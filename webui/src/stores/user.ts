import { defineStore } from 'pinia'

export interface AgeAmount {
  amount: string
  expired: number
}

export const useUserStore = defineStore('user', {
  state: () => ({
    account: undefined as unknown as string,
    spendable: '0.',
    balances: [] as Array<AgeAmount>
  }),
  getters: {},
  actions: {}
})
