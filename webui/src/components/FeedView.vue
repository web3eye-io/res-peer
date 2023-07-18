<template>
  <div
    v-for='[key, _content] in contents'
    :key='key'
    :style='{width: "100%", margin: "16px 0 16px 0"}'
    class='row'
  >
    <q-space />
    <div :style='{width: "540px"}'>
      <div>{{ _content.cid }}</div>
      <div>{{ _content.title }}</div>
      <div v-html='_content.content' />
      <div>{{ _content.likes }}</div>
      <div>{{ _content.dislikes }}</div>
    </div>
    <q-space />
  </div>
</template>

<script setup lang='ts'>
import { useContentStore } from 'src/stores/content'
import { computed } from 'vue'

const content = useContentStore()
const contents = computed(() => content.contents)

</script>
