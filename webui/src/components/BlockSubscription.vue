<script setup lang="ts">
import { provideApolloClient, useSubscription } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { onMounted, watch } from 'vue'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const block = useBlockStore()

const subscribe = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useSubscription(gql`
    subscription notifications($chainId: String!) {
      notifications(chainId: $chainId)
    }
  `, {
    chainId: targetChain.value
  }))

  onResult((res) => {
    const data = res.data as Record<string, Record<string, Record<string, Record<string, unknown>>>>
    if (data.notifications.reason.NewBlock) {
      block.blockHeight = data.notifications.reason.NewBlock.height as number
      block.blockHash = data.notifications.reason.NewBlock.hash as string
    }
  })
}

watch(targetChain, () => {
  if (!targetChain.value) return
  subscribe()
})

onMounted(() => {
  if (!targetChain.value) return
  subscribe()
})

</script>
