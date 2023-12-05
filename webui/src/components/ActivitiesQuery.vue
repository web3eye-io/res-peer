<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { computed, watch, ref } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { Activity, useActivityStore } from 'src/stores/activity'
import { targetChain } from 'src/stores/chain'

const activity = useActivityStore()
const activitiesKeys = computed(() => activity.activitiesKeys)
const activities = computed(() => activity.activities)
const contentIndex = ref(-1)
const activityKey = computed(() => contentIndex.value >= 0 ? activitiesKeys.value[contentIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getActivities = (activityKey: number, done?: () => void) => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getActivities($activityKey: Int!) {
      activities(u64: $activityKey) {
        id
        title
        slogan
        banner
        posters
        introduction
        host
        hostResume
        createdAt
        activityType
        votable
        voteType
        objectType
        objectCandidates
        condition {
          classes
          minWords
          maxWords
        }
        sponsors
        prizeConfigs {
          place
          medal
          title
          rewardAmount
        }
        announcements
        prizeAnnouncement
        voterRewardPercent
        votePowers
        voters
        budgetAmount
        joinType
        location
        comments
        registers
        registerStartAt
        registerEndAt
        voteStartAt
        voteEndAt
        participantors
        winners {
          place
          objectId
        }
        finalized
      }
    }
  `, {
    activityKey,
    endpoint: 'activity',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  watch(result, () => {
    activities.value.set(Number(activityKey), (result.value as Record<string, Activity>).activities)
    done?.()
  })
}

watch(activityKey, () => {
  if (!activityKey.value) {
    return
  }
  getActivities(activityKey.value, () => {
    contentIndex.value++
  })
})

watch(activitiesKeys, () => {
  if (activitiesKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

watch(blockHeight, () => {
  if (activitiesKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

</script>
