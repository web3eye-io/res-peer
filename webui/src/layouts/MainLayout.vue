<template>
  <q-layout view='lHh Lpr lFf'>
    <q-header elevated>
      <q-toolbar>
        <q-img src='~assets/ResPeer.png' />
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang='ts'></script>

<style scoped lang='sass'>
.q-layout__section--marginal
  background-color: white
</style>
