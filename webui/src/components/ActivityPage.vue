<template>
  <q-table
    :rows='activities'
    :columns='(columns as never)'
  >
    <template #top-left>
      <div class='text-h5'>
        My Activities
      </div>
    </template>
    <template #top-right>
      <q-btn
        dense flat label='Create'
        color='blue'
        @click='onCreateActivityClick'
      />
    </template>
  </q-table>
</template>

<script lang='ts' setup>
import { Activity, useActivityStore } from 'src/stores/activity'
import { useUserStore } from 'src/stores/user'
import { computed } from 'vue'
import { useRouter } from 'vue-router'

const user = useUserStore()
const account = computed(() => user.account)
const activity = useActivityStore()
const activities = computed(() => activity._activities(account.value))

const columns = computed(() => [
  {
    name: 'ID',
    label: 'ID',
    field: (row: Activity) => row.id
  }, {
    name: 'Title',
    label: 'Title',
    field: (row: Activity) => row.title
  }, {
    name: 'RegisterStartAt',
    label: 'Register Start At',
    field: (row: Activity) => row.registerEndAt
  }, {
    name: 'VoteStartAt',
    label: 'Vote Start At',
    field: (row: Activity) => row.voteStartAt
  }
])

const router = useRouter()
const onCreateActivityClick = () => {
  void router.push({ path: '/create/activity' })
}

</script>
