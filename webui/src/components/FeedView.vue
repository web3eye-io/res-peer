<template>
  <div
    v-for='[key, _content] in contents'
    :key='key'
  >
    <div>{{ _content.cid }}</div>
    <div>{{ _content.title }}</div>
    <div>{{ _content.content }}</div>
    <div>{{ _content.likes }}</div>
    <div>{{ _content.dislikes }}</div>
  </div>
</template>

<script setup lang='ts'>
import { useContentStore } from 'src/stores/content'
import { computed } from 'vue'

const content = useContentStore()
const contents = computed(() => content.contents)

</script>
