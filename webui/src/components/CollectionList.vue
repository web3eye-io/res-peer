<template>
  <q-table
    :rows='collections'
    :columns='(columns as never)'
  >
    <template #top-left>
      <div class='text-h5'>
        My Collections
      </div>
    </template>
  </q-table>
</template>

<script setup lang='ts'>
import { Collection, useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { computed } from 'vue'

const collection = useCollectionStore()
const user = useUserStore()
const account = computed(() => user.account)
const collections = computed(() => Array.from(collection._collections(account.value)))

const columns = computed(() => [
  {
    name: 'Collection ID',
    label: 'Collection ID',
    align: 'left',
    field: (row: Collection) => row.collectionId
  }, {
    name: 'Base URI',
    label: 'Base URI',
    field: (row: Collection) => row.baseUri
  }, {
    name: 'Name',
    label: 'Name',
    field: (row: Collection) => row.name
  }, {
    name: 'Price',
    label: 'Price',
    field: (row: Collection) => row.price ? row.price : 'NOT SET'
  }, {
    name: 'NFTs',
    label: 'NFTs',
    field: (row: Collection) => row.nfts?.size ? row.nfts.size : 0
  }, {
    name: 'Created At',
    label: 'Created At',
    field: (row: Collection) => new Date(row.createdAt / 1000).toDateString()
  }, {
    name: 'Publisher',
    label: 'Publisher',
    field: (row: Collection) => row.publisher.substring(0, 5) + '...' + row.publisher.substring(row.publisher.length - 5)
  }
])

</script>
