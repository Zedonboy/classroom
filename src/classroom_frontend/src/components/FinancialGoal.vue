<script setup lang="ts">
import { ref, reactive } from 'vue'
import { canisterId, createActor } from "@declaration/classroom_backend"
import { FinancialGoal } from '@declaration/classroom_backend/classroom_backend.did'
import { HOST } from '../config'
import { toast } from 'vue3-toastify'
import 'vue3-toastify/dist/index.css'

interface FormResponse {
  status: 'success' | 'error'
  message: string
}

// State
const formData = reactive<FinancialGoal>({
  purpose: '',
  timeframe: '',
  total_amount: 0,
  total_income_monthly: 0,
  total_expenses_monthly: 0
})

const loading = ref(false)
const response = ref<FormResponse | null>(null)

// Form validation with detailed error messages
const validateForm = (): boolean => {
  if (!formData.purpose.trim()) {
    toast.error("Please enter a purpose for your financial goal", {
      position: toast.POSITION.TOP_RIGHT,
      autoClose: 3000,
    })
    return false
  }
  
  if (!formData.timeframe.trim()) {
    toast.error("Please specify a timeframe for your goal", {
      position: toast.POSITION.TOP_RIGHT,
      autoClose: 3000,
    })
    return false
  }
  
  if (!formData.total_amount || formData.total_amount <= 0) {
    toast.error("Please enter a valid total amount greater than 0", {
      position: toast.POSITION.TOP_RIGHT,
      autoClose: 3000,
    })
    return false
  }
  
  if (!formData.total_income_monthly || formData.total_income_monthly <= 0) {
    toast.error("Please enter a valid monthly income greater than 0", {
      position: toast.POSITION.TOP_RIGHT,
      autoClose: 3000,
    })
    return false
  }
  
  if (!formData.total_expenses_monthly || formData.total_expenses_monthly <= 0) {
    toast.error("Please enter valid monthly expenses greater than 0", {
      position: toast.POSITION.TOP_RIGHT,
      autoClose: 3000,
    })
    return false
  }
  
  if (formData.total_expenses_monthly > formData.total_income_monthly) {
    toast.warn("Your monthly expenses exceed your income. Please review your budget.", {
      position: toast.POSITION.TOP_RIGHT,
      autoClose: 5000,
    })
    return false
  }

  return true
}

// Form submission handler with enhanced error handling
const submitForm = async () => {
  if (!validateForm()) return

  loading.value = true
  response.value = null

  console.log(canisterId)
  try {
    let actor = createActor(canisterId, {
      agentOptions: {
        host: HOST
      }
    })

    const result = await actor.submit_financial_goal(formData)

    if ("Ok" in result) {
      toast.success('Financial goal submitted successfully!', {
        position: toast.POSITION.TOP_RIGHT,
        autoClose: 3000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        //@ts-ignore
        draggable: true,
        theme: 'light'
      })
      
      resetForm()
    } else {
      toast.error('Failed to submit financial goal: ' + ("Err" in result ? result.Err : "Unknown error"), {
        position: toast.POSITION.TOP_RIGHT,
        autoClose: 5000,
      })
    }
  } catch (error: any) {
    let errorMessage = 'An unexpected error occurred'
    
    // Handle different types of errors
    if (error.message?.includes('connection')) {
      errorMessage = 'Connection error. Please check your internet connection.'
    } else if (error.message?.includes('timeout')) {
      errorMessage = 'Request timed out. Please try again.'
    } else if (error.message?.includes('rejected')) {
      errorMessage = 'Request was rejected. Please try again later.'
    }
    
    toast.error(errorMessage, {
      position: toast.POSITION.TOP_RIGHT,
      autoClose: 5000,
      hideProgressBar: false,
      closeOnClick: true,
      pauseOnHover: true,
      //@ts-ignore
      draggable: true,
      theme: 'light'
    })
    
    response.value = {
      status: 'error',
      message: errorMessage
    }
  } finally {
    loading.value = false
  }
}

