<template>
  <q-page>
    <!-- when the query response is not received yet, data from it is undefined,
    so before referring to it we need to use v-if -->
    <div v-if='result'>
      GraphQL subscription result: {{ result }}
    </div>
    <div>error... {{ error }}</div>
    <div>loading... {{ loading }}</div>
    <div>variables... {{ variables }}</div>
  </q-page>
</template>

<script setup lang="ts">
import { useSubscription } from '@vue/apollo-composable'
// import { ApolloQuery } from '@vue/apollo-components'
import gql from 'graphql-tag'
import { watch } from 'vue'

const {
  result,
  loading,
  error,
  variables,
  /* fetchMore, subscribeToMore, */ onResult,
  onError
} = useSubscription(gql`
  subscription {
    notifications
  }
`)

watch(result, () => {
  console.log(1, result.value)
})

onResult((res) => {
  console.log(2, res)
})

onError((error) => {
  console.log(3, error)
})
</script>
