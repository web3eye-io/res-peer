<template>
  <div :style='{width: "1080px", margin: "32px auto"}'>
    <div class='row'>
      <q-space />
      Account: {{ account }}
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
          </q-tab-panel>
        </q-tab-panels>
      </template>
    </q-splitter>
  </div>
</template>

<script setup lang='ts'>
import { computed, ref } from 'vue'
import { useUserStore } from 'src/stores/user'

import CreateContent from 'src/components/CreateContent.vue'
import UserBalance from 'src/components/UserBalance.vue'
import ArticleList from 'src/components/ArticleList.vue'
import CreateCollection from 'src/components/CreateCollection.vue'

const user = useUserStore()
const account = computed(() => user.account)

const splitterModel = ref(20)
const tab = ref('contents')

</script>