// Handle input validation on blur
const handleBlur = (field: string, value: any) => {
  switch (field) {
    case 'purpose':
      if (!value.trim()) {
        toast.info('Please enter a purpose', {
          position: toast.POSITION.TOP_RIGHT,
          autoClose: 2000,
        })
      }
      break
    case 'timeframe':
      if (!value.trim()) {
        toast.info('Please specify a timeframe', {
          position: toast.POSITION.TOP_RIGHT,
          autoClose: 2000,
        })
      }
      break
    case 'amount':
      if (!value || value <= 0) {
        toast.info('Please enter a valid amount', {
          position: toast.POSITION.TOP_RIGHT,
          autoClose: 2000,
        })
      }
      break
    case 'income':
      if (!value || value <= 0) {
        toast.info('Please enter valid monthly income', {
          position: toast.POSITION.TOP_RIGHT,
          autoClose: 2000,
        })
      }
      break
    case 'expenses':
      if (!value || value <= 0) {
        toast.info('Please enter valid monthly expenses', {
          position: toast.POSITION.TOP_RIGHT,
          autoClose: 2000,
        })
      }
      break
  }
}

// Reset form
const resetForm = () => {
  Object.assign(formData, {
    purpose: '',
    timeframe: '',
    total_amount: 0,
    total_income_monthly: 0,
    total_expenses_monthly: 0
  })
}
</script>

<template>
  <div class="max-w-2xl mx-auto p-6">
    <h1 class="text-3xl font-bold mb-6">Financial Goal Planner</h1>

    <form @submit.prevent="submitForm" class="space-y-6">
      <!-- Purpose Input -->
      <div class="form-group">
        <label for="purpose" class="block text-sm font-medium text-gray-700 mb-1">
          Purpose
        </label>
        <input id="purpose" v-model="formData.purpose" type="text"
          class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          placeholder="e.g., Save for a new car" required @blur="handleBlur('purpose', formData.purpose)" />
      </div>

      <!-- Timeframe Input -->
      <div class="form-group">
        <label for="timeframe" class="block text-sm font-medium text-gray-700 mb-1">
          Timeframe
        </label>
        <input id="timeframe" v-model="formData.timeframe" type="text"
          class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          placeholder="e.g., 12 months" required @blur="handleBlur('timeframe', formData.timeframe)" />
      </div>

      <!-- Total Amount Input -->
      <div class="form-group">
        <label for="total_amount" class="block text-sm font-medium text-gray-700 mb-1">
          Total Amount ($)
        </label>
        <input id="total_amount" v-model.number="formData.total_amount" type="number"
          class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          placeholder="10000" min="0" step="0.01" required @blur="handleBlur('amount', formData.total_amount)" />
      </div>

      <!-- Monthly Income Input -->
      <div class="form-group">
        <label for="monthly_income" class="block text-sm font-medium text-gray-700 mb-1">
          Monthly Income ($)
        </label>
        <input id="monthly_income" v-model.number="formData.total_income_monthly" type="number"
          class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          placeholder="2500" min="0" step="0.01" required @blur="handleBlur('income', formData.total_income_monthly)" />
      </div>

      <!-- Monthly Expenses Input -->
      <div class="form-group">
        <label for="monthly_expenses" class="block text-sm font-medium text-gray-700 mb-1">
          Monthly Expenses ($)
        </label>
        <input id="monthly_expenses" v-model.number="formData.total_expenses_monthly" type="number"
          class="form-input block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          placeholder="2000" min="0" step="0.01" required @blur="handleBlur('expenses', formData.total_expenses_monthly)" />
      </div>

      <!-- Submit Button -->
      <button type="submit"
        class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
        :disabled="loading">
        <span v-if="loading" class="flex items-center">
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none"
            viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
          </svg>
          Processing...
        </span>
        <span v-else>Submit Financial Goal</span>
      </button>
    </form>
  </div>
</template>

<style scoped>
.form-group {
  @apply space-y-1;
}

.form-input {
  @apply mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500;
}
</style>