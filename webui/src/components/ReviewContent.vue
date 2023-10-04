<template>
  <q-table
    :rows='contents'
    :columns='(columns as never)'
  />
</template>

<script setup lang='ts'>
import { Content, useReviewStore } from 'src/stores/review'
import { computed } from 'vue'

const review = useReviewStore()
const contents = computed(() => Array.from(review.contentApplications.values()) || [])

const columns = computed(() => [
  {
    name: 'CID',
    label: 'CID',
    field: (row: Content) => row.cid
  }, {
    name: 'Title',
    label: 'Title',
    field: (row: Content) => row.title
  }, {
    name: 'Approved',
    label: 'Approved',
    field: (row: Content) => row.approved
  }, {
    name: 'Rejected',
    label: 'Rejected',
    field: (row: Content) => row.rejected
  }
])

</script>
