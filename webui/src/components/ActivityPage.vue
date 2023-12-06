<template>
  <q-table
    :rows='activities'
    :columns='(columns as never)'
    @row-click='(evt, row, index) => onActivityClick(row)'
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
import { Cookies, date } from 'quasar'

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
    field: (row: Activity) => date.formatDate(row.registerStartAt)
  }, {
    name: 'VoteStartAt',
    label: 'Vote Start At',
    field: (row: Activity) => date.formatDate(row.voteStartAt)
  }
])

const port = computed(() => Cookies.get('service-port'))

const router = useRouter()
const onCreateActivityClick = () => {
  void router.push({
    path: '/create/activity',
    query: {
      port: port.value
    }
  })
}

const onActivityClick = (activity: Activity) => {
  void router.push({
    path: '/create/activity',
    query: {
      port: port.value,
      activityId: activity.id
    }
  })
}

</script>
