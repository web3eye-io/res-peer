<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import * as constants from 'src/const'
import { useChainStore } from 'src/stores/chain'

const appIDs = ref([constants.feedAppID, constants.creditAppID, constants.marketAppID])

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const chain = useChainStore()
const defaultChain = computed(() => chain.defaultChain)

const requestApplication = async (index: number) => {
  if (index >= appIDs.value.length) {
    return
  }

  if (!defaultChain.value) {
    return
  }

  const appID = appIDs.value[index]
  const targetChainId = defaultChain.value

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation requestApplication ($applicationId: String!, $targetChainId: String!) {
      requestApplication(applicationId: $applicationId, targetChainId: $targetChainId)
    }
  `))
  onDone(() => {
    void requestApplication(index + 1)
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    applicationId: appID,
    targetChainId,
    endpoint: 'main'
  })
}

watch(defaultChain, () => {
  if (defaultChain.value) {
    void requestApplication(0)
  }
})

onMounted(() => {
  void requestApplication(0)
})

</script>
