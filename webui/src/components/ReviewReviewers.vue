<template>
  <q-table
    :rows='reviewers'
    :columns='(columns as never)'
    @row-click='(_evt, row, _index) => onReviewerClick(row as Reviewer)'
  />
</template>

<script setup lang='ts'>
import { Reviewer, useReviewStore } from 'src/stores/review'
import { computed } from 'vue'
import { useUserStore } from 'src/stores/user'
import { useRouter } from 'vue-router'

const review = useReviewStore()
const reviewers = computed(() => Array.from(review.reviewerApplications.values()) || [])
const user = useUserStore()
const account = computed(() => user.account)
const router = useRouter()

const columns = computed(() => [
  {
    name: 'Reviewer',
    label: 'Reviewer',
    field: (row: Reviewer) => row.reviewer
  }, {
    name: 'Resume',
    label: 'Resume',
    field: (row: Reviewer) => row.resume
  }, {
    name: 'Approved',
    label: 'Approved',
    field: (row: Reviewer) => row.approved
  }, {
    name: 'Rejected',
    label: 'Rejected',
    field: (row: Reviewer) => row.rejected
  }, {
    name: 'Reviewed',
    label: 'Reviewed',
    field: (row: Reviewer) => review.reviewerReviewed(row.reviewer, account.value)
  }
])

const onReviewerClick = (reviewer: Reviewer) => {
  void router.push({
    path: '/reviewreviewer',
    query: {
      reviewer: reviewer.reviewer
    }
  })
}

</script>
