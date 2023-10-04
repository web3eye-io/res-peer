<script lang='ts' setup>
import { computed, onMounted } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import { useUserStore } from 'src/stores/user'
import { targetChain } from 'src/stores/chain'

const user = useUserStore()
const reviewer = computed(() => user.reviewer)
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
    endpoint: 'review',
    chainId: targetChain.value
  })
}

onMounted(() => {
  if (reviewer.value) {
    void requestSubscribe()
  }
})
</script>
