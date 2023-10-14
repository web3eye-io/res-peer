<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useReviewStore, Reviewer } from 'src/stores/review'
import { computed, watch, ref, onMounted } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'

const review = useReviewStore()
const reviewerApplicationsKeys = computed(() => review.reviewerApplicationsKeys)
const reviewerApplications = computed(() => review.reviewerApplications)
const reviewerMutateKeys = computed(() => review.reviewerMutateKeys)
const reviewerIndex = ref(-1)
const reviewerApplicationKey = computed(() => reviewerIndex.value >= 0 ? reviewerApplicationsKeys.value[reviewerIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getReviewerApplication = (reviewerApplicationKey: string, done?: () => void) => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getReviewerApplication($reviewerApplicationKey: String!) {
      reviewerApplications(owner: $reviewerApplicationKey) {
        chainId
        reviewer
        resume
        reviewers
        approved
        rejected
        createdAt
      }
    }
  `, {
    reviewerApplicationKey: `${reviewerApplicationKey}`,
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  watch(result, () => {
    reviewerApplications.value.set(reviewerApplicationKey, (result.value as Record<string, Reviewer>).reviewerApplications)
    done?.()
  })
}

watch(reviewerApplicationKey, () => {
  if (!reviewerApplicationKey.value) {
    return
  }

  const index = reviewerMutateKeys.value.findIndex((el) => el === reviewerApplicationKey.value)
  if (reviewerApplications.value.get(reviewerApplicationKey.value) && index < 0) {
    reviewerIndex.value++
    return
  }

  getReviewerApplication(reviewerApplicationKey.value, () => {
    reviewerIndex.value++
    reviewerMutateKeys.value.splice(index, 1)
  })
})

watch(reviewerApplicationsKeys, () => {
  if (reviewerApplicationsKeys.value.length === 0) {
    return
  }
  reviewerIndex.value = 0
})

watch(blockHeight, () => {
  if (reviewerApplicationsKeys.value.length === 0) {
    return
  }
  reviewerIndex.value = 0
})

onMounted(() => {
  reviewerMutateKeys.value.forEach((reviewerKey) => {
    getReviewerApplication(reviewerKey)
  })
  review.reviewerMutateKeys = []
})

</script>
