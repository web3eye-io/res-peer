<template>
  <div class='text-h4' :style='{margin: "32px auto", textAlign: "center", fontWeight: 600}'>
    All Collections
  </div>
  <div class='nfts' :style='{width: "1080px", overflowX: "scroll", padding: "32px 16px 48px 16px", margin: "0 auto"}'>
    <div class='row justify-center q-gutter-sm no-wrap inline'>
      <q-card
        :style='{width: "400px", height: "520px", borderRadius: "16px"}'
        v-for='_collection in collections'
        :key='_collection.collectionId'
        class='cursor-pointer'
      >
        <q-img
          :src='collectionBanner(_collection)'
          width='100%'
          height='400px'
        >
          <template #error>
            <div class='absolute-full flex flex-center error' />
          </template>
        </q-img>
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
import { computed } from 'vue'

const collection = useCollectionStore()
const collections = computed(() => Array.from(collection.collections.values()))

const collectionBanner = (_collection: Collection) => {
  return collection.collectionBanner(_collection)
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

.error
  background-image: url(../assets/DefaultNFTBanner.png)
  border-radius: 8px
  background-size: cover
  background-repeat: no-repeat
</style>
