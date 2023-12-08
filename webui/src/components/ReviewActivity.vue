<template>
  <div class='row'>
    <q-space />
    <div :style='{maxWidth:"960px"}'>
      <div class='row' :style='{margin:"16px 0"}'>
        <div class='row cursor-pointer' :style='{lineHeight:"32px"}' @click='onBackClick'>
          <q-icon name='arrow_back' size='32px' />
          <span :style='{marginLeft:"8px"}'>{{ $t('MSG_REVIEW_ACTIVITY') }}</span>
        </div>
        <q-space />
        <div class='row' :style='{lineHeight:"32px"}'>
          <span><strong>{{ _activity?.title }}</strong></span>
        </div>
      </div>
      <q-separator />
      <div v-if='_activity' :style='{marginTop:"24px"}'>
        <activity-card :activity='(_activity as Activity)' />
      </div>
      <q-separator />
      <div :style='{marginTop: "24px"}'>
        <q-input v-model='reason' type='textarea' :label='$t("MSG_REVIEW_REASON")' :disable='reviewed' />
      </div>
      <div :style='{margin: "24px 0"}' class='row'>
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
import { Cookies } from 'quasar'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { targetChain } from 'src/stores/chain'
import { useUserStore } from 'src/stores/user'
import { Activity, useActivityStore } from 'src/stores/activity'
import { v4 as uuidv4 } from 'uuid'

import ActivityCard from './ActivityCard.vue'

interface Query {
  activityId: number
}

const route = useRoute()
const activityId = computed(() => (route.query as unknown as Query).activityId)
const activity = useActivityStore()
const _activity = computed(() => activity.activity(Number(activityId.value)))
const review = useReviewStore()
const activityApplication = computed(() => review.activity(Number(activityId.value)))
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const router = useRouter()
const user = useUserStore()
const account = computed(() => user.account)
const reviewed = computed(() => review.activityReviewed(Number(activityId.value), account.value))
const _review = computed(() => review.activityReview(Number(activityId.value), account.value))
const reason = ref(_review.value?.reason || 'I supper like this activity, it will promote liquidity of Linera' + uuidv4())
const port = computed(() => Cookies.get('service-port'))

const onApproveClick = async () => {
  if (!activityApplication.value || !reason.value.length) {
    return
  }

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation approveActivity($activityId: Int!, $reason: String!) {
      approveActivity(activityId: $activityId, reason: $reason)
    }
  `))
  onDone(() => {
    review.activityMutateKeys.push(Number(activityId.value))
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    activityId: parseInt(activityId.value.toString()),
    reason: reason.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-activities',
      port: port.value
    }
  })
}

const onRejectClick = async () => {
  if (!activityApplication.value || !reason.value.length) {
    return
  }

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation rejectActivity ($activityId: Int!, $reason: String!) {
      rejectActivity(activityId: $activityId, reason: $reason)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    activityId: parseInt(activityId.value.toString()),
    reason: reason.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-activities',
      port: port.value
    }
  })
}

const onBackClick = () => {
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-activities',
      port: port.value
    }
  })
}

</script>
