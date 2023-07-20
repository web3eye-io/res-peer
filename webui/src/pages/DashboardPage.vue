<template>
  <div :style='{width: "1080px", margin: "32px auto"}'>
    <div class='row'>
      <q-space />
      <div :style='{textAlign: "end"}'>
        <div>{{ account }}</div>
        <div>
          <span>{{ spendableCredits?.length ? spendableCredits + ' Credits' : '0 Credits' }}</span>
          <span>{{ lineraBalance?.length ? ', ' + lineraBalance + ' Linera' : ', 0 Linera' }}</span>
          <span> (1 Linera = {{ creditsPerLinera }} Credits)</span>
        </div>
      </div>
    </div>
    <q-splitter
      v-model='splitterModel'
    >
      <template #before>
        <q-tabs
          v-model='tab'
          vertical
          class='text-black'
        >
          <q-tab name='settings' label='Settings' />
          <q-tab name='contents' label='Contents' />
          <q-tab name='credits' label='Credits' />
          <q-tab name='assets' label='Assets' />
        </q-tabs>
      </template>
      <template #after>
        <q-tab-panels
          v-model='tab'
          animated
          swipeable
          vertical
          transition-prev='jump-up'
          transition-next='jump-up'
          class='text-black'
        >
          <q-tab-panel name='settings'>
            <avatar-setting />
          </q-tab-panel>
          <q-tab-panel name='contents'>
            <create-content />
            <q-separator :style='{margin: "32px 0"}' />
            <article-list article-type='MY_ARTICLE' :style='{margin: "32px 0"}' />
            <article-list article-type='MY_LIKE' :style='{margin: "32px 0"}' />
            <article-list article-type='MY_DISLIKE' :style='{margin: "32px 0"}' />
          </q-tab-panel>
          <q-tab-panel name='credits'>
            <user-balance />
          </q-tab-panel>
          <q-tab-panel name='assets'>
            <create-collection />
            <div :style='{margin: "32px 0"}'>
              <mint-nft />
            </div>
            <div :style='{margin: "32px 0"}'>
              <deposit-balance />
            </div>
            <collection-list :style='{margin: "32px 0"}' />
            <nft-list nft-type='MY_PUBLISHES' :style='{margin: "32px 0"}' />
            <nft-list nft-type='MY_ASSETS' :style='{margin: "32px 0"}' />
          </q-tab-panel>
        </q-tab-panels>
      </template>
    </q-splitter>
  </div>
</template>

<script setup lang='ts'>
import { computed, ref } from 'vue'
import { useUserStore } from 'src/stores/user'
import { useCollectionStore } from 'src/stores/collection'

import CreateContent from 'src/components/CreateContent.vue'
import UserBalance from 'src/components/UserBalance.vue'
import ArticleList from 'src/components/ArticleList.vue'
import CreateCollection from 'src/components/CreateCollection.vue'
import MintNft from 'src/components/MintNFT.vue'
import CollectionList from 'src/components/CollectionList.vue'
import NftList from 'src/components/NftList.vue'
import DepositBalance from 'src/components/DepositBalance.vue'
import AvatarSetting from 'src/components/AvatarSetting.vue'

const user = useUserStore()
const account = computed(() => user.account)
const collection = useCollectionStore()
const creditsPerLinera = computed(() => collection.creditsPerLinera)
const spendableCredits = computed(() => user.spendable)
const lineraBalance = computed(() => collection.lineraBalance)

const splitterModel = ref(20)
const tab = ref('contents')

</script>
