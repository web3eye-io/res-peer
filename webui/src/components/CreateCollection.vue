<template>
  <div class='row'>
    <span class='text-h5'>Create Collection</span>
    <q-space />
    <q-btn
      dense flat v-if='!editing' label='Create'
      color='blue'
      @click='editing = !editing'
    />
  </div>
  <div class='row'>
    <q-space />
    <q-btn
      dense flat v-if='editing' label='Create'
      color='blue'
      @click='onCreateClick'
    />
  </div>
  <q-input v-if='editing' dense label='Base uri for your collection storage (https or ipfs)' v-model='baseUri' />
  <q-toggle v-if='editing' label='Unique Price' v-model='uniquePrice' />
  <q-input
    v-if='editing && uniquePrice' v-model='price' type='number'
    label='Price' :style='{marginTop: "16px"}'
  />
  <q-input v-if='editing' v-model='name' label='Collection name' />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'

const editing = ref(false)
const baseUri = ref('')
const price = ref(0)
const uniquePrice = ref(false)
const name = ref('')

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const onCreateClick = async () => {
  if (!baseUri.value.length) {
    return
  }
  if (!name.value.length) {
    return
  }

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation createCollection ($baseUri: String!, $price: String, $name: String!) {
      createCollection(baseUri: $baseUri, price: $price, name: $name)
    }
  `))
  onDone(() => {
    editing.value = !editing.value
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    baseUri: baseUri.value,
    price: uniquePrice.value ? price : undefined,
    name: name.value,
    endpoint: 'mall'
  })
}

</script>
