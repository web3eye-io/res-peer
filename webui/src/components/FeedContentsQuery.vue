<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useContentStore, Content } from 'src/stores/content'
import { computed, watch, ref, onMounted } from 'vue'

const content = useContentStore()
const contentsKeys = computed(() => content.contentsKeys)
const contents = computed(() => content.contents)
const contentIndex = ref(-1)
const contentKey = computed(() => contentIndex.value >= 0 ? contentsKeys.value[contentIndex.value] : undefined)
const mutateKeys = computed(() => content.mutateKeys.length)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getContent = (contentKey: string, done?: () => void) => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContent($contentKey: String!) {
      contents(string: $contentKey) {
        accounts
        cid
        title
        content
        author
        likes
        dislikes
        createdAt
      }
    }
  `, {
    contentKey: `${contentKey}`,
    endpoint: 'feed'
  }))

  watch(result, () => {
    contents.value.set(contentKey, (result.value as Record<string, Content>).contents)
    done?.()
  })
}

watch(contentKey, () => {
  if (!contentKey.value) {
    return
  }
  if (contents.value.get(contentKey.value)) {
    contentIndex.value++
    return
  }

  getContent(contentKey.value, () => {
    contentIndex.value++
  })
})

watch(contentsKeys, () => {
  if (contentsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

watch(mutateKeys, () => {
  content.mutateKeys.forEach((contentKey) => {
    getContent(contentKey)
  })
  content.mutateKeys = []
})

onMounted(() => {
  content.mutateKeys.forEach((contentKey) => {
    getContent(contentKey)
  })
  content.mutateKeys = []
})

</script>
