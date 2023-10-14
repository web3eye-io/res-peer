<template>
  <q-table
    :rows='contents'
    :columns='(columns as never)'
    @row-click='(_evt, row, _index) => onContentClick(row as Content)'
  />
</template>

<script setup lang='ts'>
import { Content, useReviewStore } from 'src/stores/review'
import { computed } from 'vue'
import { useUserStore } from 'src/stores/user'
import { useRouter } from 'vue-router'
import { useFoundationStore } from 'src/stores/foundation'
import { Cookies } from 'quasar'

const review = useReviewStore()
const contents = computed(() => Array.from(review.contentApplications.values()) || [])
const user = useUserStore()
const account = computed(() => user.account)
const router = useRouter()
const foundation = useFoundationStore()
const estimatedReward = computed(() => Number(foundation.reviewRewardBalance) / foundation.reviewRewardFactor)
const port = computed(() => Cookies.get('service-port'))

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
  }, {
    name: 'Estimated Reward',
    label: 'Estimated Reward',
    field: (row: Content) => review.contentReviewed(row.cid, account.value) ? '-' : estimatedReward.value.toString() + ' Lineras'
  }, {
    name: 'Reviewed',
    label: 'Reviewed',
    field: (row: Content) => review.contentReviewed(row.cid, account.value)
  }
])

const onContentClick = (content: Content) => {
  void router.push({
    path: '/reviewcontent',
    query: {
      cid: content.cid,
      port: port.value
    }
  })
}

</script>
