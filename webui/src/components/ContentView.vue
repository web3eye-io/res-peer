<template>
  <div class='row'>
    <q-space />
    <div>
      <div :style='{paddingTop:"48px"}'>
        <content-card-view :cid='cid' @comment='onCommentClick' :list='false' :expand='true' />
      </div>
      <div v-if='commenting'>
        <q-input v-model='comment' :label='$t("MSG_COMMENT")' :placeholder='placeHolder' />
        <div class='row' :style='{marginTop:"24px"}'>
          <q-btn :label='$t("MSG_SUBMIT")' :style='{marginRight:"8px"}' @click='onSubmitClick' />
          <q-btn :label='$t("MSG_CANCEL")' @click='onCancelClick' />
        </div>
      </div>
    </div>
    <q-space />
  </div>
</template>

<script setup lang='ts'>
import { computed, ref } from 'vue'
import { useRoute } from 'vue-router'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { CID } from 'multiformats/cid'
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { targetChain } from 'src/stores/chain'

import ContentCardView from './ContentCardView.vue'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

interface Query {
  cid: string
}
const route = useRoute()
const cid = computed(() => (route.query as unknown as Query).cid)
const commenting = ref(false)
const comment = ref('')
const placeHolder = ref('Please enter meaningful comment :)')

const onCommentClick = () => {
  commenting.value = true
}

const onSubmitClick = async () => {
  if (comment.value.length <= 0) {
    return
  }

  const bytes = json.encode({ comment })
  const hash = await sha256.digest(bytes)
  const commentCid = CID.create(1, json.code, hash).toString()

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation submitComment ($cid: String!, $commentCid: String!, $comment: String!) {
      submitComment(cid: $cid, commentCid: $commentCid, comment: $comment)
    }
  `))
  onDone(() => {
    commenting.value = false
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    cid: cid.value,
    commentCid,
    comment: comment.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
}

const onCancelClick = () => {
  commenting.value = false
}

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/ResPeer.png)
  border-radius: 50%
  background-size: cover
  background-repeat: no-repeat
</style>
