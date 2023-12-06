<template>
  <div :style='{width: "1440px", margin: "32px auto"}'>
    <h4>Create Activity</h4>
    <q-input label='Title' v-model='params.title' />
    <q-input label='Slogan' v-model='params.slogan' />
    <q-input label='Banner' v-model='params.banner' />
    <q-input label='Host resume' v-model='params.hostResume' />
    <h6>Posters</h6>
    <ul>
      <li v-for='poster in params.posters' :key='poster'>
        {{ poster }}
      </li>
    </ul>
    <div class='row' v-if='addPoster' :style='{marginBottom:"16px"}'>
      <q-input dense label='New activity poster uri' :style='{width:"70%"}' v-model='newPoster' />
      <q-btn label='Confirm' @click='onConfirmAddPosterClick' />
      <q-btn label='Cancel' @click='onCancelAddPosterClick' />
    </div>
    <q-btn label='Add' @click='onAddPosterClick' />
    <q-input type='textarea' label='Introduction' v-model='params.introduction' />
    <q-toggle label='Votable' v-model='params.votable' icon='where_to_vote' />
    <div class='row'>
      <q-select :style='{width: "160px",marginRight:"16px"}' :options='ActivityTypes' v-model='params.activityType' label='Activity Type' />
      <q-select :style='{width: "160px",marginRight:"16px"}' :options='VoteTypes' v-model='params.voteType' label='Vote Type' />
      <q-select :style='{width: "160px",marginRight:"16px"}' :options='ObjectTypes' v-model='params.objectType' label='Activity Object Type' />
      <q-select :style='{width: "160px",marginRight:"16px"}' :options='JoinTypes' v-model='params.joinType' label='Join Type' />
    </div>
    <h6>Condition</h6>
    <ul>
      <li v-for='clazz in params.condition.classes' :key='clazz'>
        {{ clazz }}
      </li>
    </ul>
    <div class='row' v-if='addClass' :style='{marginBottom:"16px"}'>
      <q-input dense label='New object class' :style='{width:"70%"}' v-model='newClass' />
      <q-btn label='Confirm' @click='onConfirmAddClassClick' />
      <q-btn label='Cancel' @click='onCancelAddClassClick' />
    </div>
    <q-btn label='Add' @click='onAddClassClick' />
    <div class='row'>
      <q-input type='number' label='Minimum Words' v-model='params.condition.minWords' :style='{marginRight:"16px"}' />
      <q-input type='number' label='Maximum Words' v-model='params.condition.maxWords' />
    </div>
    <h6>Prizes</h6>
    <table :style='{width:"100%",border:"1px solid black"}' class='text-center'>
      <tr>
        <th>Place</th>
        <th>Medal</th>
        <th>Title</th>
        <th>Reward Amount (Linera)</th>
      </tr>
      <tr v-for='prize in params.prizeConfigs' :key='prize.place'>
        <td>{{ prize.place }}</td>
        <td>{{ prize.medal }}</td>
        <td>{{ prize.title }}</td>
        <td>{{ prize.rewardAmount }}</td>
        <td>
          <q-btn @click='onDeletePrizeConfig(prize.place)'>
            Delete
          </q-btn>
        </td>
      </tr>
    </table>
    <div class='row' v-if='addPrize' :style='{marginBottom:"16px"}'>
      <div class='row'>
        <q-input dense label='Prize place' v-model='newPrize.place' type='number' />
        <q-input dense label='Prize medal' v-model='newPrize.medal' :style='{width:"480px",marginRight:"16px"}' />
        <q-input dense label='Prize title' v-model='newPrize.title' :style='{width:"160px",marginRight:"16px"}' />
        <q-input dense label='Prize Reward Amount' v-model='newPrize.rewardAmount' />
      </div>
      <q-btn label='Confirm' @click='onConfirmAddPrizeClick' />
      <q-btn label='Cancel' @click='onCancelAddPrizeClick' />
    </div>
    <q-btn label='Add' @click='onAddPrizeClick' />
    <q-input type='number' label='Voter Reward Percent' v-model='params.voterRewardPercent' />
    <q-input label='Budget Amount' v-model='params.budgetAmount' />
    <q-input label='Activity Location (Url or Address)' v-model='params.location' />
    <div class='row'>
      <q-input
        filled v-model='params.registerStartAt' mask='date' :rules='["date"]'
        label='Register Start Date'
      >
        <template #append>
          <q-icon name='event' class='cursor-pointer'>
            <q-popup-proxy cover transition-show='scale' transition-hide='scale'>
              <q-date v-model='params.registerStartAt' title='Register Start Date'>
                <div class='row items-center justify-end'>
                  <q-btn v-close-popup label='Close' color='primary' flat />
                </div>
              </q-date>
            </q-popup-proxy>
          </q-icon>
        </template>
      </q-input>
      <q-input
        filled v-model='params.registerEndAt' mask='date' :rules='["date"]'
        label='Register End Date'
      >
        <template #append>
          <q-icon name='event' class='cursor-pointer'>
            <q-popup-proxy cover transition-show='scale' transition-hide='scale'>
              <q-date v-model='params.registerEndAt' title='Register End Date'>
                <div class='row items-center justify-end'>
                  <q-btn v-close-popup label='Close' color='primary' flat />
                </div>
              </q-date>
            </q-popup-proxy>
          </q-icon>
        </template>
      </q-input>
      <q-input
        filled v-model='params.voteStartAt' mask='date' :rules='["date"]'
        label='Vote Start Date'
      >
        <template #append>
          <q-icon name='event' class='cursor-pointer'>
            <q-popup-proxy cover transition-show='scale' transition-hide='scale'>
              <q-date v-model='params.voteStartAt' title='Vote Start Date'>
                <div class='row items-center justify-end'>
                  <q-btn v-close-popup label='Close' color='primary' flat />
                </div>
              </q-date>
            </q-popup-proxy>
          </q-icon>
        </template>
      </q-input>
      <q-input
        filled v-model='params.voteEndAt' mask='date' :rules='["date"]'
        label='Vote End Date'
      >
        <template #append>
          <q-icon name='event' class='cursor-pointer'>
            <q-popup-proxy cover transition-show='scale' transition-hide='scale'>
              <q-date v-model='params.voteEndAt' title='Vote End Date'>
                <div class='row items-center justify-end'>
                  <q-btn v-close-popup label='Close' color='primary' flat />
                </div>
              </q-date>
            </q-popup-proxy>
          </q-icon>
        </template>
      </q-input>
    </div>
    <q-btn label='Submit' @click='onSubmitClick' />
  </div>
