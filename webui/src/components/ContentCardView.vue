<template>
  <div :style='{width: "720px", paddingBottom: "48px"}'>
    <div class='cursor-pointer' :style='{fontWeight: "bold", fontSize: "26px", wordBreak: "break-word", marginBottom: "16px"}' @click='onTitleClick(_content.cid)'>
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
      <div class='row cursor-pointer' :style='{marginLeft: "16px"}' @click='onCommentClick(_content.cid)'>
        <q-icon name='comment' size='20px' :style='{marginRight: "6px"}' />
        {{ comments.length }}
      </div>
    </div>
    <div v-if='recommends.length' :style='{marginTop:"16px", padding:"8px", borderRadius:"8px"}' class='bg-grey-4'>
      <div v-for='recommend in recommends' :key='recommend.cid'>
        <div class='row'>
          <q-icon name='recommend' color='red' size='24px' />
          <div>
            <span :style='{color:"blue", marginLeft:"8px", lineHeight:"24px"}'>{{ recommend.author }}</span>
            <br>
            <span :style='{marginLeft:"8px", lineHeight:"24px"}'>At <span :style='{color:"grey"}'>{{ date.formatDate(recommend.createdAt / 1000) }}</span></span>
          </div>
        </div>
        <div :style='{marginLeft:"24px",marginTop:"8px"}'>
          {{ recommend.content }}
        </div>
        <div class='row' :style='{marginLeft:"24px",marginTop:"16px"}'>
          <div class='row cursor-pointer' @click='onLikeClick(recommend.cid)'>
            <q-icon name='thumb_up' size='20px' :style='{marginRight: "6px"}' />
            {{ recommend.likes }}
          </div>
          <div class='row cursor-pointer' :style='{marginLeft: "16px"}' @click='onDislikeClick(recommend.cid)'>
            <q-icon name='thumb_down' size='20px' :style='{marginRight: "6px"}' />
            {{ recommend.dislikes }}
          </div>
        </div>
      </div>
    </div>
    <div v-if='comments.length && expand' :style='{marginTop:"16px", padding:"8px", borderRadius:"8px"}' class='bg-grey-2'>
      <div v-for='(comment, index) in comments' :key='comment.cid' :style='{margin:"16px 0"}'>
        <div class='row'>
          <q-icon name='comment' color='blue' size='24px' />
          <div>
            <span :style='{color:"blue", marginLeft:"8px", lineHeight:"24px"}'>{{ comment.author }}</span>
            <br>
            <span :style='{marginLeft:"8px", lineHeight:"24px"}'>At <span :style='{color:"grey"}'>{{ date.formatDate(comment.createdAt / 1000) }}</span></span>
          </div>
        </div>
        <div :style='{marginLeft:"24px",marginTop:"8px"}'>
          {{ comment.content }}
        </div>
        <div class='row' :style='{marginLeft:"24px",marginTop:"16px"}'>
          <div class='row cursor-pointer' @click='onLikeClick(comment.cid)'>
            <q-icon name='thumb_up' size='20px' :style='{marginRight: "6px"}' />
            {{ comment.likes }}
          </div>
          <div class='row cursor-pointer' :style='{marginLeft: "16px"}' @click='onDislikeClick(comment.cid)'>
            <q-icon name='thumb_down' size='20px' :style='{marginRight: "6px"}' />
            {{ comment.dislikes }}
          </div>
        </div>
        <q-separator v-if='index < comments.length - 1' :style='{margin:"16px 0"}' />
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
import { Cookies, date } from 'quasar'
import { useRouter } from 'vue-router'

interface Props {
  cid: string
  expand?: boolean
  list?: boolean
}
const props = withDefaults(defineProps<Props>(), {
  list: true
})
const cid = toRef(props, 'cid')
const expand = toRef(props, 'expand')
const list = toRef(props, 'list')

const content = useContentStore()
const _content = computed(() => content.content(cid.value) as Content)
const comments = computed(() => content._comments(cid.value))
const recommends = computed(() => content._recommends(cid.value).slice(0, expand.value ? undefined : 1))
const collection = useCollectionStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const router = useRouter()
const port = computed(() => Cookies.get('service-port'))

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

const emit = defineEmits<{(evet: 'comment'): void}>()

const onCommentClick = (cid: string) => {
  if (list.value) {
    void router.push({
      path: '/content',
      query: {
        cid,
        port: port.value
      }
    })
    return
  }
  emit('comment')
}

const onTitleClick = (cid: string) => {
  void router.push({
    path: '/content',
    query: {
      cid,
      port: port.value
    }
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
