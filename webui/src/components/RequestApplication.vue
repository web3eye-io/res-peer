<script setup lang="ts">
import { onMounted, ref } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import * as constants from 'src/const'

const appIDs = ref([constants.feedAppID, constants.creditAppID, constants.marketAppID])

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const requestApplication = async (index: number) => {
  if (index >= appIDs.value.length) {
    return
  }

  const appID = appIDs.value[index]
  const targetChainId = constants.appChainId

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

onMounted(() => {
  void requestApplication(0)
})

</script>
