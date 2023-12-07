<template>
  <q-table
    :rows='activities'
    :columns='(columns as never)'
    @row-click='(_evt, row, _index) => onActivityClick(row as Activity)'
  />
</template>

<script setup lang='ts'>
import { Activity, useReviewStore } from 'src/stores/review'
import { computed } from 'vue'
import { useUserStore } from 'src/stores/user'
import { useRouter } from 'vue-router'
import { useFoundationStore } from 'src/stores/foundation'
import { Cookies } from 'quasar'
import { useActivityStore } from 'src/stores/activity'

const review = useReviewStore()
const activities = computed(() => Array.from(review.activityApplications.values()) || [])
const user = useUserStore()
const account = computed(() => user.account)
const router = useRouter()
const foundation = useFoundationStore()
const estimatedReward = computed(() => Number(foundation.reviewRewardBalance) / foundation.reviewRewardFactor)
const port = computed(() => Cookies.get('service-port'))
const activity = useActivityStore()

const columns = computed(() => [
  {
    name: 'ActivityID',
    label: 'Activity ID',
    field: (row: Activity) => row.activityId
  }, {
    name: 'Title',
    label: 'Title',
    field: (row: Activity) => activity.activity(row.activityId)?.title
  }, {
    name: 'Approved',
    label: 'Approved',
    field: (row: Activity) => row.approved
  }, {
    name: 'Rejected',
    label: 'Rejected',
    field: (row: Activity) => row.rejected
  }, {
    name: 'Estimated Reward',
    label: 'Estimated Reward',
    field: (row: Activity) => review.activityReviewed(row.activityId, account.value) ? '-' : estimatedReward.value.toString() + ' Lineras'
  }, {
    name: 'Reviewed',
    label: 'Reviewed',
    field: (row: Activity) => review.activityReviewed(row.activityId, account.value)
  }
])

const onActivityClick = (activity: Activity) => {
  void router.push({
    path: '/reviewactivity',
    query: {
      activityId: activity.activityId,
      port: port.value
    }
  })
}

</script>
