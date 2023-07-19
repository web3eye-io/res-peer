<template>
  <q-table
    :rows='nfts'
    :columns='(columns as never)'
  >
    <template #top-left>
      <div class='text-h5'>
        My NFTs
      </div>
    </template>
  </q-table>
</template>

<script setup lang='ts'>
import { NFTExt, useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { computed } from 'vue'

const user = useUserStore()
const account = computed(() => user.account)
const collection = useCollectionStore()
const nfts = computed(() => collection.nfts(account.value))

const columns = computed(() => [
  {
    name: 'Token ID',
    label: 'Token ID',
    align: 'left',
    field: (row: NFTExt) => row.token_id
  }, {
    name: 'Collection ID',
    label: 'Collection ID',
    align: 'left',
    field: (row: NFTExt) => row.collectionId
  }, {
    name: 'Collection Name',
    label: 'Collection Name',
    field: (row: NFTExt) => row.collectionName
  }, {
    name: 'URI',
    label: 'URI',
    field: (row: NFTExt) => row.uri
  }, {
    name: 'Price',
    label: 'Price',
    field: (row: NFTExt) => row.price ? row.price : row.collectionPrice
  }, {
    name: 'Minted At',
    label: 'Minted At',
    field: (row: NFTExt) => new Date(row.minted_at / 1000).toString()
  }
])

</script>
