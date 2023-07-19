import { defineStore } from 'pinia'

export interface NFT {
  tokenId: number
  uri: string
  price?: string
  onSale: boolean
  mintedAt: number
}

export interface NFTExt extends NFT {
  collectionId: number
  collectionName: string
  collectionPrice?: string
  baseUri: string
}

export interface Collection {
  collectionId: number
  baseUri: string
  nfts: Map<number, NFT>
  price?: string
  name: string
  createdAt: number
  publisher: string
}

export const useCollectionStore = defineStore('collection', {
  state: () => ({
    collectionsKeys: [] as Array<number>,
    collections: new Map<number, Collection>(),
    mutateKeys: [] as Array<number>
  }),
  getters: {
    nfts (): (publisher?: string) => Array<NFTExt> {
      return (publisher?: string) => {
        const nfts = [] as Array<NFTExt>
        let collections = [] as Array<Collection>
        if (publisher) {
          collections = Array.from(this.collections.values()).filter((el) => el.publisher === publisher)
        } else {
          collections = Array.from(this.collections.values())
        }
        collections.forEach((el) => {
          if (!el.nfts || !el.nfts?.size) {
            return
          }
          Array.from(el.nfts.values()).forEach((nft) => {
            nfts.push({
              collectionId: el.collectionId,
              collectionName: el.name,
              collectionPrice: el.price,
              ...nft
            } as NFTExt)
          })
        })
        return nfts
      }
    }
  },
  actions: {}
})
