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
            name='dashboard' size='24px' color='black' class='cursor-pointer'
            @click='onDashboardClick'
          />
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
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
  </q-layout>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const onGithubClick = (uri: string) => {
  window.open(uri)
}
const onDashboardClick = () => {
  void router.push({ path: '/dashboard' })
}
const onLogoClick = () => {
  void router.push({ path: '/' })
}
</script>

<style scoped lang="sass">
.q-layout__section--marginal
  background-color: white
</style>
