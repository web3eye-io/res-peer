<template>
  <div
    v-for='(_content, index) in contents'
    :key='_content.cid'
    :style='{width: "100%", margin: "16px 0 16px 0"}'
    class='row'
  >
    <q-space />
    <div :style='{width: "720px", borderBottom: index < contents.length - 1 ? "1px solid gray" : "", paddingBottom: "48px"}'>
      <div :style='{fontWeight: "bold", fontSize: "28px", wordBreak: "break-word"}'>
        {{ _content.title?.length ? _content.title : 'You should have a title!' }}
      </div>
      <div>
        By
        <span class='text-grey-6 text-bold cursor-pointer'>
          {{ _content.author?.length ? _content.author : 'Anonymous' }}
        </span>
        <q-icon name='account_circle' size='20px' :style='{marginLeft: "8px"}' class='cursor-pointer' />
      </div>
      <div>
        At
        <span class='text-grey-6 text-bold'>{{ new Date(_content.createdAt / 1000) }}</span>
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
    <q-space />
  </div>
</template>

<script setup lang='ts'>
import { Content, useContentStore } from 'src/stores/content'
import { computed } from 'vue'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'

const content = useContentStore()
const contents = computed(() => Array.from(content.contents.values()).sort((a: Content, b: Content) => a.createdAt < b.createdAt ? 1 : -1))

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const onLikeClick = async (cid: string) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation Like ($cid: String!) {
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
    endpoint: 'feed'
  })
}

const onDislikeClick = async (cid: string) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation Dislike ($cid: String!) {
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
    endpoint: 'feed'
  })
}

</script>
