<template>
  <div>
    <!-- when the query response is not received yet, data from it is undefined,
    so before referring to it we need to use v-if -->
    <div v-if='result'>
      GraphQL subscription result: {{ result }}
    </div>
    <div>error... {{ error }}</div>
    <div>loading... {{ loading }}</div>
    <div>variables... {{ variables }}</div>
  </div>
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
  onResult,
  onError
} = useSubscription(gql`
  subscription {
    notifications
  }
`)

watch(result, () => {
  console.log('Subscrption result: ', result.value)
})

onResult((res) => {
  console.log('Subscription result: ', res)
})

onError((error) => {
  console.log('Subscription error: ', error)
})
</script>
