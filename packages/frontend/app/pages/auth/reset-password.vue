<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900 dark:text-white">
          {{ t('auth.resetPassword.title') }}
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400">
          {{ t('auth.resetPassword.subtitle') }}
        </p>
      </div>
      
      <form v-if="!resetSuccess" class="mt-8 space-y-6" @submit.prevent="handleResetPassword">
        <div class="space-y-4">
          <div>
            <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ t('auth.newPassword') }}
            </label>
            <input
              id="password"
              v-model="form.password"
              name="password"
              type="password"
              autocomplete="new-password"
              required
              class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
              :placeholder="t('auth.newPassword')"
            >
          </div>
          
          <div>
            <label for="confirm_password" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ t('auth.confirmPassword') }}
            </label>
            <input
              id="confirm_password"
              v-model="confirmPassword"
              name="confirm_password"
              type="password"
              autocomplete="new-password"
              required
              class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
              :placeholder="t('auth.confirmPassword')"
            >
          </div>
        </div>

        <div v-if="authStore.error" class="text-red-600 dark:text-red-400 text-sm text-center">
          {{ authStore.error }}
        </div>
        
        <div v-if="passwordMismatch" class="text-red-600 dark:text-red-400 text-sm text-center">
          {{ t('auth.passwordMismatch') }}
        </div>
        
        <div v-if="!token" class="text-red-600 dark:text-red-400 text-sm text-center">
          {{ t('auth.resetPassword.invalidToken') }}
        </div>

        <div>
          <button
            type="submit"
            :disabled="authStore.isLoading || passwordMismatch || !token"
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="authStore.isLoading" class="absolute left-0 inset-y-0 flex items-center pl-3">
              <Icon name="heroicons:arrow-path" class="h-5 w-5 text-indigo-500 group-hover:text-indigo-400 animate-spin" />
            </span>
            {{ authStore.isLoading ? t('auth.resetting') : t('auth.resetPassword.button') }}
          </button>
        </div>
      </form>
      
      <div v-else class="text-center space-y-4">
        <Icon name="heroicons:check-circle" class="h-16 w-16 text-green-600 mx-auto" />
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          {{ t('auth.resetPassword.success') }}
        </h3>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          {{ t('auth.resetPassword.successMessage') }}
        </p>
        <NuxtLink
          to="/auth/login"
          class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          {{ t('auth.login.link') }}
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
const route = useRoute()
const { t } = useI18n()

const token = computed(() => route.query.token as string)

const form = reactive({
  password: ''
})

const confirmPassword = ref('')
const resetSuccess = ref(false)

const passwordMismatch = computed(() => {
  return confirmPassword.value && form.password !== confirmPassword.value
})

const handleResetPassword = async () => {
  if (passwordMismatch.value || !token.value) return
  
  try {
    await authStore.resetPassword(token.value, form.password)
    resetSuccess.value = true
  } catch (error) {
    console.error('Reset password failed:', error)
  }
}
</script>