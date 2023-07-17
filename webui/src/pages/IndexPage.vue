<template>
  <q-page class='row items-center justify-evenly'>
    <credit-query />
    <block-subscription @block-changed='onBlockChanged' />
  </q-page>
</template>

<script setup lang="ts">
import CreditQuery from '../components/CreditQuery.vue'
import BlockSubscription from '../components/BlockSubscription.vue'

const onBlockChanged = () => {
  console.log('Block changed')
}

</script>
