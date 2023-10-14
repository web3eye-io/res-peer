<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'

const user = useUserStore()
const account = computed(() => user.account)
const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = (): boolean => {
  return account.value?.length > 0 && marketApp.value?.length > 0 && targetChain.value?.length > 0
}

watch(targetChain, () => {
  if (!ready()) return
  getMarketInfo()
})

watch(marketApp, () => {
  if (!ready()) return
  getMarketInfo()
})

watch(account, () => {
  if (!ready()) return
  getMarketInfo()
})

watch(blockHeight, () => {
  if (!ready()) return
  getMarketInfo()
})

const collection = useCollectionStore()

const onResult = (res: Record<string, unknown>) => {
  collection.collectionsKeys = (res as Record<string, Array<number>>).collectionsKeys
  collection.creditsPerLinera = (res as Record<string, string>).creditsPerLinera
  collection.maxCreditsPercent = (res as Record<string, number>).maxCreditsPercent
  collection.tradeFeePercent = (res as Record<string, number>).tradeFeePercent
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
          tradeFeePercent
          maxCreditsPercent
          assets(owner: $account)
          avatars(owner: $account)
        }
      `, {
        account: `${account.value}`,
        endpoint: 'market',
        chainId: targetChain.value
      }, {
        fetchPolicy: 'network-only'
      })
    }
    return useQuery(gql`
        query getMarketInfo {
          collectionsKeys
          creditsPerLinera
          tradeFeePercent
          maxCreditsPercent
        }
      `, {
      account: `${account.value}`,
      endpoint: 'market',
      chainId: targetChain.value
    }, {
      fetchPolicy: 'network-only'
    })
  })

  watch(result, () => {
    const res = result.value as Record<string, unknown>
    onResult(res)
  })
}

onMounted(() => {
  if (!ready()) return
  getMarketInfo()
})

</script>
