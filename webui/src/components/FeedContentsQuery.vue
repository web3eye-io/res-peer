<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useContentStore } from 'src/stores/content'
import { computed, watch, ref } from 'vue'

const content = useContentStore()
const contentsKeys = computed(() => content.contentsKeys)
const contents = computed(() => content.contents)
const contentIndex = ref(-1)

const { refetch, onResult, onError } = useQuery(gql`
    query getContent($contentKey: String!) {
      contents(string: $contentKey) {
        accounts
        cid
        likes
        dislikes
      }
    }
  `, {
  contentKey: `${contentIndex.value >= 0 ? contentsKeys.value[contentIndex.value] : ''}`,
  endpoint: 'feed'
})

onResult((res) => {
  console.log(6, res, contentIndex.value, contentsKeys.value.length)
  contents.value.set(contentsKeys.value[contentIndex.value], res as unknown as string)
  if (contentsKeys.value.length <= contentIndex.value) {
    return
  }
  contentIndex.value += 1
  console.log(6, res, contentIndex.value, contentsKeys.value.length)
})
onError((error) => {
  console.log(7, error)
})

watch(contentIndex, () => {
  console.log(4, contentIndex.value, contentsKeys.value.length)
  if (contentIndex.value < 0) {
    return
  }
  if (!contentsKeys.value.length) {
    return
  }
  if (contentsKeys.value.length <= contentIndex.value) {
    return
  }
  console.log(5, contentIndex.value)
  let exists = 0
  for (let i = contentIndex.value; i < contentsKeys.value.length; i++) {
    if (contents.value.get(contentsKeys.value[i])) {
      exists += 1
    }
  }
  console.log(6, contentIndex.value)
  contentIndex.value += exists
  void refetch()
})

watch(contentsKeys, () => {
  console.log(7, contentIndex.value, contentsKeys.value)
  if (contentIndex.value === 0) {
    void refetch()
    return
  }
  contentIndex.value = 0
})

</script>
