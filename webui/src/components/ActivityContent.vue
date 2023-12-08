<template>
  <div :style='{width:"960px", margin:"32px 0 0 0"}'>
    <div
      class='row'
      :style='{fontWeight: "bold", fontSize: "26px", wordBreak: "break-word", marginBottom: "8px"}'
    >
      {{ activity?.title || 'You should have a title!' }}
      <q-btn
        v-if='voteEnd() && activity?.host === account'
        :style='{fontSize:"12px",marginTop:"10px",marginLeft:"10px"}'
        dense
        flat
        label='Finalize'
        color='blue-7'
        @click='onFinalizeClick'
      />
    </div>
    <div v-if='activity?.slogan?.length' class='text-blue-7' :style='{fontSize:"16px", marginBottom:"16px"}'>
      {{ activity.slogan }}
    </div>
    <div class='row'>
      <div>
        <div class='row'>
          <div>
            Hosted By
            <span class='text-grey-6 text-bold cursor-pointer'>
              {{ activity?.host || 'Anonymous' }}
            </span>
          </div>
          <a :href='activity?.hostResume' :style='{marginLeft:"8px"}'>Resume</a>
        </div>
        <div>
          Created At
          <span class='text-grey-6 text-bold'>{{ date.formatDate(activity?.createdAt || 0 / 1000) }}</span>
        </div>
      </div>
      <q-avatar :style='{marginLeft: "4px", marginTop: "-12px"}'>
        <q-img
          :src='userAvatar(activity?.host as string) || "~/assets/ResPeer.png"'
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
    <div class='row'>
      <div :style='{width:"50%"}'>
        <q-separator :style='{margin:"32px 0 0 0"}' />
        <q-splitter v-model='splitter' unit='px'>
          <template #before>
            <div :style='{margin:"4px 8px 4px 4px"}'>
              Register
            </div>
            <q-separator />
            <div v-if='activity?.votable' :style='{margin:"4px 8px 4px 4px"}'>
              Vote
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}'>
              Winner Voter Reward Percent
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}'>
              Progress
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}'>
              Join Type
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}'>
              Object Type
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}'>
              Vote Type
            </div>
          </template>
          <template #after>
            <div class='row'>
              <div :style='{margin:"4px 8px 4px 4px"}' class='text-bold text-green-6'>
                {{ date.formatDate(activity?.registerStartAt, 'YYYY-MM-DD') }} ~ {{ date.formatDate(activity?.registerEndAt, 'YYYY-MM-DD') }}
              </div>
              <q-btn
                :style='{fontSize:"12px"}'
                dense
                flat
                label='Register'
                :disable='!registerable()'
                color='blue-7'
                @click='onRegisterClick'
              />
            </div>
            <q-separator />
            <div v-if='activity?.votable' class='row'>
              <div :style='{margin:"4px 8px 4px 4px"}' class='text-bold text-green-6'>
                {{ date.formatDate(activity?.voteStartAt || 0, 'YYYY-MM-DD') }} ~ {{ date.formatDate(activity?.voteEndAt || 0, 'YYYY-MM-DD') }}
              </div>
              <q-btn
                :style='{fontSize:"12px"}'
                dense
                flat
                label='Vote'
                :disable='!votable()'
                color='blue-7'
                @click='onVoteClick'
              />
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}' class='text-bold text-grey-6'>
              {{ activity?.voterRewardPercent }} %
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}' class='text-bold text-grey-6'>
              {{ activity?.finalized ? 'Finalized' : 'In Progress' }}
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}' class='text-bold text-blue-6'>
              {{ activity?.joinType }}
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}' class='text-bold text-blue-6'>
              {{ activity?.objectType }}
            </div>
            <q-separator />
            <div :style='{margin:"4px 8px 4px 4px"}' class='text-bold text-blue-6'>
              {{ activity?.voteType }}
            </div>
          </template>
        </q-splitter>
        <q-separator :style='{margin:"0 0 32px 0"}' />
        <table>
          <tr>
            <th>Place</th>
            <th>Medal</th>
            <th>Title</th>
            <th>Reward Amount (Linera)</th>
          </tr>
          <tr v-for='prize in activity?.prizeConfigs' :key='prize.place'>
            <td class='text-center'>
              {{ prize.place }}
            </td>
            <td>
              <q-img :src='prize.medal' width='64px' :style='{borderRadius:"50%"}' />
            </td>
            <td>{{ prize.title }}</td>
            <td>{{ prize.rewardAmount }}</td>
          </tr>
        </table>
      </div>
      <q-img v-if='activity?.banner?.length' :src='activity?.banner' :style='{marginTop:"32px",marginLeft:"3%"}' width='47%'>
        <template #error>
          <div class='absolute-full flex flex-center error error-normal' />
        </template>
      </q-img>
    </div>
    <div
      class='text-grey-10'
      :style='{margin: "24px 0 24px 0", fontSize: "16px", wordBreak: "break-word"}'
      v-html='activity?.introduction || "You should have some introduction!"'
    />
    <div class='row'>
      <div class='row'>
        <q-icon name='app_registration' color='green' size='24px' />
        <span class='text-bold text-grey-7' :style='{fontSize:"18px"}'>{{ _activity.objectCandidateCount(activity?.id || -1) }}</span>
      </div>
      <div class='row' :style='{marginLeft:"8px"}'>
        <q-icon name='where_to_vote' color='blue' size='24px' />
        <span class='text-bold text-grey-7' :style='{fontSize:"18px"}'>{{ _activity.votes(activity?.id || -1) }}</span>
      </div>
      <div class='row' :style='{marginLeft:"8px"}'>
        <q-icon name='pin_drop' color='blue' size='24px' />
        <span v-if='activity?.joinType === JoinType.InPerson.toUpperCase()' class='text-grey-7' :style='{fontSize:"16px"}'>{{ activity?.location }}</span>
        <a v-if='activity?.joinType === JoinType.Online.toUpperCase()' class='text-grey-7' :style='{fontSize:"16px"}' :href='activity?.location'>{{ activity?.location }}</a>
      </div>
    </div>
    <div :style='{marginTop:"-32px"}'>
      <activity-vote :activity-index='activityId' />
    </div>
  </div>
