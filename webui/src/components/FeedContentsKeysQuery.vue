<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { useContentStore } from 'src/stores/content'
import { computed, onMounted, watch } from 'vue'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const content = useContentStore()
const application = useApplicationStore()
const feedApp = computed(() => application.feedApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return feedApp.value?.length && targetChain.value?.length
}

const getContentsKeys = () => {
  const { /* result, */ refetch /*, fetchMore */, onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContentsKeys {
      contentsKeys
    }
  `, {
    endpoint: 'feed',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  // TODO: is it still work
  watch(blockHeight, () => {
    void refetch()
  })

  onResult((res) => {
    if (res.loading) {
      return
    }
    content.contentsKeys = (res.data as Record<string, Array<string>>).contentsKeys
  })
}

watch(feedApp, () => {
  if (!ready()) return
  getContentsKeys()
})

watch(targetChain, () => {
  if (!ready()) return
  getContentsKeys()
})

onMounted(() => {
  if (!ready()) return
  getContentsKeys()
})

</script>
