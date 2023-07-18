<template>
  <div
    v-for='[key, _content] in contents'
    :key='key'
    :style='{width: "100%", margin: "16px 0 16px 0"}'
    class='row'
  >
    <q-space />
    <div :style='{width: "540px"}'>
      <div :style='{fontWeight: "bold", fontSize: "28px"}'>
        {{ _content.title?.length ? _content.title : 'You should have a title!' }}
      </div>
      <div>
        By
        <span class='text-grey-6 text-bold'>
          {{ _content.author?.length ? _content.author : 'Anonymous' }}
        </span>
      </div>
      <div>
        Cid
        <span class='text-grey-6 text-bold'>
          {{ _content.cid }}
        </span>
      </div>
      <div
        :style='{margin: "24px 0 24px 0", fontSize: "16px"}'
        v-html='_content.content?.length ? _content.content : "You should have some content!"'
      />
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
