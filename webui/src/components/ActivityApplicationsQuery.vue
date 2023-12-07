<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useReviewStore, Activity } from 'src/stores/review'
import { computed, watch, ref, onMounted } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'

const review = useReviewStore()
const activityApplicationsKeys = computed(() => review.activityApplicationsKeys)
const activityApplications = computed(() => review.activityApplications)
const activityMutateKeys = computed(() => review.activityMutateKeys)
const activityIndex = ref(-1)
const activityApplicationKey = computed(() => activityIndex.value >= 0 ? activityApplicationsKeys.value[activityIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getActivityApplication = (activityApplicationKey: number, done?: () => void) => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getActivityApplication($activityApplicationKey: Int!) {
      activityApplications(u64: $activityApplicationKey) {
        activityId
        budgetAmount
        reviewers
        approved
        rejected
        createdAt
      }
    }
  `, {
    activityApplicationKey,
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  watch(result, () => {
    activityApplications.value.set(activityApplicationKey, (result.value as Record<string, Activity>).activityApplications)
    done?.()
  })
}

watch(activityApplicationKey, () => {
  if (!activityApplicationKey.value) {
    return
  }

  const index = activityMutateKeys.value.findIndex((el) => el === activityApplicationKey.value)
  if (activityApplications.value.get(activityApplicationKey.value) && index < 0) {
    activityIndex.value++
    return
  }

  getActivityApplication(activityApplicationKey.value, () => {
    activityIndex.value++
    activityMutateKeys.value.splice(index, 1)
  })
})

watch(activityApplicationsKeys, () => {
  if (activityApplicationsKeys.value.length === 0) {
    return
  }
  activityIndex.value = 0
})

watch(blockHeight, () => {
  if (activityApplicationsKeys.value.length === 0) {
    return
  }
  activityIndex.value = 0
})

onMounted(() => {
  activityMutateKeys.value.forEach((activityKey) => {
    getActivityApplication(activityKey)
  })
  review.activityMutateKeys = []
})

</script>
