<template>
  <div :style='{margin: "64px auto"}'>
    <collection-gallery />
    <collection-nft-list :style='{marginTop: "32px"}' />
  </div>
</template>

<script setup lang='ts'>
import CollectionGallery from 'src/components/CollectionGallery.vue'
import CollectionNftList from 'src/components/CollectionNftList.vue'
</script>
