<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { useReviewStore } from 'src/stores/review'
import { computed, onMounted, watch } from 'vue'
import { targetChain } from 'src/stores/chain'
import { useUserStore } from 'src/stores/user'

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const review = useReviewStore()
const user = useUserStore()
const reviewer = computed(() => user.reviewer)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getAssetApplicationsKeys = () => {
  const { /* result, */ refetch /*, fetchMore */, onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getAssetApplicationsKeys {
      assetApplicationsKeys
    }
  `, {
    endpoint: 'review',
    chainId: targetChain.value
  }))

  // TODO: is it still work
  watch(blockHeight, () => {
    void refetch()
  })

  onResult((res) => {
    if (res.loading) return
    review.assetApplicationsKeys = (res.data as Record<string, Array<string>>).assetApplicationsKeys
  })
}

watch(reviewer, () => {
  if (!reviewer.value) return
  getAssetApplicationsKeys()
})

onMounted(() => {
  if (!reviewer.value) return
  getAssetApplicationsKeys()
})

</script>