</template>

<script lang='ts' setup>
import { computed, onMounted, ref, watch } from 'vue'
import {
  ActivityTypes,
  ActivityType,
  VoteTypes,
  VoteType,
  ObjectTypes,
  ObjectType,
  PrizeConfig,
  JoinTypes,
  JoinType,
  CreateParams,
  ObjectCondition,
  useActivityStore
} from 'src/stores/activity'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { targetChain } from 'src/stores/chain'
import { useRoute, useRouter } from 'vue-router'
import { date } from 'quasar'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

interface Query {
  activityId?: number
}
const route = useRoute()
const activityId = ref((route.query as unknown as Query).activityId)
const activity = useActivityStore()
const _activity = computed(() => activity.activity(Number(activityId.value)))

const activity2Params = () => {
  if (!_activity.value) {
    return
  }
  params.value = {
    title: _activity.value.title,
    slogan: _activity.value.slogan,
    banner: _activity.value.banner,
    hostResume: _activity.value.hostResume,
    posters: _activity.value.posters,
    introduction: _activity.value.introduction,
    activityType: activity.activityType(_activity.value.id),
    votable: _activity.value.votable,
    voteType: activity.voteType(_activity.value.id),
    objectType: activity.objectType(_activity.value.id),
    condition: _activity.value.condition,
    sponsors: _activity.value.sponsors,
    prizeConfigs: _activity.value.prizeConfigs,
    voterRewardPercent: _activity.value.voterRewardPercent,
    budgetAmount: _activity.value.budgetAmount,
    joinType: activity.joinType(_activity.value.id),
    location: _activity.value.location,
    registerStartAt: date.formatDate(_activity.value.registerStartAt, 'YYYY/MM/DD').toString(),
    registerEndAt: date.formatDate(_activity.value.registerEndAt, 'YYYY/MM/DD').toString(),
    voteStartAt: date.formatDate(_activity.value.voteStartAt, 'YYYY/MM/DD').toString(),
    voteEndAt: date.formatDate(_activity.value.voteEndAt, 'YYYY/MM/DD').toString()
  } as CreateParams
}

