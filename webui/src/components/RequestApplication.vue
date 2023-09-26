<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import * as constants from 'src/const'
import { useChainStore } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const chain = useChainStore()
const targetChainId = computed(() => chain.targetChain)
const application = useApplicationStore()

const requestApplication = async (index: number) => {
  if (index >= constants.appIds.length) {
    return
  }
  if (!targetChainId.value) {
    return
  }
  const appId = constants.appIds[index]
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation requestApplication ($chainId: String!, $applicationId: String!, $targetChainId: String!) {
      requestApplication(chainId: $chainId, applicationId: $applicationId, targetChainId: $targetChainId)
    }
  `))
  onDone(() => {
    switch (index) {
      case 0:
        application.feedApp = constants.Apps.feedApp
        break
      case 1:
        application.creditApp = constants.Apps.creditApp
        break
      case 2:
        application.marketApp = constants.Apps.marketApp
        break
    }
    void requestApplication(index + 1)
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    chainId: targetChainId.value,
    applicationId: appId,
    targetChainId: constants.appDeployChain,
    endpoint: 'main'
  })
}

watch(targetChainId, () => {
  if (targetChainId.value) {
    void requestApplication(0)
  }
})

onMounted(() => {
  void requestApplication(0)
})

</script>
