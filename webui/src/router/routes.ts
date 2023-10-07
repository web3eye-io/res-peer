import { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/IndexPage.vue') },
      { path: 'dashboard', component: () => import('pages/DashboardPage.vue') },
      { path: 'market', component: () => import('pages/NftMarketPage.vue') },
      { path: 'collection', component: () => import('pages/CollectionPage.vue') },
      { path: 'reviewcontent', component: () => import('pages/ReviewContent.vue') },
      { path: 'reviewasset', component: () => import('pages/ReviewAsset.vue') },
      { path: 'content', component: () => import('pages/ContentPage.vue') }
    ]
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue')
  }
]

export default routes
