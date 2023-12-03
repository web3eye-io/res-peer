<template>
  <div class='row' :style='{fontSize:"18px", marginBottom: "16px"}'>
    Foundation Balance
    <q-space />
    {{ foundation.foundationBalance }} Lineras
  </div>
  <div class='row' :style='{fontSize:"18px"}'>
    Author Reward Percent
    <q-space />
    {{ foundation.authorRewardPercent }}%
  </div>
  <div class='row' :style='{fontSize:"18px"}'>
    Author Reward Factor
    <q-space />
    {{ foundation.authorRewardFactor }}
  </div>
  <div class='row' :style='{fontSize:"18px"}'>
    Review Reward Percent
    <q-space />
    {{ foundation.reviewRewardPercent }}%
  </div>
  <div class='row' :style='{fontSize:"18px"}'>
    Review Reward Factor
    <q-space />
    {{ foundation.reviewRewardFactor }}
  </div>
  <div class='row' :style='{fontSize:"18px", marginBottom: "16px"}'>
    Activity Reward Percent
    <q-space />
    {{ foundation.activityRewardPercent }}%
  </div>
  <q-separator />
  <div class='row' :style='{fontSize:"18px", marginTop: "16px"}'>
    Credits / Linera
    <q-space />
    {{ collection.creditsPerLinera }}
  </div>
  <div class='row' :style='{fontSize:"18px"}'>
    Max Credits Percent
    <q-space />
    {{ collection.maxCreditsPercent }}%
  </div>
  <div class='row' :style='{fontSize:"18px", marginBottom: "16px"}'>
    Trade Fee Percent
    <q-space />
    {{ collection.tradeFeePercent }}%
  </div>
  <q-separator />
  <div class='row' :style='{fontSize:"18px", marginTop: "16px"}'>
    Review Reward Balance
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
    Author Reward Balance
    <q-space />
    {{ foundation.authorRewardBalance }} Lineras
  </div>
  <div class='row' :style='{fontSize:"18px", marginBottom: "16px"}'>
    Current Author Reward
    <q-space />
    {{ Number(foundation.authorRewardBalance) / foundation.authorRewardFactor }} Lineras
  </div>
  <q-separator />
  <q-separator />
  <div class='row' :style='{fontSize:"18px", margin: "16px 0"}'>
    Activity Reward Balance
    <q-space />
    {{ foundation.activityRewardBalance }} Lineras
  </div>
  <q-separator />
  <div class='row' :style='{fontSize:"18px", marginTop: "16px", color: "blue", fontWeight: "bold"}'>
    Join ResPeer now! Active to Earn Linera and Credits
  </div>
</template>

<script lang='ts' setup>
import { useCollectionStore } from 'src/stores/collection'
import { useFoundationStore } from 'src/stores/foundation'

const foundation = useFoundationStore()
const collection = useCollectionStore()

</script>
