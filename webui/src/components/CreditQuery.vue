<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useUserStore } from 'src/stores/user'
import { computed, watch } from 'vue'

const user = useUserStore()

const account = computed(() => user.account)

const { refetch } = useQuery(gql`
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
