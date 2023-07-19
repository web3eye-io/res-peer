<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useCollectionStore } from 'src/stores/collection'
import { computed, watch } from 'vue'

const { refetch, onResult } = useQuery(gql`
  query getCollectionsKeys {
    collectionsKeys
    creditsPerLinera
  }
`, {
  endpoint: 'mall'
})

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
watch(blockHeight, () => {
  void refetch()
})

const collection = useCollectionStore()

onResult((res) => {
  if (res.loading) {
    return
  }
  collection.collectionsKeys = (res.data as Record<string, Array<number>>).collectionsKeys
  collection.creditsPerLinera = (res.data as Record<string, string>).creditsPerLinera
})

</script>
