import type { ApolloClientOptions } from '@apollo/client/core'
import { createHttpLink, InMemoryCache, split } from '@apollo/client/core'
// import type { BootFileParams } from '@quasar/app'
import { GraphQLWsLink } from '@apollo/client/link/subscriptions'
import { createClient } from 'graphql-ws'
import { getMainDefinition } from '@apollo/client/utilities'
import * as constants from 'src/const'
import { Cookies } from 'quasar'

export /* async */ function getClientOptions (/* {app, router, ...}, options?: Partial<BootFileParams<unknown>> */) {
  const port = Cookies.get('service-port') || constants.port
  const wsLink = new GraphQLWsLink(
    createClient({
      url: 'ws://localhost:' + port + '/ws'
    })
  )

  const httpLink = createHttpLink({
    uri: (operation) => {
      const chainId = operation.variables.chainId as string
      switch (operation.variables.endpoint) {
        case 'feed':
          return 'http://localhost:' + port + '/chains/' + chainId + '/applications/' + constants.Apps.feedApp
        case 'credit':
          return 'http://localhost:' + port + '/chains/' + chainId + '/applications/' + constants.Apps.creditApp
        case 'market':
          return 'http://localhost:' + port + '/chains/' + chainId + '/applications/' + constants.Apps.marketApp
        case 'review':
          return 'http://localhost:' + port + '/chains/' + chainId + '/applications/' + constants.Apps.reviewApp
        case 'foundation':
          return 'http://localhost:' + port + '/chains/' + chainId + '/applications/' + constants.Apps.foundationApp
        case 'activity':
          return 'http://localhost:' + port + '/chains/' + chainId + '/applications/' + constants.Apps.activityApp
        case 'main':
          return 'http://localhost:' + port
        default:
          return 'http://localhost:' + port
      }
    }
  })

  const splitLink = split(
    ({ query }) => {
      const definition = getMainDefinition(query)
      return (
        definition.kind === 'OperationDefinition' &&
        definition.operation === 'subscription'
      )
    },
    wsLink,
    httpLink
  )

  return <ApolloClientOptions<unknown>>Object.assign(
    // General options.
    <ApolloClientOptions<unknown>>{
      link: splitLink,
      cache: new InMemoryCache()
    },

    // Specific Quasar mode options.
    process.env.MODE === 'spa'
      ? {
          //
        }
      : {},
    process.env.MODE === 'ssr'
      ? {
          //
        }
      : {},
    process.env.MODE === 'pwa'
      ? {
          //
        }
      : {},
    process.env.MODE === 'bex'
      ? {
          //
        }
      : {},
    process.env.MODE === 'cordova'
      ? {
          //
        }
      : {},
    process.env.MODE === 'capacitor'
      ? {
          //
        }
      : {},
    process.env.MODE === 'electron'
      ? {
          //
        }
      : {},

    // dev/prod options.
    process.env.DEV
      ? {
          //
        }
      : {},
    process.env.PROD
      ? {
          //
        }
      : {},

    // For ssr mode, when on server.
    process.env.MODE === 'ssr' && process.env.SERVER
      ? {
          ssrMode: true
        }
      : {},
    // For ssr mode, when on client.
    process.env.MODE === 'ssr' && process.env.CLIENT
      ? {
          ssrForceFetchDelay: 100
        }
      : {}
  )
}