watch(_activity, () => {
  activity2Params()
})

const params = ref({
  title: 'The Best Novel of the Month',
  slogan: 'Find the best author of the world, and support them!',
  banner: 'https://ipfs.moralis.io:2053/ipfs/QmcyuFVLbfBmSeQ9ynu4dk67r97nB1abEekotuVuRGWedm',
  hostResume: 'https://www.baidu.com',
  posters: [
    'https://ipfs.moralis.io:2053/ipfs/QmbzbxQcdcUcdzqrtU1S7cFd7swmcBnytWv5rf1yw94g9s',
    'https://ipfs.moralis.io:2053/ipfs/Qmdco6YbN7qK81tNbQw3RmnowSUXiEuaKjG6tt8FsChDuA',
    'https://ipfs.moralis.io:2053/ipfs/QmTSfgqeKUNvNUqRfQSFTbPkjjFDtGZFxWQkhyqWwMHhAJ'
  ],
  introduction: 'Within the vote period, user can vote for their favorite novel listed in the candidates. The only criteria is how much you like the novel. The author of the winner novel will get 90% of the reward amount. Remainant part of the reward will be distributed to voter according to their vote power when they vote.',
  votable: true,
  activityType: ActivityType.Campaign,
  voteType: VoteType.Power,
  objectType: ObjectType.Content,
  joinType: JoinType.Online,
  condition: {
    classes: [] as Array<string>,
    minWords: 0,
    maxWords: 1000000
  } as ObjectCondition,
  voterRewardPercent: 12,
  budgetAmount: '10000000',
  registerStartAt: date.formatDate(new Date().toString(), 'YYYY/MM/DD'),
  registerEndAt: date.formatDate(new Date(Date.now() + 3600000000).toString(), 'YYYY/MM/DD'),
  voteStartAt: date.formatDate(new Date().toString(), 'YYYY/MM/DD'),
  voteEndAt: date.formatDate(new Date(Date.now() + 3600000000).toString(), 'YYYY/MM/DD'),
  prizeConfigs: [
    {
      place: 1,
      medal: 'https://ipfs.moralis.io:2053/ipfs/QmTDxnzcvj2p3xBrKcGv1wxoyhAn2yzCQnZZ9LmFjReuH9',
      title: 'Best Month Novel',
      rewardAmount: '12500.'
    }, {
      place: 2,
      medal: 'https://ipfs.moralis.io:2053/ipfs/QmbvZ2hbF3nEq5r3ijMEiSGssAmJvtyFwiejTAGHv74LR5',
      title: 'Better Month Novel',
      rewardAmount: '7500.'
    }, {
      place: 3,
      medal: 'https://ipfs.moralis.io:2053/ipfs/QmVpwaCqLut3wqwB5KSQr2fGnbLuJt5e3LhNvzvcisewZB',
      title: 'Good Month Novel',
      rewardAmount: '2500.'
    }
  ],
  location: 'https://ipfs.moralis.io:2053/ipfs/QmcyuFVLbfBmSeQ9ynu4dk67r97nB1abEekotuVuRGWedm'
} as CreateParams)

const newPoster = ref('')
const addPoster = ref(false)
const newClass = ref('')
const addClass = ref(false)
const addPrize = ref(false)
const newPrize = ref({} as PrizeConfig)

const onConfirmAddPosterClick = () => {
  if (!newPoster.value.length || params.value.posters.includes(newPoster.value)) {
    return
  }
  params.value.posters.push(newPoster.value)
  addPoster.value = false
  newPoster.value = ''
}

