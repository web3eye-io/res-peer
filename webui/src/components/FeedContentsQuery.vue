<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useContentStore, Content } from 'src/stores/content'
import { computed, watch, ref, onMounted } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'

const content = useContentStore()
const contentsKeys = computed(() => content.contentsKeys)
const contents = computed(() => content.contents)
const recommends = computed(() => content.recommends)
const comments = computed(() => content.comments)
const contentIndex = ref(-1)
const contentKey = computed(() => contentIndex.value >= 0 ? contentsKeys.value[contentIndex.value] : undefined)
const mutateKeys = computed(() => content.mutateKeys)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getContent = (contentKey: string, done?: () => void) => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContent($contentKey: String!) {
      contents(string: $contentKey) {
        accounts
        cid
        commentToCid
        title
        content
        author
        likes
        dislikes
        createdAt
      }
      contentRecommends(string: $contentKey)
      contentComments(string: $contentKey)
    }
  `, {
    contentKey: `${contentKey}`,
    endpoint: 'feed',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  watch(result, () => {
    contents.value.set(contentKey, (result.value as Record<string, Content>).contents)
    recommends.value.set(contentKey, (result.value as Record<string, Array<string>>).contentRecommends)
    comments.value.set(contentKey, (result.value as Record<string, Array<string>>).contentComments)
    done?.()
  })
}

watch(contentKey, () => {
  if (!contentKey.value) {
    return
  }
  const index = mutateKeys.value.findIndex((el) => el === contentKey.value)
  if (contents.value.get(contentKey.value) && index < 0) {
    contentIndex.value++
    return
  }

  getContent(contentKey.value, () => {
    contentIndex.value++
    mutateKeys.value.splice(index, 1)
  })
})

watch(contentsKeys, () => {
  if (contentsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

watch(blockHeight, () => {
  if (contentsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

onMounted(() => {
  content.mutateKeys.forEach((contentKey) => {
    getContent(contentKey)
  })
  content.mutateKeys = []
})

</script>
