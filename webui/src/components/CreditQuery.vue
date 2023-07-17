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
    <!-- button @click='fetchMore()'>Refresh</button -->
  </div>
</template>

<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
// import { ApolloQuery } from '@vue/apollo-components'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { computed, watch } from 'vue'

const {
  result,
  loading,
  error,
  variables,
  refetch
  // fetchMore, subscribeToMore
  // onResult,
  // onError
} = useQuery(gql`
  query credit {
    balancesKeys
    spendables(
      owner: "b975c98d6921a2beb1d974d83a29304eb5f5ad301a55e56e7984079607fcb633"
    )
    spendablesKeys
    balance
    balances(
      owner: "b975c98d6921a2beb1d974d83a29304eb5f5ad301a55e56e7984079607fcb633"
    ) {
      amounts {
        amount
        expired
      }
    }
  }
`)

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
watch(blockHeight, () => {
  void refetch()
})

</script>
