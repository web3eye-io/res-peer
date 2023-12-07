<template>
  <div :style='{margin:"64px 0",width:"960px"}'>
    <div v-if='activity.objectType(activityId) !== ObjectType.Content'>
      Coming soon...
    </div>
    <div v-else>
      <div v-if='contents.length'>
        <q-table
          flat
          :rows='contents'
          :columns='(columns as any)'
        >
          <template #top-left>
            <div class='row'>
              <div :style='{fontSize:"18px",marginTop:"4px"}' class='text-bold text-grey-9'>
                {{ _activity?.title }}
              </div>
              <div :style='{fontSize:"22px",marginLeft:"16px"}' class='text-bold text-green-9'>
                LeaderBoard
              </div>
            </div>
          </template>
          <template #header='props'>
            <q-tr :props='props'>
              <q-th>Medal</q-th>
              <q-th>Place</q-th>
              <q-th>Title</q-th>
              <q-th>Vote Power</q-th>
              <q-th>Votes</q-th>
              <q-th />
            </q-tr>
          </template>
          <template #body='props'>
            <q-tr :props='props' class='cursor-pointer'>
              <q-td key='Medal' :props='props' @click='onContentClick(props.row.cid)'>
                <q-icon name='workspace_premium' :class='placeColor(props.row.cid)' size='32px' />
                <q-icon v-if='activity.objectWon(Number(activityId), props.row.cid)' name='emoji_events' :class='placeColor(props.row.cid)' size='32px' />
                <q-icon v-if='activity.objectVoteCount(Number(activityId), props.row.cid) || true' name='where_to_vote' color='blue' size='32px' />
              </q-td>
              <q-td key='Place' :props='props' @click='onContentClick(props.row.cid)'>
                {{ objectPlace(props.row.cid) }}
              </q-td>
              <q-td key='Title' :props='props' @click='onContentClick(props.row.cid)'>
                {{ props.row.title }}
              </q-td>
              <q-td key='Vote Power' :props='props' @click='onContentClick(props.row.cid)'>
                {{ activity.objectVotePower(Number(activityId), props.row.cid) }}
              </q-td>
              <q-td key='Votes' :props='props' @click='onContentClick(props.row.cid)'>
                {{ activity.objectVoteCount(Number(activityId), props.row.cid) }}
              </q-td>
              <q-td key='Vote' :props='props'>
                <q-btn
                  flat
                  dense
                  class='text-blue-6'
                  @click='onVoteClick(props.row.cid)'
                  :disable='objectVoted(props.row.cid)'
                >
                  VOTE
                </q-btn>
              </q-td>
            </q-tr>
          </template>
        </q-table>
      </div>
      <div v-else class='text-center cursor-pointer text-grey-8'>
        Publish your article and register to <strong class='text-blue'>{{ _activity?.title }}</strong> now!
      </div>
    </div>
  </div>
</template>

<script lang='ts' setup>
import { Cookies } from 'quasar'
import { useActivityStore, ObjectType } from 'src/stores/activity'
import { useContentStore } from 'src/stores/content'
import { useUserStore } from 'src/stores/user'
import { computed, ref, toRef } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { targetChain } from 'src/stores/chain'
import { Content } from 'src/stores/review'

interface Query {
  activityId: number
}

interface Props {
  activityIndex?: number
}

const _props = defineProps<Props>()
const route = useRoute()

const activityId = ref((route.query as unknown as Query).activityId || toRef(_props, 'activityIndex').value as number)

const activity = useActivityStore()
const _activity = computed(() => activity.activity(Number(activityId.value)))
const objectCandidates = computed(() => activity.objectCandidates(Number(activityId.value)))

const user = useUserStore()
const account = computed(() => user.account)

const content = useContentStore()
const contents = computed(() => content._contents().filter((el) => objectCandidates.value.includes(el.cid)).sort((a, b) => {
  return activity.objectVotePower(activityId.value, b.cid) - activity.objectVotePower(activityId.value, a.cid)
}))

const objectPlace = (objectId: string) => {
  return contents.value.findIndex((el) => el.cid === objectId) + 1
}

const placeColor = (objectId: string) => {
  if (objectPlace(objectId) === 1) {
    return 'text-amber-9'
  }
  if (objectPlace(objectId) === 2) {
    return 'text-green-9'
  }
  if (objectPlace(objectId) === 3) {
    return 'text-purple-9'
  }
  return 'text-grey-9'
}

const router = useRouter()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const objectVoted = (cid: string) => {
  return activity.objectVoted(Number(activityId.value), cid, account.value)
}

const onVoteClick = async (cid: string) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation vote ($activityId: Int!, $objectId: String!) {
      vote(activityId: $activityId, objectId: $objectId)
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

const columns = ref([
  {
    name: 'Medal',
    label: 'Medal',
    align: 'center',
    field: 1,
    sortable: true
  }, {
    name: 'Place',
    label: 'Place',
    align: 'center',
    field: (row: Content) => objectPlace(row.cid),
    sortable: true
  }, {
    name: 'Title',
    label: 'Title',
    align: 'left',
    field: (row: Content) => row.title,
    sortable: true
  }, {
    name: 'Vote Power',
    label: 'Vote Power',
    align: 'center',
    field: (row: Content) => activity.objectVotePower(Number(activityId.value), row.cid),
    sortable: true
  }, {
    name: 'Votes',
    label: 'Votes',
    align: 'center',
    field: (row: Content) => activity.objectVoteCount(Number(activityId.value), row.cid),
    sortable: true
  }, {
    name: 'Vote',
    label: 'Vote',
    align: 'center',
    field: 2,
    sortable: true
  }
])

const onContentClick = (cid: string) => {
  void router.push({
    path: '/content',
    query: {
      cid,
      port: Cookies.get('service-port')
    }
  })
}

</script>
