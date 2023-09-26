<template>
  <div class='row q-gutter-sm' :style='{width: "1080px", margin: "32px auto"}'>
    <q-card
      :style='{width: "350px", borderRadius: "16px"}'
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
        <div class='row no-wrap text-no-wrap'>
          <div v-if='_nft.name?.length' :style='{fontWeight: 600, fontSize: "16px", marginBottom: "8px"}' class='text-pink-9'>
            {{ _nft.name }}
          </div>
          <div :style='{fontWeight: 600, fontSize: "16px", marginBottom: "8px", marginLeft: "8px"}' class='text-cyan-8 no-wrap'>
            {{ _nft.collectionName }} #{{ _nft.token_id }}
          </div>
        </div>
        <div class='text-grey-6'>
          {{ new Date(_nft.minted_at / 1000).toDateString() }}
        </div>
        <div>
          <strong class='text-positive' :style='{fontWeight: 600, fontSize: "16px"}'>{{ nftPrice(_nft) }}</strong> Linera
        </div>
      </div>
      <div class='row'>
        <q-btn
          flat label='Buy' :style='{width: "60%", borderRadius: "0 0 0 16px"}'
          class='text-white bg-primary text-bold'
          @click='onBuyClick(_nft)'
        />
        <q-input
          dense filled label='Credits to use' v-model='creditsToUse'
          type='number' :style='{width: "40%", borderRadius: "0 0 16px 0"}'
        />
      </div>
    </q-card>
  </div>
</template>

<script setup lang='ts'>

import { NFTExt, useCollectionStore } from 'src/stores/collection'
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { targetChain } from 'src/stores/chain'

interface Query {
  collectionId: number
}

const route = useRoute()
const query = computed(() => route.query as unknown as Query)

interface NFTAnother extends NFTExt {
  showBuy: boolean
}

const collection = useCollectionStore()
const collections = computed(() => collection.nftsByCollectionID(parseInt(query.value.collectionId?.toString())))
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

const creditsToUse = ref(0)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const onBuyClick = async (_nft: NFTAnother) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation buyNft ($collectionId: Int!, $tokenId: Int!, $credits: String) {
      buyNft(collectionId: $collectionId, tokenId: $tokenId, credits: $credits)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    collectionId: _nft.collectionId,
    tokenId: _nft.token_id,
    credits: creditsToUse.value.toString(),
    endpoint: 'market',
    chainId: targetChain.value
  })
}

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/DefaultNFTBanner.png)
  border-radius: 8px
  background-size: cover
  background-repeat: no-repeat
</style>
