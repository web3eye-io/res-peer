<template>
  <div>{{ activity }}</div>
</template>

<script lang='ts' setup>
import { defineProps, toRef } from 'vue'

import { Activity } from 'src/stores/activity'

interface Props {
  activity: Activity
}

const props = defineProps<Props>()
const activity = toRef(props, 'activity')

</script>
