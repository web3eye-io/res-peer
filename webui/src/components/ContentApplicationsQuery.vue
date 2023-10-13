<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useReviewStore, Content } from 'src/stores/review'
import { computed, watch, ref } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'

const review = useReviewStore()
const contentApplicationsKeys = computed(() => review.contentApplicationsKeys)
const contentApplications = computed(() => review.contentApplications)
const contentIndex = ref(-1)
const contentApplicationKey = computed(() => contentIndex.value >= 0 ? contentApplicationsKeys.value[contentIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getContentApplication = (contentApplicationKey: string, done?: () => void) => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContentApplication($contentApplicationKey: String!) {
      contentApplications(string: $contentApplicationKey) {
        cid
        commentToCid
        author
        title
        content
        reviewers
        approved
        rejected
        createdAt
      }
    }
  `, {
    contentApplicationKey: `${contentApplicationKey}`,
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  watch(result, () => {
    contentApplications.value.set(contentApplicationKey, (result.value as Record<string, Content>).contentApplications)
    done?.()
  })
}

watch(contentApplicationKey, () => {
  if (!contentApplicationKey.value) {
    return
  }
  getContentApplication(contentApplicationKey.value, () => {
    contentIndex.value++
  })
})

watch(contentApplicationsKeys, () => {
  if (contentApplicationsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

watch(blockHeight, () => {
  if (contentApplicationsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

</script>
