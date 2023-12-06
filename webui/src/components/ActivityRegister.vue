<template>
  <div :style='{margin:"64px 0",width:"960px"}'>
    <div v-if='activity.objectType(activityId) !== ObjectType.Content'>
      Coming soon...
    </div>
    <div v-else>
      <div v-if='contents.length'>
        <div
          v-for='(_content, i) in contents'
          :key='_content.cid'
          :style='{borderBottom:i < contents.length - 1 ? "1px solid grey" : "",marginBottom:"32px"}'
        >
          <div :style='{paddingBottom: "48px"}'>
            <div
              class='cursor-pointer'
              :style='{fontWeight: "bold", fontSize: "26px", wordBreak: "break-word", marginBottom: "4px"}'
              @click='onTitleClick(_content.cid)'
            >
              {{ _content.title?.length ? _content.title : 'You should have a title!' }}
            </div>
            <div :style='{marginBottom: "12px"}' class='text-grey-8'>
              {{ date.formatDate(_content.createdAt / 1000, 'YYYY-MM-DD HH:mm') }}
            </div>
            <div :style='{wordBreak:"break-word"}'>
              {{ _content.content }}
            </div>
            <q-btn
              flat
              dense
              class='text-blue-6'
              :style='{marginTop:"16px"}'
              @click='onRegisterClick(_content.cid)'
              :disable='objectRegistered(_content.cid)'
            >
              REGISTER TO <span class='text-green-8' :style='{marginLeft:"8px",fontSize:"12px"}'>{{ _activity?.title }}</span>
            </q-btn>
          </div>
        </div>
      </div>
      <div v-else class='text-center cursor-pointer text-grey-8'>
        Publish your article and register to <strong class='text-blue'>{{ _activity?.title }}</strong> now!
      </div>
    </div>
  </div>
</template>

<script lang='ts' setup>
import { Cookies, date } from 'quasar'
import { useActivityStore, ObjectType } from 'src/stores/activity'
import { useContentStore } from 'src/stores/content'
import { useUserStore } from 'src/stores/user'
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { targetChain } from 'src/stores/chain'

interface Query {
  activityId: number
}
const route = useRoute()
const activityId = ref((route.query as unknown as Query).activityId)
const activity = useActivityStore()
const _activity = computed(() => activity.activity(Number(activityId.value)))

const user = useUserStore()
const account = computed(() => user.account)

const content = useContentStore()
const contents = computed(() => content._contents(account.value))

const router = useRouter()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const onTitleClick = (cid: string) => {
  void router.push({
    path: '/content',
    query: {
      cid,
      port: Cookies.get('service-port')
    }
  })
}

const objectRegistered = (cid: string) => {
  return activity.objectRegistered(Number(activityId.value), cid)
}

const onRegisterClick = async (cid: string) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation register ($activityId: Int!, $objectId: String!) {
      register(activityId: $activityId, objectId: $objectId)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    activityId: parseInt(`${activityId.value}`),
    objectId: cid,
    endpoint: 'activity',
    chainId: targetChain.value
  })
}

</script>