const onCancelAddPosterClick = () => {
  addPoster.value = false
}

const onAddPosterClick = () => {
  addPoster.value = true
}

const onConfirmAddClassClick = () => {
  if (!newClass.value.length || params.value.condition.classes?.includes(newClass.value)) {
    return
  }
  params.value.condition.classes?.push(newClass.value)
  addClass.value = false
  newClass.value = ''
}

const onCancelAddClassClick = () => {
  addClass.value = false
}

const onAddClassClick = () => {
  addClass.value = true
}

const onConfirmAddPrizeClick = () => {
  if (!newPrize.value.title || params.value.prizeConfigs.find((el) => el.place === newPrize.value.place)) {
    return
  }
  params.value.prizeConfigs.push({ ...newPrize.value })
  addPrize.value = false
  newPrize.value = {} as unknown as PrizeConfig
}

const onCancelAddPrizeClick = () => {
  addPrize.value = false
}

const onAddPrizeClick = () => {
  addPrize.value = true
}

const router = useRouter()

const params2Gql = () => {
  let s = 'mutation'
  if (!_activity.value) {
    s += ` createActivity {
      create (params: {
    `
  } else {
    s += ` updateActivity {
      update (params: {
        activity_id: ${_activity.value.id},
    `
  }
  s += `
      title: "${params.value.title}",
  `
  if (params.value.slogan?.length) {
    s += `slogan: "${params.value.slogan}",`
  }
  s += `
      banner: "${params.value.banner}",
      host_resume: "${params.value.hostResume}",
      posters: [
  `
  params.value.posters?.forEach((poster, i) => {
    s += i < params.value.posters.length - 1 ? `"${poster}",` : `"${poster}"`
  })
  s += '],'
  s += `
    introduction: "${params.value.introduction}",
    activity_type: ${params.value.activityType},
    votable: ${params.value.votable as unknown as string},
    vote_type: ${params.value.voteType},
    object_type: ${params.value.objectType},
    condition: {
      classes: [
  `
  params.value.condition.classes?.forEach((clazz, i) => {
    s += i < (params.value.condition.classes?.length || 0 - 1) ? clazz + ',' : clazz
  })
  s += `    ],
      min_words: ${params.value.condition.minWords},
      max_words: ${params.value.condition.maxWords}
    },
    prize_configs: [
  `
  params.value.prizeConfigs?.forEach((prize, i) => {
    s += `{
      place: ${prize.place},
      medal: "${prize.medal}",
      title: "${prize.title}"
    `
    if (prize.rewardAmount) {
      s += `,reward_amount: "${prize.rewardAmount}"`
    }
    s += '}'
    if (i < params.value.prizeConfigs.length - 1) {
      s += ','
    }
  })
  s += `  ],
    voter_reward_percent: ${params.value.voterRewardPercent},
    budget_amount: "${params.value.budgetAmount as unknown as string}",
    join_type: ${params.value.joinType},
    location: "${params.value.location}",
    register_start_at: ${Date.parse(params.value.registerStartAt)},
    register_end_at: ${Date.parse(params.value.registerEndAt)},
    vote_start_at: ${Date.parse(params.value.voteStartAt)},
    vote_end_at: ${Date.parse(params.value.voteEndAt)},
    sponsors: []
  `
  s += '})'
  s += '}'
  return s
}

const onSubmitClick = async () => {
  const gqlStr = params2Gql()
  console.log(gqlStr)
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql(
    gqlStr
  )))
  onDone(() => {
    void router.push({
      path: '/dashboard',
      query: {
        tab: 'activity'
      }
    })
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    params,
    endpoint: 'activity',
    chainId: targetChain.value
  })
}

const onDeletePrizeConfig = (place: number) => {
  const index = params.value.prizeConfigs.findIndex((el) => el.place === place)
  params.value.prizeConfigs.splice(index >= 0 ? index : 0, index >= 0 ? 1 : 0)
}

onMounted(() => {
  activity2Params()
})

</script>
