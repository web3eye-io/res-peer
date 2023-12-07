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
      { path: 'reviewreviewer', component: () => import('pages/ReviewReviewer.vue') },
      { path: 'reviewactivity', component: () => import('pages/ReviewActivity.vue') },
      { path: 'content', component: () => import('pages/ContentPage.vue') },
      { path: 'create/activity', component: () => import('pages/CreateActivity.vue') },
      { path: 'activities', component: () => import('pages/ActivitiesPage.vue') },
      { path: 'activity', component: () => import('pages/ActivityPage.vue') },
      { path: 'activity/register', component: () => import('pages/ActivityRegister.vue') },
      { path: 'activity/vote', component: () => import('pages/ActivityVote.vue') }
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
