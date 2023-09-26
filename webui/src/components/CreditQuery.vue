<script setup lang="ts">
import { useBlockStore } from 'src/stores/block'
import { AgeAmount, useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useApplicationStore } from 'src/stores/application'

const user = useUserStore()
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const account = computed(() => user.account)
const application = useApplicationStore()
const creditApp = computed(() => application.creditApp)
// const chain = useChainStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return account.value?.length && creditApp.value?.length
}

const getBalance = () => {
  const { result, refetch /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getBalance($owner: String!) {
      spendables(
        owner: $owner
      )
      balances(
        owner: $owner
      ) {
        amounts {
          amount
          expired
        }
      }
    }
  `, {
    owner: account.value,
    endpoint: 'credit'
  }))

  watch(result, () => {
    if (!ready()) {
      return
    }
    user.spendable = (result.value as Record<string, string>).spendables
    const balance = (result.value as Record<string, Record<string, Array<AgeAmount>>>).balances
    if (balance) {
      user.amounts = balance.amounts
    }
  })

  watch(blockHeight, () => {
    if (!ready()) {
      return
    }
    void refetch({
      owner: account.value,
      endpoint: 'credit'
    })
  })

  watch(account, () => {
    if (!ready()) {
      return
    }
    void refetch({
      owner: account.value,
      endpoint: 'credit'
    })
  })
}

watch(creditApp, () => {
  if (!ready()) {
    return
  }
  getBalance()
})

onMounted(() => {
  if (!ready()) {
    return
  }
  getBalance()
})

</script>
