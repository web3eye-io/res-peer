<template>
  <q-page class='row items-center justify-evenly'>
    <apollo-query />
    <apollo-subscription />
  </q-page>
</template>

<script setup lang="ts">
import ApolloQuery from '../components/ApolloQuery.vue'
import ApolloSubscription from '../components/ApolloSubscription.vue'
</script>
