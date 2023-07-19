import { defineStore } from 'pinia'

export interface NFT {
  tokenId: string
  uri: string
  price?: string
  onSale: boolean
}

export interface Collection {
  collectionId: string
  baseUri: string
  nfts: Map<string, NFT>
  price?: string
  name: string
}

export const useCollectionStore = defineStore('collection', {
  state: () => ({
    collectionsKeys: [] as Array<string>,
    collections: new Map<string, Collection>()
  }),
  getters: {},
  actions: {}
})
