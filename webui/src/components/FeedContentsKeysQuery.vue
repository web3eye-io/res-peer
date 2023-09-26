<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { useContentStore } from 'src/stores/content'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const content = useContentStore()
const application = useApplicationStore()
const feedApp = computed(() => application.feedApp)
const fetchTicker = ref(-1)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getContentsKeys = () => {
  const { /* result, */ refetch /*, fetchMore */, onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContentsKeys {
      contentsKeys
    }
  `, {
    endpoint: 'feed',
    chainId: targetChain.value
  }))

  // TODO: is it still work
  watch(blockHeight, () => {
    void refetch()
  })

  if (fetchTicker.value >= 0) {
    window.clearInterval(fetchTicker.value)
  }
  fetchTicker.value = window.setInterval(() => {
    void refetch()
  }, 3000)

  onResult((res) => {
    if (res.loading) {
      return
    }
    content.contentsKeys = (res.data as Record<string, Array<string>>).contentsKeys
  })
}

watch(feedApp, () => {
  if (feedApp.value) {
    getContentsKeys()
  }
})

onMounted(() => {
  if (feedApp.value) {
    getContentsKeys()
  }
})

onUnmounted(() => {
  if (fetchTicker.value >= 0) {
    window.clearInterval(fetchTicker.value)
  }
})

</script>
