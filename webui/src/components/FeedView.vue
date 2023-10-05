<template>
  <div
    v-for='(_content, index) in contents'
    :key='_content.cid'
    :style='{width: "100%", margin: "16px 0 16px 0"}'
    class='row'
  >
    <q-space />
    <div :style='{borderBottom: index < contents.length - 1 ? "1px solid grey" : ""}'>
      <content-card-view :cid='_content.cid' />
    </div>
    <q-space />
  </div>
</template>

<script setup lang='ts'>
import { useContentStore } from 'src/stores/content'
import { computed } from 'vue'

import ContentCardView from './ContentCardView.vue'

const content = useContentStore()
const contents = computed(() => content._contents())

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/ResPeer.png)
  border-radius: 50%
  background-size: cover
  background-repeat: no-repeat
</style>
