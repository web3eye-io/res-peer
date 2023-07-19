<template>
  <div class='text-h4' :style='{margin: "32px auto", textAlign: "center", fontWeight: 600}'>
    All Collections
  </div>
  <div class='nfts' :style='{width: "100%", display: "inline-block", overflowX: "scroll", padding: "16px 16px"}'>
    <div class='row justify-center q-gutter-sm no-wrap inline'>
      <q-card
        :style='{width: "400px", height: "520px"}'
        v-for='_collection in collections'
        :key='_collection.collectionId'
        class='cursor-pointer'
      >
        <q-img
          :src='collectionBanners.get(_collection.collectionId)'
          @error='onBannerError(_collection)'
        />
        <div :style='{padding: "16px 32px"}'>
          <div :style='{fontWeight: 600, fontSize: "16px", marginBottom: "8px"}' class='text-cyan-8'>
            {{ _collection.name }}
          </div>
          <div class='text-grey-6'>
            {{ new Date(_collection.createdAt / 1000).toDateString() }}
          </div>
          <div>
            <strong class='text-positive' :style='{fontWeight: 600, fontSize: "16px"}'>{{ _collection.price }}</strong> Linera
          </div>
        </div>
      </q-card>
    </div>
  </div>
</template>

<script setup lang='ts'>
import { Collection, useCollectionStore } from 'src/stores/collection'
import { computed, ref, watch } from 'vue'

const collection = useCollectionStore()
const collections = computed(() => Array.from(collection.collections.values()))
const collectionBanners = ref(new Map<number, string>())
const defaultBanner = ref('images/DefaultNFTBanner.png')

watch(collections, () => {
  collections.value.forEach((el) => {
    collectionBanners.value.set(el.collectionId, collectionBanner(el))
  })
})

const collectionBanner = (_collection: Collection) => {
  const nfts = collection.nftsByCollections(_collection.collectionId)
  if (nfts.length === 0) {
    return defaultBanner.value
  }
  return _collection.baseUri + nfts[0].uri
}

const onBannerError = (_collection: Collection) => {
  collectionBanners.value.set(_collection.collectionId, defaultBanner.value)
}

</script>

<style scoped lang='sass'>
/* width */
::-webkit-scrollbar
  height: 8px

/* Track */
::-webkit-scrollbar-track
  background: #f1f1f1

/* Handle */
::-webkit-scrollbar-thumb
  background: #888

/* Handle on hover */
::-webkit-scrollbar-thumb:hover
  background: #555
</style>
