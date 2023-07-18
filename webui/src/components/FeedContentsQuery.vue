<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useContentStore, Content } from 'src/stores/content'
import { computed, watch, ref } from 'vue'

const content = useContentStore()
const contentsKeys = computed(() => content.contentsKeys)
const contents = computed(() => content.contents)
const contentIndex = ref(-1)
const contentKey = computed(() => contentIndex.value >= 0 ? contentsKeys.value[contentIndex.value] : undefined)

watch(contentKey, () => {
  if (!contentKey.value) {
    return
  }
  if (contents.value.get(contentKey.value)) {
    contentIndex.value++
    return
  }

  const options = /* await */ getClientOptions(/* {app, router ...} */)
  const apolloClient = new ApolloClient(options)
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContent($contentKey: String!) {
      contents(string: $contentKey) {
        accounts
        cid
        likes
        dislikes
      }
    }
  `, {
    contentKey: `${contentKey.value as string}`,
    endpoint: 'feed'
  }))

  watch(result, () => {
    contents.value.set(contentKey.value as string, (result.value as Record<string, Content>).contents)
    contentIndex.value++
  })
})

watch(contentsKeys, () => {
  if (contentsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

</script>
