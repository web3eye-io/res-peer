<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'

const user = useUserStore()
const account = computed(() => user.account)

const { refetch, onResult } = useQuery(gql`
  query getMallInfo($account: String!) {
    collectionsKeys
    creditsPerLinera
    balances(owner: $account)
    assets(owner: $account)
  }
`, {
  account: `${account.value}`,
  endpoint: 'mall'
})

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
watch(blockHeight, () => {
  void refetch({
    account: `${account.value}`,
    endpoint: 'mall'
  })
})

watch(account, () => {
  void refetch({
    account: `${account.value}`,
    endpoint: 'mall'
  })
})

const collection = useCollectionStore()

onResult((res) => {
  if (res.loading || !res.data) {
    return
  }
  // TODO: a bug here will cause balances to be another value from credits, don't know why
  collection.collectionsKeys = (res.data as Record<string, Array<number>>).collectionsKeys
  collection.creditsPerLinera = (res.data as Record<string, string>).creditsPerLinera
  const balance = (res.data as Record<string, string>).balances
  if (typeof (balance) === 'string') {
    collection.lineraBalance = (res.data as Record<string, string>).balances
  }
  const assets = (res.data as Record<string, Record<number, Array<number>>>).assets
  Object.keys(assets).forEach((key, index) => {
    collection.assets.set(parseInt(key), Object.values(assets)[index])
  })
})

onMounted(() => {
  void refetch({
    account: `${account.value}`,
    endpoint: 'mall'
  })
})

</script>
