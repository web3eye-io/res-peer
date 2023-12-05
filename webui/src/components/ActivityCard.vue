<template>
  <div>
    <div
      class='cursor-pointer'
      :style='{fontWeight: "bold", fontSize: "26px", wordBreak: "break-word", marginBottom: "8px"}'
      @click='onTitleClick(activity.id)'
    >
      {{ activity.title?.length ? activity.title : 'You should have a title!' }}
    </div>
    <div v-if='activity.slogan?.length' class='text-blue-7' :style='{fontSize:"16px", marginBottom:"16px"}'>
      {{ activity.slogan }}
    </div>
    <div class='row'>
      <div>
        <div>
          Hosted By
          <span class='text-grey-6 text-bold cursor-pointer'>
            {{ activity.host?.length ? activity.host : 'Anonymous' }}
          </span>
        </div>
        <div>
          Created At
          <span class='text-grey-6 text-bold'>{{ date.formatDate(activity.createdAt / 1000) }}</span>
        </div>
      </div>
      <q-avatar :style='{marginLeft: "4px", marginTop: "-12px"}'>
        <q-img
          :src='userAvatar(activity.host) ? userAvatar(activity.host) : "~/assets/ResPeer.png"'
          width='32px'
          height='32px'
          fit='cover'
          :style='{borderRadius: "50%"}'
        >
          <template #error>
            <div class='absolute-full flex flex-center error' />
          </template>
        </q-img>
      </q-avatar>
    </div>
    <q-img v-if='activity.banner?.length' :src='activity.banner' :style='{marginTop:"32px"}'>
      <template #error>
        <div class='absolute-full flex flex-center error error-normal' />
      </template>
    </q-img>
    <div
      class='text-grey-7'
      :style='{margin: "24px 0 24px 0", fontSize: "16px", wordBreak: "break-word"}'
      v-html='activity.introduction?.length ? activity.introduction : "You should have some introduction!"'
    />
  </div>
</template>

<script lang='ts' setup>
import { defineProps, toRef } from 'vue'
import { date } from 'quasar'
import { useCollectionStore } from 'src/stores/collection'

import { Activity } from 'src/stores/activity'

const collection = useCollectionStore()

interface Props {
  activity: Activity
}

const props = defineProps<Props>()
const activity = toRef(props, 'activity')

const onTitleClick = (id: number) => {
  console.log(id)
}

const userAvatar = (account: string) => {
  const ids = collection.avatars.get(account)
  if (!ids) {
    return collection.nftBannerByID(1001, 1000)
  }
  return collection.nftBannerByID(ids[0], ids[1])
}

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/ResPeer.png)
  border-radius: 50%
  background-size: cover
  background-repeat: no-repeat

.error-normal
  border-radius: 0
</style>
