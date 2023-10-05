<template>
  <div :style='{width: "720px", paddingBottom: "48px"}'>
    <div :style='{fontWeight: "bold", fontSize: "28px", wordBreak: "break-word", marginBottom: "16px"}'>
      {{ _content.title?.length ? _content.title : 'You should have a title!' }}
    </div>
    <div>
      By
      <span class='text-grey-6 text-bold cursor-pointer'>
        {{ _content.author?.length ? _content.author : 'Anonymous' }}
      </span>
      <q-avatar :style='{marginLeft: "8px"}'>
        <q-img
          :src='userAvatar(_content.author) ? userAvatar(_content.author) : "~/assets/ResPeer.png"'
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
    <div>
      At
      <span class='text-grey-6 text-bold'>{{ date.formatDate(_content.createdAt / 1000) }}</span>
    </div>
    <div>
      Cid
      <span class='text-grey-6 text-bold cursor-pointer'>
        {{ _content.cid }}
      </span>
    </div>
    <div
      :style='{margin: "24px 0 24px 0", fontSize: "16px", wordBreak: "break-word"}'
      v-html='_content.content?.length ? _content.content : "You should have some content!"'
    />
    <div class='row'>
      <div class='row cursor-pointer' @click='onLikeClick(_content.cid)'>
        <q-icon name='thumb_up' size='20px' :style='{marginRight: "6px"}' />
        {{ _content.likes }}
      </div>
      <div class='row cursor-pointer' :style='{marginLeft: "16px"}' @click='onDislikeClick(_content.cid)'>
        <q-icon name='thumb_down' size='20px' :style='{marginRight: "6px"}' />
        {{ _content.dislikes }}
      </div>
    </div>
  </div>
</template>

<script setup lang='ts'>
import { useContentStore, Content } from 'src/stores/content'
import { computed, onMounted, toRef, watch } from 'vue'
import { provideApolloClient, useMutation, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useCollectionStore } from 'src/stores/collection'
import { targetChain } from 'src/stores/chain'
import { date } from 'quasar'

interface Props {
  cid: string
}
const props = defineProps<Props>()
const cid = toRef(props, 'cid')

const content = useContentStore()
const _content = computed(() => content.content(cid.value) as Content)
const collection = useCollectionStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const userAvatar = (account: string) => {
  const ids = collection.avatars.get(account)
  if (!ids) {
    return collection.nftBannerByID(1001, 1000)
  }
  return collection.nftBannerByID(ids[0], ids[1])
}

const ready = () => {
  return targetChain.value?.length && _content.value
}

const getContentAvatar = () => {
  const account = _content.value?.author
  if (collection.avatars.get(account)) {
    return
  }

  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getMarketInfo($account: String!) {
        avatars(owner: $account)
      }
    `, {
    account: `${account}`,
    endpoint: 'market',
    chainId: targetChain.value
  }))

  watch(result, () => {
    const res = result.value as Record<string, Array<number>>
    collection.avatars.set(account, res.avatars)
  })
}

watch(targetChain, () => {
  if (!ready()) return
  getContentAvatar()
})

onMounted(() => {
  if (!ready()) return
  getContentAvatar()
})

const onLikeClick = async (cid: string) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation like ($cid: String!) {
      like(ccid: $cid)
    }
  `))
  onDone(() => {
    content.mutateKeys.push(cid)
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    cid,
    endpoint: 'feed',
    chainId: targetChain.value
  })
}

const onDislikeClick = async (cid: string) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation dislike ($cid: String!) {
      dislike(ccid: $cid)
    }
  `))
  onDone(() => {
    content.mutateKeys.push(cid)
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    cid,
    endpoint: 'feed',
    chainId: targetChain.value
  })
}

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/ResPeer.png)
  border-radius: 50%
  background-size: cover
  background-repeat: no-repeat
</style>
