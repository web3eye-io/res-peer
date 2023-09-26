<template>
  <div class='row'>
    <span class='text-h5'>Avatar</span>
    <q-space />
    <q-btn
      dense flat v-if='!editing' label='Set Avatar'
      color='blue'
      @click='editing = !editing'
    />
  </div>
  <div class='row'>
    <q-space />
    <q-btn
      dense flat v-if='editing' label='Set Avatar'
      color='blue'
      @click='onSetAvatarClick'
    />
  </div>
  <q-input
    v-if='editing' v-model='collectionId' type='number' filled
    :style='{marginTop: "16px"}' label='Avatar asset collection id'
  />
  <q-input
    v-if='editing' v-model='tokenId' type='number' filled
    :style='{marginTop: "16px"}' label='Avatar asset token id'
  />
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { targetChain } from 'src/stores/chain'

const editing = ref(false)
const collectionId = ref(0)
const tokenId = ref(0)
const collection = useCollectionStore()
const user = useUserStore()
const account = computed(() => user.account)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const onSetAvatarClick = async () => {
  if (collectionId.value <= 0) {
    return
  }
  if (tokenId.value <= 0) {
    return
  }

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation setAvatar ($collectionId: Int!, $tokenId: Int!) {
      setAvatar(collectionId: $collectionId, tokenId: $tokenId)
    }
  `))
  onDone(() => {
    editing.value = !editing.value
    collection.avatars.set(account.value, [collectionId.value, tokenId.value])
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    collectionId: parseInt(collectionId.value.toString()),
    tokenId: parseInt(tokenId.value.toString()),
    endpoint: 'market',
    chainId: targetChain.value
  })
}

</script>
