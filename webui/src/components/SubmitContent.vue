<template>
  <div class='row'>
    <span class='text-h5'>Create Post</span>
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
      dense flat v-if='editing' label='Publish'
      color='blue'
      @click='onPublishClick'
    />
  </div>
  <q-input v-if='editing' dense label='Title' v-model='title' />
  <q-input
    v-if='editing' v-model='content' type='textarea' filled
    :style='{marginTop: "16px"}'
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { CID } from 'multiformats/cid'
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { targetChain } from 'src/stores/chain'

const title = ref('')
const content = ref('')
const editing = ref(false)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const onPublishClick = async () => {
  if (title.value.length <= 0 || content.value.length <= 0) {
    return
  }

  const bytes = json.encode({ content })
  const hash = await sha256.digest(bytes)
  const cid = CID.create(1, json.code, hash).toString()

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation submitContent ($cid: String!, $title: String!, $content: String!) {
      submitContent(cid: $cid, title: $title, content: $content)
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
    title: title.value,
    content: content.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
}

</script>
