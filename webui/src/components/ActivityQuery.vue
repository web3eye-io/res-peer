<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'
import { useActivityStore } from 'src/stores/activity'

const user = useUserStore()
const account = computed(() => user.account)
const application = useApplicationStore()
const activityApp = computed(() => application.activityApp)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const activity = useActivityStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = (): boolean => {
  return account.value?.length > 0 && activityApp.value?.length > 0 && targetChain.value?.length > 0
}

watch(targetChain, () => {
  if (!ready()) return
  getActivities()
})

watch(activityApp, () => {
  if (!ready()) return
  getActivities()
})

watch(account, () => {
  if (!ready()) return
  getActivities()
})

watch(blockHeight, () => {
  if (!ready()) return
  getActivities()
})

const onResult = (res: Record<string, unknown>) => {
  activity.activities = res
}

const getActivities = () => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => {
    if (account.value) {
      return useQuery(gql`
        query getFoundationInfo($account: String!) {
          foundationBalance
          reviewRewardPercent
          reviewRewardBalance
          reviewRewardFactor
          authorRewardPercent
          authorRewardBalance
          authorRewardFactor
          activityRewardPercent
          activityRewardBalance
          userBalances(owner: $account)
        }
      `, {
        account: `${account.value}`,
        endpoint: 'foundation',
        chainId: targetChain.value
      }, {
        fetchPolicy: 'network-only'
      })
    }
    return useQuery(gql`
        query getFoundationInfo {
          foundationBalance
          reviewRewardPercent
          reviewRewardBalance
          reviewRewardFactor
          authorRewardPercent
          authorRewardBalance
          authorRewardFactor
          activityRewardPercent
          activityRewardBalance
        }
      `, {
      endpoint: 'foundation',
      chainId: targetChain.value
    }, {
      fetchPolicy: 'network-only'
    })
  })

  watch(result, () => {
    const res = result.value as Record<string, unknown>
    onResult(res)
  })
}

onMounted(() => {
  if (!ready()) return
  getActivities()
})

</script>
