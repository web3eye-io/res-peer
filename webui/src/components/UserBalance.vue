<template>
  <q-table
    :rows='creditAmounts'
    :columns='(columns as never)'
    :rows-per-page-options='[10]'
  >
    <template #top-left>
      <div class='text-h4'>
        {{ spendableCredits }} Credits
      </div>
    </template>
  </q-table>
</template>

<script setup lang='ts'>
import { AgeAmount, useUserStore } from 'src/stores/user'
import { computed } from 'vue'

const user = useUserStore()
const spendableCredits = computed(() => user.spendable)
const creditAmounts = computed(() => Array.from(user.amounts).sort((a, b) => a.expired > b.expired ? 1 : -1))

const columns = computed(() => [
  {
    name: 'Amount',
    label: 'Amount',
    align: 'left',
    field: (row: AgeAmount) => row.amount
  }, {
    name: 'Expired Date',
    label: 'Expired Date',
    field: (row: AgeAmount) => new Date(row.expired / 1000).toDateString()
  }
])

</script>

<style scoped lang='sass'>
</style>
