<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useCollectionStore, Collection } from 'src/stores/collection'
import { computed, watch, ref } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'

const collection = useCollectionStore()
const collectionsKeys = computed(() => collection.collectionsKeys)
const collections = computed(() => collection.collections)
const collectionIndex = ref(-1)
const collectionKey = computed(() => collectionIndex.value >= 0 ? collectionsKeys.value[collectionIndex.value] : undefined)
const mutateKeys = computed(() => collection.mutateKeys)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const forceFetch = ref(false)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getCollection = (collectionKey: number, done?: () => void) => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getCollection($collectionKey: Int!) {
      collections(u64: $collectionKey) {
        price
        baseUri
        uris
        nfts
        collectionId
        name
        publisher
        createdAt
      }
    }
  `, {
    collectionKey: parseInt(collectionKey.toString()),
    endpoint: 'market',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  watch(result, () => {
    collections.value.set(collectionKey, (result.value as Record<string, Collection>).collections)
    done?.()
  })
}

watch(collectionKey, () => {
  if (!collectionKey.value) {
    return
  }
  const index = collection.mutateKeys.findIndex((el) => el === collectionKey.value)
  if (collections.value.get(collectionKey.value) && index < 0 && !forceFetch.value) {
    collectionIndex.value++
    return
  }
  getCollection(collectionKey.value, () => {
    collectionIndex.value++
  })
})

watch(targetChain, () => {
  if (collectionsKeys.value.length === 0) {
    return
  }
  forceFetch.value = false
  collectionIndex.value = 0
})

watch(collectionsKeys, () => {
  if (collectionsKeys.value.length === 0) {
    return
  }
  forceFetch.value = false
  collectionIndex.value = 0
})

watch(blockHeight, () => {
  if (collectionsKeys.value.length === 0) {
    return
  }
  forceFetch.value = true
  collectionIndex.value = 0
})

watch(mutateKeys, () => {
  if (collectionsKeys.value.length === 0) {
    return
  }
  forceFetch.value = false
  collectionIndex.value = 0
})

</script>
