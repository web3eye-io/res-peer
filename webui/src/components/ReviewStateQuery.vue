<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { useReviewStore } from 'src/stores/review'
import { computed, onMounted, watch } from 'vue'
import { targetChain } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const review = useReviewStore()
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = (): boolean => {
  return reviewApp.value?.length > 0 && targetChain.value?.length > 0
}

const getReviewState = () => {
  const { /* result, */ refetch /*, fetchMore */, onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getReviewState {
      contentApprovedThreshold
      contentRejectedThreshold
      assetApprovedThreshold
      assetRejectedThreshold
      reviewerApprovedThreshold
      reviewerRejectedThreshold
      activityApprovedThreshold
      activityRejectedThreshold
      reviewerNumber
    }
  `, {
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
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
    review.activityApprovedThreshold = ret.activityApprovedThreshold
    review.activityRejectedThreshold = ret.activityRejectedThreshold
    review.reviewerNumber = ret.reviewerNumber
  })
}

watch(targetChain, () => {
  if (!ready()) return
  getReviewState()
})

watch(reviewApp, () => {
  if (!ready()) return
  getReviewState()
})

onMounted(() => {
  if (!ready()) return
  getReviewState()
})

</script>
