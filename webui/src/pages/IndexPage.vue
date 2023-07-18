<template>
  <q-page class='row items-center justify-evenly'>
    <credit-query />
    <block-subscription />
    <feed-contents-keys-query />
    <feed-contents-query />
    <feed-view />
  </q-page>
</template>

<script setup lang="ts">
import CreditQuery from 'src/components/CreditQuery.vue'
import BlockSubscription from 'src/components/BlockSubscription.vue'
import FeedContentsKeysQuery from 'src/components/FeedContentsKeysQuery.vue'
import FeedContentsQuery from 'src/components/FeedContentsQuery.vue'
import FeedView from 'src/components/FeedView.vue'
</script>
