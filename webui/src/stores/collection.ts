import { defineStore } from 'pinia'

export interface NFT {
  tokenId: number
  uri: string
  price?: string
  onSale: boolean
}

export interface Collection {
  collectionId: number
  baseUri: string
  nfts: Map<number, NFT>
  price?: string
  name: string
}

export const useCollectionStore = defineStore('collection', {
  state: () => ({
    collectionsKeys: [] as Array<number>,
    collections: new Map<number, Collection>()
  }),
  getters: {},
  actions: {}
})
