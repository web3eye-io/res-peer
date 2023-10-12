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
import { useFoundationStore } from 'src/stores/foundation'

const user = useUserStore()
const account = computed(() => user.account)
const application = useApplicationStore()
const foundationApp = computed(() => application.foundationApp)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const foundation = useFoundationStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = (): boolean => {
  return account.value?.length > 0 && foundationApp.value?.length > 0 && targetChain.value?.length > 0
}

watch(targetChain, () => {
  if (!ready()) return
  getFoundationInfo()
})

watch(foundationApp, () => {
  if (!ready()) return
  getFoundationInfo()
})

watch(account, () => {
  if (!ready()) return
  getFoundationInfo()
})

watch(blockHeight, () => {
  if (!ready()) return
  getFoundationInfo()
})

const onResult = (res: Record<string, unknown>) => {
  foundation.userLineraBalance = (res as Record<string, string>).userBalances
  foundation.foundationBalance = (res as Record<string, string>).foundationBalance
  foundation.reviewRewardBalance = (res as Record<string, string>).reviewRewardBalance
  foundation.authorRewardBalance = (res as Record<string, string>).authorRewardBalance
  foundation.activityRewardBalance = (res as Record<string, string>).activityRewardBalance
  foundation.reviewRewardPercent = (res as Record<string, number>).reviewRewardPercent
  foundation.reviewRewardFactor = (res as Record<string, number>).reviewRewardFactor
  foundation.authorRewardPercent = (res as Record<string, number>).authorRewardPercent
  foundation.authorRewardFactor = (res as Record<string, number>).authorRewardFactor
  foundation.activityRewardPercent = (res as Record<string, number>).activityRewardPercent
}

const getFoundationInfo = () => {
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
  getFoundationInfo()
})

</script>
