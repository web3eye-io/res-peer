<template>
  <div class='text-h4'>
    {{ spendableCredits }} Credits
  </div>
  <table>
    <tr>
      <th>Amount</th>
      <th>Expired Date</th>
    </tr>

    <tr
      v-for='(amount, index) in creditAmounts'
      :key='index'
    >
      <td>{{ amount.amount }}</td>
      <td :style='{color: amount.expired / 1000 < Date.now() ? "red" : "" }'>
        {{ new Date(amount.expired / 1000) }}
      </td>
    </tr>
  </table>
</template>

<script setup lang='ts'>
import { useUserStore } from 'src/stores/user'
import { computed } from 'vue'

const user = useUserStore()
const spendableCredits = computed(() => user.spendable)
const creditAmounts = computed(() => Array.from(user.amounts).sort((a, b) => a.expired > b.expired ? 1 : -1))

</script>

<style scoped lang='sass'>
table, th, td
  border-collapse: collapse

tr:nth-child(even)
  background-color: #dddddd
</style>
