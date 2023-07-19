<template>
  <q-table
    :rows='nfts'
    :columns='(columns as never)'
  >
    <template #top-left>
      <div class='text-h5'>
        {{ label }}
      </div>
    </template>
  </q-table>
</template>

<script setup lang='ts'>
import { NFTExt, useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { computed, toRef } from 'vue'

interface Props {
  nftType: string
}

const props = defineProps<Props>()
const nftType = toRef(props, 'nftType')

const user = useUserStore()
const account = computed(() => user.account)
const collection = useCollectionStore()
const nfts = computed(() => {
  switch (nftType.value) {
    case 'MY_PUBLISHES':
      return collection.nftsByPublisher(account.value)
    case 'MY_ASSETS':
      return collection.nftsByOwner(account.value)
  }
  return collection.nftsByPublisher(account.value)
})

const columns = computed(() => [
  {
    name: 'Token ID',
    label: 'Token ID',
    align: 'left',
    field: (row: NFTExt) => row.token_id
  }, {
    name: 'Name',
    label: 'Name',
    field: (row: NFTExt) => row.name
  }, {
    name: 'Collection ID',
    label: 'Collection ID',
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
    field: (row: NFTExt) => new Date(row.minted_at / 1000).toDateString()
  }
])

const label = computed(() => {
  switch (nftType.value) {
    case 'MY_PUBLISHES':
      return 'My Publishes'
    case 'MY_ASSETS':
      return 'My Assets'
  }
  return 'My NFTs'
})

</script>
