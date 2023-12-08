<template>
  <div class='row'>
    <q-space />
    <div :style='{maxWidth:"800px"}'>
      <div class='row' :style='{margin:"16px 0"}'>
        <div class='row cursor-pointer' :style='{lineHeight:"32px"}' @click='onBackClick'>
          <q-icon name='arrow_back' size='32px' />
          <span :style='{marginLeft:"8px"}'>{{ $t('MSG_REVIEW_CONTENT') }}</span>
        </div>
        <q-space />
        <div class='row' :style='{lineHeight:"32px"}'>
          <span><strong>{{ content?.title }}</strong></span>
        </div>
      </div>
      <q-separator />
      <div :style='{marginTop:"24px"}'>
        <div :style='{fontWeight: "bold", fontSize: "28px", wordBreak: "break-word", marginBottom: "16px"}'>
          {{ content?.title || 'You should have a title!' }}
        </div>
        <div>
          By
          <span class='text-grey-6 text-bold cursor-pointer'>
            {{ content?.author || 'Anonymous' }}
          </span>
        </div>
        <div>
          At
          <span class='text-grey-6 text-bold'>
            {{ content?.createdAt ? date.formatDate(content?.createdAt / 1000) : '' }}
          </span>
        </div>
        <div>
          Cid
          <span class='text-grey-6 text-bold cursor-pointer'>
            {{ content?.cid }}
          </span>
        </div>
        <div
          :style='{margin: "24px 0 24px 0", fontSize: "16px", wordBreak: "break-word"}'
          v-html='content?.content || "You should have some content!"'
        />
      </div>
      <q-separator />
      <div :style='{marginTop: "24px"}'>
        <q-input v-model='reason' type='textarea' :label='$t("MSG_REVIEW_REASON")' :disable='reviewed' />
      </div>
      <div :style='{marginTop: "24px"}' class='row'>
        <q-btn :label='$t("MSG_APPROVE")' :style='{marginRight:"16px",color: _review?.approved ? "blue" : ""}' @click='onApproveClick' :disable='reviewed' />
        <q-btn :label='$t("MSG_REJECT")' :style='{color: _review && !_review?.approved ? "red" : ""}' @click='onRejectClick' :disable='reviewed' />
      </div>
    </div>
    <q-space />
  </div>
</template>

<script lang='ts' setup>
import { useReviewStore } from 'src/stores/review'
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Cookies, date } from 'quasar'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { CID } from 'multiformats/cid'
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'
import { targetChain } from 'src/stores/chain'
import { useUserStore } from 'src/stores/user'
import { v4 as uuidv4 } from 'uuid'

interface Query {
  cid: string
}

const route = useRoute()
const cid = computed(() => (route.query as unknown as Query).cid)
const review = useReviewStore()
const content = computed(() => review.content(cid.value))
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const router = useRouter()
const user = useUserStore()
const account = computed(() => user.account)
const reviewed = computed(() => review.contentReviewed(cid.value, account.value))
const _review = computed(() => review.contentReview(cid.value, account.value))
const reason = ref(_review.value?.reason || 'I supper like this article not only it\'s about Linera, but also it\'s write by KK.' + uuidv4())
const port = computed(() => Cookies.get('service-port'))

const onApproveClick = async () => {
  if (!content.value || !reason.value.length) {
    return
  }

  const bytes = json.encode({ reason })
  const hash = await sha256.digest(bytes)
  const cid = CID.create(1, json.code, hash).toString()

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation approveContent ($contentCid: String!, $reasonCid: String!, $reason: String!) {
      approveContent(contentCid: $contentCid, reasonCid: $reasonCid, reason: $reason)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    contentCid: content.value.cid,
    reasonCid: cid,
    reason: reason.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
  void router.push({
    path: '/',
    query: {
      port: port.value
    }
  })
}

const onRejectClick = async () => {
  if (!content.value || !reason.value.length) {
    return
  }

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation rejectContent ($contentCid: String!, $reason: String!) {
      rejectContent(contentCid: $contentCid, reason: $reason)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    contentCid: content.value.cid,
    reason: reason.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-contents',
      port: port.value
    }
  })
}

const onBackClick = () => {
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-contents',
      port: port.value
    }
  })
}

</script>
