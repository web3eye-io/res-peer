<template>
  <div class='row'>
    <q-space />
    <div :style='{width:"1280px"}'>
      <div class='row' :style='{margin:"16px 0"}'>
        <div class='row cursor-pointer' :style='{lineHeight:"32px"}' @click='onBackClick'>
          <q-icon name='arrow_back' size='32px' />
          <span :style='{marginLeft:"8px"}'>{{ $t('MSG_REVIEW_REVIEWER') }}</span>
        </div>
        <q-space />
        <div class='row' :style='{lineHeight:"32px"}'>
          <span><strong>{{ reviewer?.reviewer }}</strong></span>
        </div>
      </div>
      <q-separator />
      <div :style='{margin:"24px 0"}'>
        <div :style='{fontWeight: "bold", fontSize: "18px", wordBreak: "break-word", marginBottom:"16px"}'>
          {{ reviewer?.reviewer || 'You should have an address!' }}
        </div>
        <div>
          Chain
          <span class='text-grey-6 text-bold'>
            {{ reviewer?.chainId || 'You should have a chain id!' }}
          </span>
        </div>
        <div>
          At
          <span class='text-grey-6 text-bold'>
            {{ reviewer?.createdAt ? date.formatDate(reviewer?.createdAt / 1000) : '' }}
          </span>
        </div>
        <div>
          Resume
          <a v-if='reviewer?.resume?.length' :href='reviewer.resume'>{{ reviewer.resume }}</a>
          <span v-else class='text-grey-6 text-bold'>
            You should have a resume!
          </span>
        </div>
      </div>
      <iframe
        :style='{margin:"24px 0"}'
        :src='reviewer?.resume'
        scrolling='yes'
        width='100%'
        height='640px'
      />
      <q-separator />
      <div :style='{marginTop: "24px"}'>
        <q-input v-model='reason' type='textarea' :label='$t("MSG_REVIEW_REASON")' :disable='reviewed' />
      </div>
      <div :style='{marginTop: "24px", marginBottom: "48px"}' class='row'>
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
import { targetChain } from 'src/stores/chain'
import { useUserStore } from 'src/stores/user'
import { v4 as uuidv4 } from 'uuid'

interface Query {
  reviewer: string
}

const route = useRoute()
const candidate = computed(() => (route.query as unknown as Query).reviewer)
const review = useReviewStore()
const reviewer = computed(() => review.reviewer(candidate.value))
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const router = useRouter()
const user = useUserStore()
const account = computed(() => user.account)
const reviewed = computed(() => review.reviewerReviewed(candidate.value, account.value))
const _review = computed(() => review.reviewerReview(candidate.value, account.value))
const reason = ref(_review.value?.reason || 'I supper like this man not only it\'s from Linera, but also it\'s recommended by KK.' + uuidv4())
const port = computed(() => Cookies.get('service-port'))

const onApproveClick = async () => {
  if (!reviewer.value || !reason.value.length) {
    return
  }

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation approveReviewer ($candidate: String!, $reason: String!) {
      approveReviewer(candidate: $candidate, reason: $reason)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    candidate: reviewer.value.reviewer,
    reason: reason.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-reviewers',
      port: port.value
    }
  })
}

const onRejectClick = async () => {
  if (!reviewer.value || !reason.value.length) {
    return
  }

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation rejectReviewer ($candidate: String!, $reason: String!) {
      rejectReviewer(candidate: $candidate, reason: $reason)
    }
  `))
  onDone(() => {
    review.reviewerMutateKeys.push(candidate.value)
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    candidate: reviewer.value.reviewer,
    reason: reason.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-reviewers',
      port: port.value
    }
  })
}

const onBackClick = () => {
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-reviewers',
      port: port.value
    }
  })
}

</script>
