<template>
  <div class='row'>
    <q-space />
    <q-btn
      dense flat v-if='!editing' label='Create'
      color='blue'
      @click='editing = !editing'
    />
  </div>
  <div class='row'>
    <q-space />
    <q-btn
      dense flat v-if='editing' label='Publish'
      color='blue'
      @click='onPublishClick'
    />
  </div>
  <q-input v-if='editing' dense label='Title' v-model='title' />
  <q-input
    v-if='editing' v-model='content' type='textarea' filled
    :style='{marginTop: "16px"}'
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'

const title = ref('')
const content = ref('')
const editing = ref(false)

const onPublishClick = () => {
  editing.value = !editing.value
}

</script>
