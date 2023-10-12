<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'

const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return targetChain.value?.length > 0 && marketApp.value?.length > 0
}

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
    endpoint: 'market',
    chainId: targetChain.value
  })
}

watch(targetChain, () => {
  if (!ready()) return
  void requestSubscribe()
})

watch(marketApp, () => {
  if (!ready()) return
  void requestSubscribe()
})

onMounted(() => {
  if (!ready()) return
  void requestSubscribe()
})

</script>
