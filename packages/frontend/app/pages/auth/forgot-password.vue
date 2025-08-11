<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900 dark:text-white">
          {{ t('auth.forgotPassword.title') }}
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400">
          {{ t('auth.forgotPassword.subtitle') }}
        </p>
      </div>
      
      <form v-if="!emailSent" class="mt-8 space-y-6" @submit.prevent="handleForgotPassword">
        <div>
          <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
            {{ t('auth.email') }}
          </label>
          <input
            id="email"
            v-model="email"
            name="email"
            type="email"
            autocomplete="email"
            required
            class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
            :placeholder="t('auth.email')"
          >
        </div>

        <div v-if="authStore.error" class="text-red-600 dark:text-red-400 text-sm text-center">
          {{ authStore.error }}
        </div>

        <div>
          <button
            type="submit"
            :disabled="authStore.isLoading"
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="authStore.isLoading" class="absolute left-0 inset-y-0 flex items-center pl-3">
              <Icon name="heroicons:arrow-path" class="h-5 w-5 text-indigo-500 group-hover:text-indigo-400 animate-spin" />
            </span>
            {{ authStore.isLoading ? t('auth.sending') : t('auth.forgotPassword.button') }}
          </button>
        </div>
      </form>
      
      <div v-else class="text-center space-y-4">
        <Icon name="heroicons:check-circle" class="h-16 w-16 text-green-600 mx-auto" />
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          {{ t('auth.forgotPassword.emailSent') }}
        </h3>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          {{ t('auth.forgotPassword.checkEmail') }}
        </p>
      </div>

      <div class="text-center">
        <NuxtLink
          to="/auth/login"
          class="font-medium text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300"
        >
          {{ t('auth.backToLogin') }}
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'

definePageMeta({
  middleware: 'guest',
  layout: false
})

const authStore = useAuthStore()
const { t } = useI18n()

const email = ref('')
const emailSent = ref(false)

const handleForgotPassword = async () => {
  try {
    await authStore.forgotPassword(email.value)
    emailSent.value = true
  } catch (error) {
    console.error('Forgot password failed:', error)
  }
}
</script>