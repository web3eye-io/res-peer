<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { computed, watch } from 'vue'

const { refetch } = useQuery(gql`
  query {
    balancesKeys
    spendables(
      owner: "b975c98d6921a2beb1d974d83a29304eb5f5ad301a55e56e7984079607fcb633"
    )
    spendablesKeys
    balance
    balances(
      owner: "b975c98d6921a2beb1d974d83a29304eb5f5ad301a55e56e7984079607fcb633"
    ) {
      amounts {
        amount
        expired
      }
    }
  }
`, {
  endpoint: 'credit'
})

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
watch(blockHeight, () => {
  void refetch()
})

</script>
