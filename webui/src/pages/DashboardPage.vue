<template>
  <div :style='{width: "1440px", margin: "32px auto"}'>
    <div class='row'>
      <q-space />
      <div class='row' :style='{textAlign: "end"}'>
        <div :style='{marginTop: "4px"}' class='text-green-7'>
          <span class='text-bold'>{{ account }}</span>
          <div>
            <span>{{ spendableCredits?.length ? spendableCredits + ' Credits' : '0 Credits' }}</span>
            <span>{{ lineraBalance?.length ? ', ' + lineraBalance + ' Linera' : ', 0 Linera' }}</span>
            <span> (1 Linera = {{ creditsPerLinera }} Credits)</span>
          </div>
        </div>
        <q-avatar :style='{marginLeft: "8px"}' size='48px'>
          <q-img
            :src='avatar?.length ? avatar : "~assets/ResPeer.png"'
            width='48px'
            height='48px'
            fit='contain'
            :style='{borderRadius: "50%"}'
          >
            <template #error>
              <div class='absolute-full flex flex-center error' />
            </template>
          </q-img>
        </q-avatar>
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
          <q-tab v-if='reviewer' name='review-contents' label='Review Content' />
          <q-tab v-if='reviewer' name='review-assets' label='Review Asset' />
          <q-tab v-if='reviewer' name='review-reviewers' label='Review Reviewer' />
          <q-tab v-if='reviewer' name='review-activities' label='Review Activity' />
          <q-tab v-if='!reviewer' name='apply-reviewer' label='Apply Reviewer' />
          <q-tab name='foundation' label='Foundation' />
          <q-tab name='activity' label='Activity' />
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
            <submit-content />
            <q-separator :style='{margin: "32px 0"}' />
            <content-application-list :style='{margin: "32px 0"}' />
            <article-list article-type='MY_ARTICLE' :style='{margin: "32px 0"}' />
            <article-list article-type='MY_LIKE' :style='{margin: "32px 0"}' />
            <article-list article-type='MY_DISLIKE' :style='{margin: "32px 0"}' />
          </q-tab-panel>
          <q-tab-panel name='review-contents'>
            <review-contents />
          </q-tab-panel>
          <q-tab-panel name='review-assets'>
            <review-assets />
          </q-tab-panel>
          <q-tab-panel name='review-reviewers'>
            <review-reviewers />
          </q-tab-panel>
          <q-tab-panel name='review-activities'>
            <review-activities />
          </q-tab-panel>
          <q-tab-panel name='credits'>
            <user-balance />
          </q-tab-panel>
          <q-tab-panel name='assets'>
            <submit-asset />
            <div :style='{margin: "32px 0"}'>
              <mint-nft />
            </div>
            <asset-application-list :style='{margin: "32px 0"}' />
            <collection-list :style='{margin: "32px 0"}' />
            <nft-list nft-type='MY_PUBLISHES' :style='{margin: "32px 0"}' />
            <nft-list nft-type='MY_ASSETS' :style='{margin: "32px 0"}' />
          </q-tab-panel>
          <q-tab-panel name='apply-reviewer'>
            <apply-reviewer />
          </q-tab-panel>
          <q-tab-panel name='foundation'>
            <div :style='{marginBottom: "16px"}'>
              <deposit-balance />
            </div>
            <q-separator />
            <div :style='{marginTop: "16px"}'>
              <foundation-page />
            </div>
          </q-tab-panel>
          <q-tab-panel name='activity'>
            <activity-page />
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
import { useFoundationStore } from 'src/stores/foundation'
import { useRoute } from 'vue-router'

import SubmitContent from 'src/components/SubmitContent.vue'
import UserBalance from 'src/components/UserBalance.vue'
import ArticleList from 'src/components/ArticleList.vue'
import SubmitAsset from 'src/components/SubmitAsset.vue'
import MintNft from 'src/components/MintNFT.vue'
import CollectionList from 'src/components/CollectionList.vue'
import NftList from 'src/components/NftList.vue'
import DepositBalance from 'src/components/DepositBalance.vue'
import AvatarSetting from 'src/components/AvatarSetting.vue'
import ReviewContents from 'src/components/ReviewContents.vue'
import ReviewAssets from 'src/components/ReviewAssets.vue'
import ReviewReviewers from 'src/components/ReviewReviewers.vue'
import ApplyReviewer from 'src/components/ApplyReviewer.vue'
import ContentApplicationList from 'src/components/ContentApplicationList.vue'
import AssetApplicationList from 'src/components/AssetApplicationList.vue'
import FoundationPage from 'src/components/FoundationPage.vue'
import ActivityPage from 'src/components/ActivityPage.vue'
import ReviewActivities from 'src/components/ReviewActivities.vue'

interface Query {
  tab: string
}
const route = useRoute()
const tab = ref((route.query as unknown as Query).tab || 'contents')
const splitterModel = ref(20)

const user = useUserStore()
const account = computed(() => user.account)
const reviewer = computed(() => user.reviewer)
const collection = useCollectionStore()
const foundation = useFoundationStore()
const creditsPerLinera = computed(() => collection.creditsPerLinera)
const spendableCredits = computed(() => user.spendable)
const lineraBalance = computed(() => foundation.userLineraBalance)
const avatarIds = computed(() => collection.avatars.get(account.value))
const avatar = computed(() => avatarIds.value ? collection.nftBannerByID(avatarIds.value[0], avatarIds.value[1]) : collection.nftBannerByID(1001, 1000))

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/ResPeer.png)
  border-radius: 50%
  background-size: cover
  background-repeat: no-repeat
</style>
