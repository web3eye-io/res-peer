<template>
  <q-page class='row items-center justify-evenly'>
    <div :style='{margin: "32px auto 64px auto"}'>
      <feed-view />
    </div>
  </q-page>
</template>

<script setup lang="ts">
import FeedView from 'src/components/FeedView.vue'
</script>
