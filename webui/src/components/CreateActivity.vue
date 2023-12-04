<template>
  <div :style='{width: "1440px", margin: "32px auto"}'>
    <h4>Create Activity</h4>
    <q-input label='Title' v-model='params.title' />
    <q-input label='Slogan' v-model='params.slogan' />
    <q-input label='Banner' v-model='params.banner' />
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
    <table :style='{width:"100%",border:"1px solid black"}'>
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
      </tr>
    </table>
    <div class='row' v-if='addPrize' :style='{marginBottom:"16px"}'>
      <div class='row'>
        <q-input dense label='Prize place' v-model='newPrize.place' type='number' />
        <q-input dense label='Prize medal' v-model='newPrize.medal' />
        <q-input dense label='Prize title' v-model='newPrize.title' />
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
import { ref } from 'vue'
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
  ObjectCondition
} from 'src/stores/activity'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { targetChain } from 'src/stores/chain'
import { useRouter } from 'vue-router'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const params = ref({
  votable: true,
  activityType: ActivityType.Campaign,
  voteType: VoteType.Power,
  objectType: ObjectType.Content,
  joinType: JoinType.Online,
  condition: {
    minWords: 0,
    maxWords: 1000000
  } as ObjectCondition,
  voterRewardPercent: 12,
  budgetAmount: '10000000'
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
  params.value.prizeConfigs.push(newPrize.value)
  addPrize.value = false
}

const onCancelAddPrizeClick = () => {
  addPrize.value = false
}

const onAddPrizeClick = () => {
  addPrize.value = true
}

const router = useRouter()

const onSubmitClick = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation createActivity ($params: CreateParams!) {
      create(params: $params)
    }
  `))
  onDone(() => {
    void router.back()
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

</script>
