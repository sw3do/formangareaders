<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900">
    <div class="max-w-md w-full text-center">
      <div v-if="isLoading" class="space-y-4">
        <Icon name="heroicons:arrow-path" class="h-12 w-12 text-indigo-600 animate-spin mx-auto" />
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {{ t('auth.processingLogin') }}
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          {{ t('auth.pleaseWait') }}
        </p>
      </div>
      
      <div v-else-if="error" class="space-y-4">
        <Icon name="heroicons:x-circle" class="h-12 w-12 text-red-600 mx-auto" />
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {{ t('auth.loginFailed') }}
        </h2>
        <p class="text-red-600 dark:text-red-400">
          {{ error }}
        </p>
        <NuxtLink
          to="/auth/login"
          class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          {{ t('auth.backToLogin') }}
        </NuxtLink>
      </div>
      
      <div v-else class="space-y-4">
        <Icon name="heroicons:check-circle" class="h-12 w-12 text-green-600 mx-auto" />
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {{ t('auth.loginSuccess') }}
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          {{ t('auth.redirecting') }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AuthResponse } from '~/types/auth'
import { useAuthStore } from '~/stores/auth'

definePageMeta({
  layout: false
})

const authStore = useAuthStore()
const route = useRoute()
const { t } = useI18n()

const isLoading = ref(true)
const error = ref<string | null>(null)

onMounted(async () => {
  try {
    const token = route.query.token as string
    const userString = route.query.user as string
    
    if (!token || !userString) {
      throw new Error(t('auth.invalidCallback'))
    }
    
    const user = JSON.parse(decodeURIComponent(userString))
    
    const authResponse: AuthResponse = {
      token,
      user
    }
    
    authStore.setAuth(authResponse)
    
    setTimeout(() => {
      navigateTo('/')
    }, 1500)
    
  } catch (err: any) {
    error.value = err.message || t('auth.callbackError')
  } finally {
    isLoading.value = false
  }
})
</script>