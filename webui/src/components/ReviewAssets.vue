<template>
  <q-table
    :rows='assets'
    :columns='(columns as never)'
    @row-click='(_evt, row, _index) => onAssetClick(row as Asset)'
  />
</template>

<script setup lang='ts'>
import { Asset, useReviewStore } from 'src/stores/review'
import { computed } from 'vue'
import { useUserStore } from 'src/stores/user'
import { useRouter } from 'vue-router'
import { useFoundationStore } from 'src/stores/foundation'
import { Cookies } from 'quasar'

const review = useReviewStore()
const assets = computed(() => Array.from(review.assetApplications.values()) || [])
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
    field: (row: Asset) => row.cid
  }, {
    name: 'Title',
    label: 'Title',
    field: (row: Asset) => row.name
  }, {
    name: 'Approved',
    label: 'Approved',
    field: (row: Asset) => row.approved
  }, {
    name: 'Rejected',
    label: 'Rejected',
    field: (row: Asset) => row.rejected
  }, {
    name: 'Estimated Reward',
    label: 'Estimated Reward',
    field: (row: Asset) => review.assetReviewed(row.cid, account.value) ? '-' : estimatedReward.value.toString() + ' Lineras'
  }, {
    name: 'Reviewed',
    label: 'Reviewed',
    field: (row: Asset) => review.assetReviewed(row.cid, account.value)
  }
])

const onAssetClick = (asset: Asset) => {
  void router.push({
    path: '/reviewasset',
    query: {
      cid: asset.cid,
      port: port.value
    }
  })
}

</script>
