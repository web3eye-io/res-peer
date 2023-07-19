import { defineStore } from 'pinia'

export interface NFT {
  token_id: number
  uri: string
  price?: string
  on_sale: boolean
  minted_at: number
  name: string
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
    creditsPerLinera: '0.',
    mutateKeys: [] as Array<number>
  }),
  getters: {
    nftsByPublisher (): (publisher?: string) => Array<NFTExt> {
      return (publisher?: string) => {
        const nfts = [] as Array<NFTExt>
        let collections = [] as Array<Collection>
        if (publisher) {
          collections = Array.from(this.collections.values()).filter((el) => el.publisher === publisher)
        } else {
          collections = Array.from(this.collections.values())
        }
        collections.forEach((el) => {
          if (!el.nfts || !Object.keys(el.nfts).length) {
            return
          }
          Array.from(Object.values(el.nfts)).forEach((nft) => {
            nfts.push({
              collectionId: el.collectionId,
              collectionName: el.name,
              collectionPrice: el.price,
              ...nft
            } as NFTExt)
          })
        })
        return nfts.sort((a, b) => a.token_id > b.token_id ? 1 : -1)
      }
    },
    nftsByCollections (): (collectionId?: number) => Array<NFTExt> {
      return (collectionId?: number) => {
        const nfts = [] as Array<NFTExt>
        let collections = [] as Array<Collection>
        if (collectionId) {
          collections = [this.collections.get(collectionId) as Collection]
        } else {
          collections = Array.from(this.collections.values())
        }
        if (!collections) {
          return []
        }
        collections.forEach((el) => {
          if (!el || !el.nfts || !Object.keys(el.nfts).length) {
            return
          }
          Array.from(Object.values(el.nfts)).forEach((nft) => {
            nfts.push({
              collectionId: el.collectionId,
              collectionName: el.name,
              collectionPrice: el.price,
              ...nft
            } as NFTExt)
          })
        })
        return nfts.sort((a, b) => a.token_id > b.token_id ? 1 : -1)
      }
    },
    nftBanner (): (nft: NFTExt) => string {
      return (nft: NFTExt) => {
        if (nft.baseUri?.endsWith('/')) {
          nft.baseUri = nft.baseUri.substring(0, nft.baseUri.length - 1)
        }
        return nft.baseUri + '/' + nft.uri
      }
    }
  },
  actions: {}
})
