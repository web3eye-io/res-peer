<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
// import { useChainStore } from 'src/stores/chain'
import { onMounted, watch } from 'vue'

// const chain = useChainStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getChains = (done?: () => void) => {
  console.log('get chains')
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query chains {
      list
      default
    }
  `, {
    endpoint: 'main'
  }, {
    fetchPolicy: 'cache-and-network'
  }))

  watch(result, () => {
    console.log(result.value)
    done?.()
  })
}

onMounted(() => {
  getChains()
})

</script>
