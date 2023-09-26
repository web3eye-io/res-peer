import { defineStore } from 'pinia'
import { computed } from 'vue'

export const useChainStore = defineStore('chain', {
  state: () => ({
    chains: [] as Array<string>,
    defaultChain: undefined as unknown as string,
    targetChain: undefined as unknown as string
  }),
  getters: {},
  actions: {}
})

const chain = useChainStore()
export const targetChain = computed(() => chain.targetChain)
