<script lang='ts' setup>
import { computed, onMounted, watch } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import { targetChain } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'

const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return targetChain.value?.length > 0 && reviewApp.value?.length > 0
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
    endpoint: 'review',
    chainId: targetChain.value
  })
}

watch(targetChain, () => {
  if (!ready()) return
  void requestSubscribe()
})

watch(reviewApp, () => {
  if (!ready()) return
  void requestSubscribe()
})

onMounted(() => {
  if (!ready()) return
  void requestSubscribe()
})
</script>
