<template>
  <q-layout view='lHh Lpr lFf'>
    <q-header elevated>
      <q-toolbar>
        <div class='row' :style='{width: "720px", margin: "0 auto"}'>
          <q-img src='~assets/ResPeer.png' width='160px' fit='contain' class='cursor-pointer' />
          <q-space />
          <q-icon name='dashboard' size='24px' color='black' class='cursor-pointer' />
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>

    <q-footer elevated :style='{height: "32px", lineHeight: "32px"}'>
      <div class='row' :style='{width: "720px", margin: "0 auto"}'>
        <q-img src='~assets/ResPeer.png' width='80px' fit='contain' class='cursor-pointer' />
        <q-space />
        <span class='text-grey-6'>Peer-to-Peer content publishing application on Linera</span>
        <q-space />
        <q-img
          src='https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSAPxlNRQziXOi61fD4jtkxAm-v6pPbT_UIF5IL1_PqCQ&s=10'
          width='24px'
          height='24px'
          :style='{marginLeft: "8px", marginTop: "4px"}'
          class='cursor-pointer'
        />
      </div>
    </q-footer>
  </q-layout>
</template>

<script setup lang="ts"></script>

<style scoped lang="sass">
.q-layout__section--marginal
  background-color: white
</style>
