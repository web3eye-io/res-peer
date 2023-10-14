<template>
  <div class='row'>
    <q-input dense label='Resume url' v-model='resume' :style='{width:"80%"}' />
    <q-btn @click='onApplyClick' flat class='text-blue'>
      {{ reviewerApplication ? 'Update Resume' : 'Apply Reviewer' }}
    </q-btn>
    <div v-if='!rejected && reviewerApplication'>
      {{ approved ? 'Approved' : reviewerApplication?.approved + ' reviewers approved (' + approvedThreshold + ' needed)' }}
    </div>
    <div v-if='rejected'>
      {{ reviewerApplication?.rejected + ' reviewers rejected your application! Please check the review and try again.' }}
    </div>
  </div>
</template>

<script lang='ts' setup>
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { useApplicationStore } from 'src/stores/application'
import { useUserStore } from 'src/stores/user'
import { computed, ref } from 'vue'
import { targetChain } from 'src/stores/chain'
import { useReviewStore } from 'src/stores/review'

const application = useApplicationStore()
const user = useUserStore()
const reviewerApplication = computed(() => user.reviewerApplication)
const review = useReviewStore()
const approvedThreshold = computed(() => review.reviewerApprovedThreshold > review.reviewerNumber ? review.reviewerNumber : review.reviewerApprovedThreshold)
const rejectedThreshold = computed(() => review.reviewerRejectedThreshold)
const approved = computed(() => reviewerApplication.value?.approved >= approvedThreshold.value)
const rejected = computed(() => reviewerApplication.value?.rejected >= rejectedThreshold.value)
const reviewApp = computed(() => application.reviewApp)
const account = computed(() => user.account)
const resume = ref(reviewerApplication.value?.resume)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = (): boolean => {
  return account.value?.length > 0 && reviewApp.value?.length > 0 && targetChain.value?.length > 0 && resume.value?.length > 0
}

const applyReviewer = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation applyReviewer($resume: String!) {
      applyReviewer(resume: $resume)
    }
  `))
  onDone(() => {
    review.reviewerMutateKeys.push(account.value)
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    resume: resume.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
}

const onApplyClick = async () => {
  if (!ready()) return
  await applyReviewer()
}

</script>