</template>

<script lang='ts' setup>
import { computed, ref } from 'vue'
import { Cookies, date } from 'quasar'
import { useCollectionStore } from 'src/stores/collection'
import { useActivityStore, JoinType } from 'src/stores/activity'
import { useRoute, useRouter } from 'vue-router'
import { useUserStore } from 'src/stores/user'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'

import ActivityVote from './ActivityVote.vue'
import { targetChain } from 'src/stores/chain'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const collection = useCollectionStore()
const splitter = ref(200)

interface Query {
  activityId: number
}

const route = useRoute()
const activityId = ref((route.query as unknown as Query).activityId)
const _activity = useActivityStore()
const activity = computed(() => _activity.activity(Number(activityId.value)))

const user = useUserStore()
const account = computed(() => user.account)

const userAvatar = (account: string) => {
  const ids = collection.avatars.get(account)
  if (!ids) {
    return collection.nftBannerByID(1001, 1000)
  }
  return collection.nftBannerByID(ids[0], ids[1])
}

const registerable = () => {
  const now = Date.now()
  return (activity.value?.registerStartAt || 0) <= now && (activity.value?.registerEndAt || 0) > now
}

const votable = () => {
  const now = Date.now()
  return (activity.value?.voteStartAt || 0) <= now &&
         (activity.value?.voteEndAt || 0) > now &&
        _activity.objectCandidateCount(activity.value?.id || -1)
}

const voteEnd = () => {
  return (activity.value?.voteEndAt || 0) < Date.now() && !activity.value?.finalized
}

const router = useRouter()
const onRegisterClick = () => {
  void router.push({
    path: '/activity/register',
    query: {
      port: Cookies.get('service-port'),
      activityId: activity.value?.id
    }
  })
}

const onVoteClick = () => {
  void router.push({
    path: '/activity/vote',
    query: {
      port: Cookies.get('service-port'),
      activityId: activity.value?.id
    }
  })
}

const onFinalizeClick = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation finalize($activityId: Int!) {
      finalize(activityId: $activityId)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    activityId: parseInt(activityId.value.toString()),
    endpoint: 'activity',
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

.error-normal
  border-radius: 0

table
  font-family: arial, sans-serif
  border-collapse: collapse
  width: 100%

td, th
  border: 1px solid #dddddd
  text-align: left
  padding: 8px

tr:nth-child(even)
  background-color: #eeeeee
</style>
