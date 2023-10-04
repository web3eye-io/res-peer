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

const getContentApplicationsKeys = () => {
  const { /* result, */ refetch /*, fetchMore */, onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContentApplicationsKeys {
      contentApplicationsKeys
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
    review.contentApplicationsKeys = (res.data as Record<string, Array<string>>).contentApplicationsKeys
  })
}

watch(reviewer, () => {
  if (!reviewer.value) return
  getContentApplicationsKeys()
})

onMounted(() => {
  if (!reviewer.value) return
  getContentApplicationsKeys()
})

</script>
