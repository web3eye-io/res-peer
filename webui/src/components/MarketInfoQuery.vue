<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'

const user = useUserStore()
const account = computed(() => user.account)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
watch(blockHeight, () => {
  getMarketInfo()
})

watch(account, () => {
  getMarketInfo()
})

const collection = useCollectionStore()

const onResult = (res: Record<string, unknown>) => {
  // TODO: a bug here will cause balances to be another value from credits, don't know why
  collection.collectionsKeys = (res as Record<string, Array<number>>).collectionsKeys
  collection.creditsPerLinera = (res as Record<string, string>).creditsPerLinera
  const balance = (res as Record<string, string>).balances
  if (balance) {
    if (typeof (balance) === 'string') {
      collection.lineraBalance = (res as Record<string, string>).balances
    }
  }
  const assets = (res as Record<string, Record<number, Array<number>>>).assets
  if (assets) {
    Object.keys(assets).forEach((key, index) => {
      collection.assets.set(parseInt(key), Object.values(assets)[index])
    })
  }
}

const getMarketInfo = () => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => {
    if (account.value) {
      return useQuery(gql`
        query getMarketInfo($account: String!) {
          collectionsKeys
          creditsPerLinera
          balances(owner: $account)
          assets(owner: $account)
          avatars(owner: $account)
        }
      `, {
        account: `${account.value}`,
        endpoint: 'market'
      })
    }
    return useQuery(gql`
        query getMarketInfo {
          collectionsKeys
          creditsPerLinera
        }
      `, {
      account: `${account.value}`,
      endpoint: 'market'
    })
  })

  watch(result, () => {
    const res = result.value as Record<string, unknown>
    onResult(res)
  })
}

onMounted(() => {
  getMarketInfo()
})

</script>
