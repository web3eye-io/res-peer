<template>
  <div class='row q-gutter-sm' :style='{width: "1080px", margin: "32px auto"}'>
    <q-card
      :style='{width: "400px", height: "545px"}'
      v-for='_nft in nfts'
      :key='_nft.token_id'
      class='cursor-pointer q-hoverable'
      @mouseenter='onFocus(_nft)'
      @mouseleave='onBlur(_nft)'
    >
      <q-img
        :src='nftBanner(_nft)'
        width='100%'
        height='400px'
      >
        <template #error>
          <div class='absolute-full flex flex-center error' />
        </template>
      </q-img>
      <div :style='{padding: "16px 32px"}'>
        <div class='row'>
          <div :style='{fontWeight: 600, fontSize: "16px", marginBottom: "8px"}' class='text-cyan-8'>
            {{ _nft.collectionName }} #{{ _nft.token_id }}
          </div>
          <div v-if='_nft.name?.length' :style='{fontWeight: 600, fontSize: "16px", marginBottom: "8px"}' class='text-pink-9'>
            {{ ' (' + _nft.name + ')' }}
          </div>
        </div>
        <div class='text-grey-6'>
          {{ new Date(_nft.minted_at / 1000).toDateString() }}
        </div>
        <div>
          <strong class='text-positive' :style='{fontWeight: 600, fontSize: "16px"}'>{{ nftPrice(_nft) }}</strong> Linera
        </div>
      </div>
      <q-btn
        v-if='_nft.showBuy' flat label='Buy' :style='{width: "100%"}'
        class='text-white bg-primary text-bold'
      />
    </q-card>
  </div>
</template>

<script setup lang='ts'>

import { NFTExt, useCollectionStore } from 'src/stores/collection'
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'

interface Query {
  collectionId: number
}

const route = useRoute()
const query = computed(() => route.query as unknown as Query)

interface NFTAnother extends NFTExt {
  showBuy: boolean
}

const collection = useCollectionStore()
const collections = computed(() => collection.nftsByCollections(parseInt(query.value.collectionId.toString())))
const nfts = ref([] as Array<NFTAnother>)
watch(collections, () => {
  nfts.value = collections.value.map((el) => {
    return {
      showBuy: false,
      ...el
    } as NFTAnother
  })
})

const nftBanner = (_nft: NFTExt) => {
  return collection.nftBanner(_nft)
}

const nftPrice = (_nft: NFTExt) => {
  return _nft.price ? _nft.price : _nft.collectionPrice
}

const onFocus = (_nft: NFTAnother) => {
  _nft.showBuy = true
}

const onBlur = (_nft: NFTAnother) => {
  _nft.showBuy = false
}

onMounted(() => {
  nfts.value = collections.value.map((el) => {
    return {
      showBuy: false,
      ...el
    } as NFTAnother
  })
})

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/DefaultNFTBanner.png)
  border-radius: 8px
  background-size: cover
  background-repeat: no-repeat
</style>
