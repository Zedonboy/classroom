<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { canisterId, createActor } from "@declaration/classroom_backend"
import { HOST } from '../config'
import { toast } from 'vue3-toastify'

interface ExchangeRate {
  from_currency: string
  to_currency: string
  rate: number
  last_updated: string
}

const fromCurrency = ref('USD')
const toCurrency = ref('EUR')
const exchangeRate = ref<ExchangeRate | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

const fetchRate = async () => {
  loading.value = true
  error.value = null

  try {
    const actor = createActor(canisterId, {
      agentOptions: {
        host: HOST
      }
    })

    const result = await actor.get_exchange_rate(fromCurrency.value, toCurrency.value)

    if ("Ok" in result) {
      exchangeRate.value = result.Ok
      toast.success('Exchange rate updated successfully!', {
        position: toast.POSITION.TOP_RIGHT,
        autoClose: 2000,
      })
    } else {
      throw new Error(result.Err || 'Failed to fetch exchange rate')
    }
  } catch (err: any) {
    error.value = 'Failed to fetch exchange rate. Please try again.'
    toast.error(error.value, {
      position: toast.POSITION.TOP_RIGHT,
      autoClose: 3000,
    })
    console.error('Error fetching rate:', err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchRate()
})
</script>

<template>
  <div class="forex-widget p-4 bg-white rounded-lg shadow-md">
    <div class="mb-4">
      <h2 class="text-xl font-bold mb-4">Currency Exchange Rate</h2>

      <!-- Currency Selection -->
      <div class="grid grid-cols-2 gap-4 mb-4">
        <div>
          <label class="block text-sm font-medium mb-1">From Currency</label>
          <select v-model="fromCurrency" class="w-full p-2 border rounded" @change="fetchRate">
            <option value="USD">USD</option>
            <option value="EUR">EUR</option>
            <option value="GBP">GBP</option>
            <option value="JPY">JPY</option>
            <option value="BTC">BTC</option>
          </select>
        </div>

        <div>
          <label class="block text-sm font-medium mb-1">To Currency</label>
          <select v-model="toCurrency" class="w-full p-2 border rounded" @change="fetchRate">
            <option value="USD">USD</option>
            <option value="EUR">EUR</option>
            <option value="GBP">GBP</option>
            <option value="JPY">JPY</option>
            <option value="BTC">BTC</option>
          </select>
        </div>
      </div>

      <!-- Exchange Rate Display -->
      <div v-if="exchangeRate" class="text-center p-4 bg-gray-50 rounded">
        <div class="text-2xl font-bold mb-2">
          1 {{ exchangeRate.from_currency }} = {{ Number(exchangeRate.rate).toFixed(4) }} {{ exchangeRate.to_currency }}
        </div>
        <div class="text-sm text-gray-500">
          Last Updated: {{ exchangeRate.last_updated }}
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="text-center p-4">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 mx-auto"></div>
      </div>

      <!-- Refresh Button -->
      <button @click="fetchRate"
        class="mt-4 w-full bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        :disabled="loading">
        {{ loading ? 'Refreshing...' : 'Refresh Rate' }}
      </button>
    </div>
  </div>
</template>