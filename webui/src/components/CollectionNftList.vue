<template>
  <q-table
    flat
    :rows='collections'
    :rows-per-page-options='[6]'
    :style='{width: "1080px", margin: "0px auto"}'
  >
    <template #header>
      <q-tr>
        <q-th class='text-grey-7' :style='{fontSize: "16px"}'>
          Rank
        </q-th>
        <q-th class='text-grey-7' :style='{fontSize: "16px"}'>
          Collection ID
        </q-th>
        <q-th class='text-grey-7' :style='{fontSize: "16px"}'>
          Collection
        </q-th>
        <Q-th />
        <q-th class='text-grey-7' :style='{fontSize: "16px"}'>
          Price
        </q-th>
        <q-th class='text-grey-7' :style='{fontSize: "16px"}'>
          URI
        </q-th>
      </q-tr>
    </template>
    <template #top-left>
      <div class='text-h4' :style='{margin: "32px auto"}'>
        Collection Gallery
      </div>
    </template>
    <template #body='props'>
      <q-tr :props='props' class='cursor-pointer' @click='onCollectionClick(props.row)'>
        <q-td class='text-center' :style='{fontWeight: 600, fontSize: "16px"}'>
          {{ props.rowIndex + 1 }}
        </q-td>
        <q-td class='text-center' :style='{fontWeight: 600, fontSize: "16px"}'>
          {{ props.row.collectionId }}
        </q-td>
        <q-td class='text-right'>
          <q-img
            :src='collectionBanner(props.row)'
            width='120px'
            height='130px'
            :style='{borderRadius: "8px"}'
          >
            <template #error>
              <div class='absolute-full flex flex-center error' />
            </template>
          </q-img>
        </q-td>
        <q-td :style='{fontWeight: 600, fontSize: "16px"}'>
          {{ props.row.name }}
        </q-td>
        <q-td class='text-center' :style='{fontWeight: 600, fontSize: "16px"}'>
          {{ props.row.price ? props.row.price + ' Linera' : 'Price in NFT' }}
        </q-td>
        <q-td class='text-center' :style='{fontWeight: 600, fontSize: "16px"}'>
          {{ props.row.baseUri }}
        </q-td>
      </q-tr>
    </template>
  </q-table>
</template>

<script setup lang='ts'>
import { Cookies } from 'quasar'
import { useCollectionStore, Collection } from 'src/stores/collection'
import { computed, ref, watch } from 'vue'
import { useRouter } from 'vue-router'

const collection = useCollectionStore()
const collections = computed(() => Array.from(collection.collections.values()).filter((el) => collection.nftsByCollectionID(el.collectionId).length > 0))
const collectionBanners = ref(new Map<number, string>())
const port = computed(() => Cookies.get('service-port'))

watch(collections, () => {
  collections.value.forEach((el) => {
    collectionBanners.value.set(el.collectionId, collectionBanner(el))
  })
})

const collectionBanner = (_collection: Collection) => {
  return collection.collectionBanner(_collection)
}

const router = useRouter()
const onCollectionClick = (_collection: Collection) => {
  void router.push({
    path: '/collection',
    query: {
      collectionId: _collection.collectionId,
      port: port.value
    }
  })
}

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/DefaultNFTBanner.png)
  border-radius: 8px
  background-size: cover
  background-repeat: no-repeat
</style>
