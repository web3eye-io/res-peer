<template>
  <div class='row' :style='{fontSize:"18px", marginBottom: "16px"}'>
    Foundation Balance
    <q-space />
    {{ foundation.foundationBalance }} Lineras
  </div>
  <q-separator />
  <div class='row' :style='{fontSize:"18px", marginTop: "16px"}'>
    Foundation Review Reward Balance
    <q-space />
    {{ foundation.reviewRewardBalance }} Lineras
  </div>
  <div class='row' :style='{fontSize:"18px", marginBottom: "16px"}'>
    Current Review Reward
    <q-space />
    {{ Number(foundation.reviewRewardBalance) / foundation.reviewRewardFactor }} Lineras
  </div>
  <q-separator />
  <div class='row' :style='{fontSize:"18px", marginTop: "16px"}'>
    Foundation Author Reward Balance
    <q-space />
    {{ foundation.authorRewardBalance }} Lineras
  </div>
  <div class='row' :style='{fontSize:"18px", marginBottom: "16px"}'>
    Current Author Reward
    <q-space />
    {{ Number(foundation.authorRewardBalance) / foundation.authorRewardFactor }} Lineras
  </div>
  <q-separator />
  <div class='row' :style='{fontSize:"18px", marginTop: "16px", color: "blue", fontWeight: "bold"}'>
    Join ResPeer now! Active to Earn Linera and Credits
  </div>
</template>

<script lang='ts' setup>
import { useFoundationStore } from 'src/stores/foundation'

const foundation = useFoundationStore()

</script>
