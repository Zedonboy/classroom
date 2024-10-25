<script setup lang="ts">
import { ref } from 'vue'
import FinancialGoalPlanner from "./components/FinancialGoal.vue"
import ForexWidget from './components/Forex.vue'

const activeTab = ref('financial-goal')

const switchTab = (tab: string) => {
  activeTab.value = tab
}
</script>

<template>
  <div class="max-w-4xl mx-auto p-6">
    <!-- Tab Navigation -->
    <div class="mb-6 border-b border-gray-200">
      <nav class="-mb-px flex space-x-8" aria-label="Tabs">
        <button
          @click="switchTab('financial-goal')"
          :class="[
            activeTab === 'financial-goal'
              ? 'border-indigo-500 text-indigo-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300',
            'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm'
          ]"
        >
          Financial Goal Planner
        </button>
        <button
          @click="switchTab('forex')"
          :class="[
            activeTab === 'forex'
              ? 'border-indigo-500 text-indigo-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300',
            'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm'
          ]"
        >
          Currency Exchange
        </button>
      </nav>
    </div>

    <!-- Tab Panels -->
    <div class="mt-6">
      <div v-show="activeTab === 'financial-goal'">
        <FinancialGoalPlanner />
      </div>
      <div v-show="activeTab === 'forex'">
        <ForexWidget />
      </div>
    </div>
  </div>
</template>