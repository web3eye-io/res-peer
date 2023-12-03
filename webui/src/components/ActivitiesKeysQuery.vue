<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { computed, onMounted, watch } from 'vue'
import { targetChain } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'
import { useActivityStore } from 'src/stores/activity'

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const activity = useActivityStore()
const application = useApplicationStore()
const activityApp = computed(() => application.activityApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return targetChain.value?.length > 0 && activityApp.value?.length > 0
}

const getActivitiesKeys = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getActivitiesKeys {
      activitiesKeys
    }
  `, {
    endpoint: 'activity',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    activity.activitiesKeys = (res.data as Record<string, Array<number>>).activitiesKeys
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  getActivitiesKeys()
})

watch(activityApp, () => {
  if (!ready()) return
  getActivitiesKeys()
})

onMounted(() => {
  if (!ready()) return
  getActivitiesKeys()
})

</script>
