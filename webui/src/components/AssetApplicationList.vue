<template>
  <q-table
    :rows='assets'
    :columns='(columns as never)'
  >
    <template #top-left>
      <div class='text-h5'>
        Asset Applications
      </div>
    </template>
  </q-table>
</template>

<script setup lang='ts'>
import { useReviewStore, Asset } from 'src/stores/review'
import { useUserStore } from 'src/stores/user'
import { computed } from 'vue'

const user = useUserStore()
const review = useReviewStore()
const account = computed(() => user.account)
const assets = computed(() => review.assets(account.value))

const columns = computed(() => [
  {
    name: 'Base URI',
    label: 'Base URI',
    field: (row: Asset) => row.baseUri
  }, {
    name: 'Name',
    label: 'Name',
    field: (row: Asset) => row.name
  }, {
    name: 'Price',
    label: 'Price',
    field: (row: Asset) => row.price ? row.price : 'NOT SET'
  }, {
    name: 'Approved',
    label: 'Approved',
    field: (row: Asset) => row.approved
  }, {
    name: 'Rejected',
    label: 'Rejected',
    field: (row: Asset) => row.rejected
  }, {
    name: 'Created At',
    label: 'Created At',
    field: (row: Asset) => new Date(row.createdAt / 1000).toDateString()
  }, {
    name: 'State',
    label: 'State',
    field: (row: Asset) => row.rejected >= review.assetRejectedThreshold ? 'Rejected' : (review.assetRejectedThreshold - row.approved).toString() + ' Approvals Needed'
  }
])

</script>
