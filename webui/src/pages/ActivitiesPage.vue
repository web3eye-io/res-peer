<template>
  <div class='row'>
    <q-space />
    <div :style='{width:"960px"}'>
      <activity-card
        v-for='(_activity, i) in activities'
        :key='_activity.id'
        :activity='_activity'
        :style='{margin:"64px 0",paddingBottom:"32px",borderBottom: i < activities.length - 1 ? "1px solid grey" : ""}'
      />
    </div>
    <q-space />
  </div>
</template>

<script lang='ts' setup>
import { useActivityStore } from 'src/stores/activity'
import { computed } from 'vue'
import { useReviewStore } from 'src/stores/review'

import ActivityCard from 'src/components/ActivityCard.vue'

const review = useReviewStore()
const reviewerNumber = computed(() => review.reviewerNumber)
const approvedThreshold = computed(() => review.activityApprovedThreshold)
const activityApplications = computed(() => review.activityApplications)

const activity = useActivityStore()
const activities = computed(() => activity._activities().filter((el) => {
  const approved = activityApplications.value.get(el.id)?.approved
  if (!approved) {
    return false
  }
  return approved >= approvedThreshold.value || approved >= reviewerNumber.value
}))

</script>
