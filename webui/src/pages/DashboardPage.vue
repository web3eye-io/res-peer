<template>
  <div :style='{width: "1080px", margin: "32px auto"}'>
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
          </q-tab-panel>
          <q-tab-panel name='credits'>
            <user-balance />
          </q-tab-panel>
          <q-tab-panel name='assets'>
            <div>
              TODO
            </div>
          </q-tab-panel>
        </q-tab-panels>
      </template>
    </q-splitter>
  </div>
</template>

<script setup lang='ts'>
import { ref } from 'vue'

import CreateContent from 'src/components/CreateContent.vue'
import UserBalance from 'src/components/UserBalance.vue'

const splitterModel = ref(20)
const tab = ref('contents')

</script>
