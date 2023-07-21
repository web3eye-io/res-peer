<script setup lang="ts">
import { useSubscription } from '@vue/apollo-composable'
// import { ApolloQuery } from '@vue/apollo-components'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'

const {
  onResult
} = useSubscription(gql`
  subscription {
    notifications
  }
`)

const block = useBlockStore()

onResult((res) => {
  const data = res.data as Record<string, Record<string, Record<string, Record<string, unknown>>>>
  if (data.notifications.reason.NewBlock) {
    block.blockHeight = data.notifications.reason.NewBlock.height as number
    block.blockHash = data.notifications.reason.NewBlock.hash as string
  }
})
</script>
