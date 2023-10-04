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

const getReviewState = () => {
  const { /* result, */ refetch /*, fetchMore */, onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getReviewState {
      contentApprovedThreshold
      contentRejectedThreshold
      assetApprovedThreshold
      assetRejectedThreshold
      reviewerApprovedThreshold
      reviewerRejectedThreshold
    }
  `, {
    endpoint: 'review',
    chainId: targetChain.value
  }))

  watch(blockHeight, () => {
    void refetch()
  })

  onResult((res) => {
    if (res.loading) return
    const ret = (res.data as Record<string, number>)
    review.contentApprovedThreshold = ret.contentApprovedThreshold
    review.contentRejectedThreshold = ret.contentRejectedThreshold
    review.assetApprovedThreshold = ret.assetApprovedThreshold
    review.assetRejectedThreshold = ret.assetRejectedThreshold
    review.reviewerApprovedThreshold = ret.reviewerApprovedThreshold
    review.reviewerRejectedThreshold = ret.reviewerRejectedThreshold
  })
}

watch(reviewer, () => {
  if (!reviewer.value) return
  getReviewState()
})

onMounted(() => {
  if (!reviewer.value) return
  getReviewState()
})

</script>
