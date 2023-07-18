<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { AgeAmount, useUserStore } from 'src/stores/user'
import { computed, watch } from 'vue'

const user = useUserStore()

const account = computed(() => user.account)

const { result, refetch } = useQuery(gql`
  query getBalance($owner: String!) {
    spendables(
      owner: $owner
    )
    balances(
      owner: $owner
    ) {
      amounts {
        amount
        expired
      }
    }
  }
`, {
  owner: account.value,
  endpoint: 'credit'
})

watch(result, () => {
  if (!result.value) {
    return
  }
  user.spendable = (result.value as Record<string, string>).spendables
  user.balances = (result.value as Record<string, Record<string, Array<AgeAmount>>>).balances.amounts
})

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
watch(blockHeight, () => {
  if (!account.value?.length) {
    return
  }
  void refetch({
    owner: account.value,
    endpoint: 'credit'
  })
})

watch(account, () => {
  if (!account.value?.length) {
    return
  }
  void refetch({
    owner: account.value,
    endpoint: 'credit'
  })
})

</script>
