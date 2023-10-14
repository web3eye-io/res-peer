import { defineStore } from 'pinia'

export interface NFT {
  token_id: number
  uri_index: number
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
  uri: string
}

export interface Collection {
  collectionId: number
  baseUri: string
  uris: Array<string>
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
    tradeFeePercent: 0,
    maxCreditsPercent: 0,
    assets: new Map<number, Array<number>>(),
    mutateKeys: [] as Array<number>,
    avatars: new Map<string, Array<number>>()
  }),
  getters: {
    _collections (): (publisher?: string) => Array<Collection> {
      return (author?: string) => {
        return Array.from(this.collections.values()).filter((el) => {
          let ok = true
          if (author) ok &&= el.publisher === author
          return ok
        }).sort((a: Collection, b: Collection) => a.createdAt < b.createdAt ? 1 : -1)
      }
    },
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
          Array.from(Object.values(el.nfts)).forEach((nft: NFT) => {
            nfts.push({
              collectionId: el.collectionId,
              collectionName: el.name,
              collectionPrice: el.price,
              baseUri: el.baseUri,
              uri: el.uris[nft.uri_index],
              ...nft
            } as NFTExt)
          })
        })
        return nfts.sort((a, b) => a.token_id > b.token_id ? 1 : -1)
      }
    },
    nftsByCollectionID (): (collectionId?: number) => Array<NFTExt> {
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
          Array.from(Object.values(el.nfts)).forEach((nft: NFT) => {
            nfts.push({
              collectionId: el.collectionId,
              collectionName: el.name,
              collectionPrice: el.price,
              baseUri: el.baseUri,
              uri: el.uris[nft.uri_index],
              ...nft
            } as NFTExt)
          })
        })
        return nfts.sort((a, b) => a.token_id > b.token_id ? 1 : -1)
      }
    },
    nftsByOwner (): (owner?: string) => Array<NFTExt> {
      return (owner?: string) => {
        return this.nftsByCollectionID().filter((el) => {
          const collection = this.collections.get(el.collectionId)
          if (!collection) {
            return false
          }
          const assets = this.assets.get(el.collectionId)
          if (owner && collection.publisher !== owner && assets && assets.findIndex((tokenId) => tokenId === el.token_id) < 0) {
            return false
          }
          return true
        }).sort((a, b) => a.token_id > b.token_id ? 1 : -1)
      }
    },
    nftBanner (): (nft: NFTExt) => string {
      return (nft: NFTExt) => {
        if (nft.baseUri?.endsWith('/')) {
          nft.baseUri = nft.baseUri.substring(0, nft.baseUri.length - 1)
        }
        return nft.baseUri + '/' + nft.uri
      }
    },
    collectionBanner (): (collection: Collection) => string {
      return (collection: Collection) => {
        let baseUri = collection.baseUri
        if (collection.baseUri?.endsWith('/')) {
          baseUri = collection.baseUri.substring(0, collection.baseUri.length - 1)
        }
        const nfts = this.nftsByCollectionID(collection.collectionId)
        return baseUri + '/' + nfts[0]?.uri
      }
    },
    nftBannerByID (): (collectionId: number, tokenId: number) => string | undefined {
      return (collectionId: number, tokenId: number) => {
        const collection = this.collections.get(collectionId)
        if (!collection) {
          return undefined
        }
        const nfts = this.nftsByCollectionID(collectionId)
        if (!nfts) {
          return undefined
        }
        let nft = nfts.find((el) => el.token_id === tokenId)
        if (!nft) {
          nft = nfts.find((el) => el.token_id === 1000)
        }
        if (!nft) {
          return undefined
        }
        return collection.baseUri + '/' + nft.uri
      }
    }
  },
  actions: {}
})
