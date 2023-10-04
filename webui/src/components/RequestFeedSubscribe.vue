<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'

const application = useApplicationStore()
const feedApp = computed(() => application.feedApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const requestSubscribe = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation requestSubscribe {
      requestSubscribe
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    endpoint: 'feed',
    chainId: targetChain.value
  })
}

watch(feedApp, () => {
  if (!feedApp.value) return
  void requestSubscribe()
})

onMounted(() => {
  if (!feedApp.value) return
  void requestSubscribe()
})

</script>
