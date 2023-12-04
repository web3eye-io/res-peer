<template>
  <div class='row'>
    <q-space />
    <div :style='{width:"720px"}'>
      <activity-card
        v-for='_activity in activities'
        :key='_activity.id'
        :activity='_activity'
        :style='{margin:"64px 0",paddingBottom:"32px",borderBottom:"1px solid grey"}'
      />
    </div>
    <q-space />
  </div>
</template>

<script lang='ts' setup>
import { useActivityStore } from 'src/stores/activity'
import { computed } from 'vue'

import ActivityCard from 'src/components/ActivityCard.vue'

const activity = useActivityStore()
const activities = computed(() => activity._activities())

</script>
