<template>
  <div>
    <!-- when the query response is not received yet, data from it is undefined,
    so before referring to it we need to use v-if -->
    <div v-if='result'>
      GraphQL query result: {{ result }}
    </div>
    <div>error... {{ error }}</div>
    <div>loading... {{ loading }}</div>
    <div>variables... {{ variables }}</div>
    <button @click='refetch()'>
      Refresh
    </button>
  </div>
</template>

<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useContentStore } from 'src/stores/content'
import { computed, watch } from 'vue'

const {
  result,
  loading,
  error,
  variables,
  refetch,
  // fetchMore, subscribeToMore
  onResult
  // onError
} = useQuery(gql`
  query getContentsKeys {
    contentsKeys
  }
`, {
  endpoint: 'feed'
})

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
watch(blockHeight, () => {
  void refetch()
})

const content = useContentStore()
onResult((res) => {
  if (res.loading) {
    return
  }
  content.contentsKeys = (res.data as Record<string, Array<string>>).contentsKeys
})

</script>
