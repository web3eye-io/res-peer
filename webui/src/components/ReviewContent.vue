<template>
  <q-table
    :rows='contents'
    :columns='(columns as never)'
    @row-click='(_evt, row, _index) => onContentClick(row as Content)'
  />
  <q-dialog
    v-model='reviewing'
    position='standard'
  >
    <q-card :style='{padding:"24px", width:"1280px"}'>
      <q-card-section>
        <q-item-label>CID: {{ target.cid }}</q-item-label>
        <q-item-label>Author: {{ target.author }}</q-item-label>
        <q-item-label v-if='target.commentToCid'>
          Comment To CID: {{ target.commentToCid }}
        </q-item-label>
        <q-item-label>Title: {{ target.title }}</q-item-label>
        <q-item-label>Content: <span v-html='target.content' /></q-item-label>
        <q-item-label>Approved: {{ target.approved }}</q-item-label>
        <q-item-label>Rejected: {{ target.rejected }}</q-item-label>
        <q-item-label>CreatedAt: {{ date.formatDate(target.createdAt / 1000) }}</q-item-label>
      </q-card-section>
      <q-input v-model='reason' :label='$t("MSG_REVIEW_REASON")' type='textarea' />
      <div class='row' :style='{marginTop: "24px"}'>
        <q-btn @click='onApprove'>
          {{ $t('MSG_APPROVE') }}
        </q-btn>
        <q-btn @click='onReject'>
          {{ $t('MSG_REJECT') }}
        </q-btn>
      </div>
    </q-card>
  </q-dialog>
</template>

<script setup lang='ts'>
import { Content, useReviewStore } from 'src/stores/review'
import { computed, ref } from 'vue'
import { date } from 'quasar'
import { useUserStore } from 'src/stores/user'

const review = useReviewStore()
const contents = computed(() => Array.from(review.contentApplications.values()) || [])
const user = useUserStore()
const account = computed(() => user.account)
const reviewing = ref(false)
const target = ref(undefined as unknown as Content)
const reason = ref('')

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
    name: 'Reviewed',
    label: 'Reviewed',
    field: (row: Content) => review.reviewed(row.cid, account.value)
  }
])

const onContentClick = (content: Content) => {
  target.value = content
  reviewing.value = true
}

const onApprove = () => {
  reviewing.value = false
}

const onReject = () => {
  reviewing.value = false
}

</script>
