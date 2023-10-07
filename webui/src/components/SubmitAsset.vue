<template>
  <div class='row'>
    <span class='text-h5'>Submit Asset</span>
    <q-space />
    <q-btn
      dense flat v-if='!editing' label='Submit'
      color='blue'
      @click='onEditClick'
    />
  </div>
  <div class='row'>
    <q-space />
    <q-btn
      dense flat v-if='editing' label='Submit'
      color='blue'
      @click='onSubmitClick'
    />
  </div>
  <q-input v-if='editing' dense label='Base uri for your collection storage (https or ipfs)' v-model='baseUri' />
  <ul v-if='editing && uris.length'>
    <li v-for='_uri in uris' :key='_uri'>
      {{ _uri }}
    </li>
  </ul>
  <div v-if='editing'>
    <div class='row' v-if='adding'>
      <q-input dense label='Uri for your collection nfts except base uri' :style='{width:"70%"}' v-model='uri' />
      <q-btn label='Confirm' @click='onConfirmAddUriClick' />
      <q-btn label='Cancel' @click='onCancelAddUriClick' />
    </div>
    <q-btn label='Add' @click='onAddUriClick' />
  </div>
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
import { targetChain } from 'src/stores/chain'
import { CID } from 'multiformats/cid'
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'

const editing = ref(false)
const baseUri = ref('')
const price = ref(0)
const uniquePrice = ref(false)
const name = ref('')
const uris = ref([] as Array<string>)
const uri = ref('')
const adding = ref(false)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const onSubmitClick = async () => {
  if (!baseUri.value.length) {
    return
  }
  if (!name.value.length) {
    return
  }
  if (!uris.value.length) {
    return
  }

  const bytes = json.encode({ uris })
  const hash = await sha256.digest(bytes)
  const cid = CID.create(1, json.code, hash).toString()

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation submitAsset ($cid: String!, $baseUri: String!, $uris: [String!]!, $price: String, $name: String!) {
      submitAsset(cid: $cid, baseUri: $baseUri, uris: $uris, price: $price, name: $name)
    }
  `))
  onDone(() => {
    editing.value = !editing.value
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    cid,
    baseUri: baseUri.value,
    uris: uris.value,
    price: uniquePrice.value ? price.value : undefined,
    name: name.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
}

const onAddUriClick = () => {
  adding.value = true
}

const onConfirmAddUriClick = () => {
  adding.value = false
  if (uri.value.length > 0 && uris.value.findIndex((el) => el === uri.value) < 0) {
    uris.value.push(uri.value)
  }
  uri.value = ''
}

const onCancelAddUriClick = () => {
  adding.value = false
}

const onEditClick = () => {
  editing.value = true
  uris.value = []
}

</script>
