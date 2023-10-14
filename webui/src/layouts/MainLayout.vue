<template>
  <q-layout view='lHh Lpr lFf'>
    <q-header elevated>
      <q-toolbar>
        <div class='row' :style='{width: "720px", margin: "0 auto"}'>
          <q-img
            src='~assets/ResPeer.png' width='160px' fit='contain' class='cursor-pointer'
            @click='onLogoClick'
          />
          <q-space />
          <q-icon
            name='store' size='24px' color='black' class='cursor-pointer'
            @click='onNFTMarketClick'
          />
          <q-icon
            v-if='!account?.length'
            name='login' size='24px' color='black' class='cursor-pointer'
            :style='{marginLeft: "8px"}'
            @click='onLoginClick'
          />
          <q-icon
            v-if='account?.length'
            name='dashboard' size='24px' color='black' class='cursor-pointer'
            :style='{marginLeft: "8px"}'
            @click='onDashboardClick'
          />
          <q-icon
            v-if='account?.length'
            name='logout' size='24px' color='black' class='cursor-pointer'
            :style='{marginLeft: "8px"}'
            @click='onLogoutClick'
          />
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
      <chain-query />
      <request-application />
      <credit-query />
      <block-subscription />
      <feed-contents-keys-query />
      <feed-contents-query />
      <market-info-query />
      <market-collections-query />
      <request-feed-subscribe />
      <request-review-subscribe />
      <request-credit-subscribe />
      <request-foundation-subscribe />
      <request-market-subscribe />
      <user-reviewer-query />
      <content-applications-keys-query />
      <content-applications-query />
      <asset-applications-keys-query />
      <asset-applications-query />
      <reviewer-applications-keys-query />
      <reviewer-applications-query />
      <review-state-query />
      <foundation-info-query />
    </q-page-container>

    <q-footer elevated :style='{height: "32px", lineHeight: "32px"}'>
      <div class='row' :style='{width: "720px", margin: "0 auto"}'>
        <q-img
          src='~assets/ResPeer.png' width='80px' fit='contain' class='cursor-pointer'
          @click='onLogoClick'
        />
        <q-space />
        <span class='text-grey-6'>Peer-to-Peer content publishing application on Linera</span>
        <q-space />
        <q-img
          src='https://avatars.githubusercontent.com/u/107513858?s=48&v=4'
          width='24px'
          height='24px'
          :style='{marginLeft: "8px", marginTop: "4px"}'
          class='cursor-pointer'
          @click='onGithubClick("https://github.com/linera-io/linera-protocol.git")'
        />
        <q-img
          src='https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSAPxlNRQziXOi61fD4jtkxAm-v6pPbT_UIF5IL1_PqCQ&s=10'
          width='24px'
          height='24px'
          :style='{marginLeft: "8px", marginTop: "4px"}'
          class='cursor-pointer'

          @click='onGithubClick("https://github.com/web3eye-io/res-peer.git")'
        />
      </div>
    </q-footer>
    <q-dialog
      v-model='logining'
      @hide='onHide'
      position='standard'
    >
      <q-card :style='{padding: "32px", width: "100%"}'>
        <q-card-section>
          <div class='text-h5'>
            Login
          </div>
        </q-card-section>
        <q-card-section>
          <q-input :style='{width: "100%"}' label='Linera Address' v-model='account' />
        </q-card-section>
        <q-card-section>
          <q-btn label='Login' @click='onLoginConfirmClick' />
        </q-card-section>
      </q-card>
    </q-dialog>
  </q-layout>
</template>

<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { onBeforeMount, onMounted, ref } from 'vue'
import { Cookies } from 'quasar'
import { useUserStore } from 'src/stores/user'
import * as constants from 'src/const'

import CreditQuery from 'src/components/CreditQuery.vue'
import BlockSubscription from 'src/components/BlockSubscription.vue'
import FeedContentsKeysQuery from 'src/components/FeedContentsKeysQuery.vue'
import FeedContentsQuery from 'src/components/FeedContentsQuery.vue'
import MarketInfoQuery from 'src/components/MarketInfoQuery.vue'
import MarketCollectionsQuery from 'src/components/MarketCollectionsQuery.vue'
import RequestApplication from 'src/components/RequestApplication.vue'
import RequestFeedSubscribe from 'src/components/RequestFeedSubscribe.vue'
import RequestCreditSubscribe from 'src/components/RequestCreditSubscribe.vue'
import RequestFoundationSubscribe from 'src/components/RequestFoundationSubscribe.vue'
import RequestMarketSubscribe from 'src/components/RequestMarketSubscribe.vue'
import ChainQuery from 'src/components/ChainQuery.vue'
import UserReviewerQuery from 'src/components/UserReviewerQuery.vue'
import RequestReviewSubscribe from 'src/components/RequestReviewSubscribe.vue'
import ContentApplicationsKeysQuery from 'src/components/ContentApplicationsKeysQuery.vue'
import ContentApplicationsQuery from 'src/components/ContentApplicationsQuery.vue'
import AssetApplicationsKeysQuery from 'src/components/AssetApplicationsKeysQuery.vue'
import AssetApplicationsQuery from 'src/components/AssetApplicationsQuery.vue'
import ReviewerApplicationsKeysQuery from 'src/components/ReviewerApplicationsKeysQuery.vue'
import ReviewerApplicationsQuery from 'src/components/ReviewerApplicationsQuery.vue'
import ReviewStateQuery from 'src/components/ReviewStateQuery.vue'
import FoundationInfoQuery from 'src/components/FoundationInfoQuery.vue'

const router = useRouter()
const account = ref('')
const logining = ref(false)
const user = useUserStore()
const route = useRoute()

interface Query {
  port: number
}

const port = ref((route.query as unknown as Query).port || constants.port)

const onGithubClick = (uri: string) => {
  window.open(uri)
}
const onDashboardClick = () => {
  void router.push({
    path: '/dashboard',
    query: {
      port: port.value
    }
  })
}
const onLogoClick = () => {
  void router.push({
    path: '/',
    query: {
      port: port.value
    }
  })
}
const onLoginClick = () => {
  logining.value = true
}
const onHide = () => {
  logining.value = false
}
const onLoginConfirmClick = () => {
  if (account.value.length === 0) {
    return
  }
  onHide()
  Cookies.set('account', account.value)
  user.account = account.value
}
onBeforeMount(() => {
  Cookies.set('service-port', port.value.toString())
})
onMounted(() => {
  account.value = Cookies.get('account')
  user.account = account.value
})

const onLogoutClick = () => {
  Cookies.remove('account')
  user.account = undefined as unknown as string
  account.value = undefined as unknown as string
}
const onNFTMarketClick = () => {
  void router.push({
    path: '/market',
    query: {
      port: port.value
    }
  })
}
</script>

<style scoped lang="sass">
.q-layout__section--marginal
  background-color: white
</style>
