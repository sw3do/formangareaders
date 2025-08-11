<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div class="text-center">
        <div v-if="isVerifying" class="space-y-4">
          <Icon name="heroicons:arrow-path" class="h-16 w-16 text-indigo-600 mx-auto animate-spin" />
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            {{ t('auth.verifyEmail.verifying') }}
          </h2>
          <p class="text-gray-600 dark:text-gray-400">
            {{ t('auth.verifyEmail.pleaseWait') }}
          </p>
        </div>
        
        <div v-else-if="verificationSuccess" class="space-y-4">
          <Icon name="heroicons:check-circle" class="h-16 w-16 text-green-600 mx-auto" />
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            {{ t('auth.verifyEmail.success') }}
          </h2>
          <p class="text-gray-600 dark:text-gray-400">
            {{ t('auth.verifyEmail.successMessage') }}
          </p>
          <NuxtLink
            to="/auth/login"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          >
            {{ t('auth.login.link') }}
          </NuxtLink>
        </div>
        
        <div v-else class="space-y-4">
          <Icon name="heroicons:x-circle" class="h-16 w-16 text-red-600 mx-auto" />
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            {{ t('auth.verifyEmail.failed') }}
          </h2>
          <p class="text-gray-600 dark:text-gray-400">
            {{ authStore.error || t('auth.verifyEmail.failedMessage') }}
          </p>
          <div class="space-y-2">
            <button
              @click="resendVerification"
              :disabled="authStore.isLoading"
              class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <span v-if="authStore.isLoading" class="mr-2">
                <Icon name="heroicons:arrow-path" class="h-4 w-4 animate-spin" />
              </span>
              {{ t('auth.verifyEmail.resend') }}
            </button>
            <NuxtLink
              to="/auth/login"
              class="w-full flex justify-center py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
              {{ t('auth.backToLogin') }}
            </NuxtLink>
          </div>
        </div>
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

const isVerifying = ref(true)
const verificationSuccess = ref(false)
const userEmail = ref('')

const token = computed(() => route.query.token as string)

onMounted(async () => {
  if (!token.value) {
    isVerifying.value = false
    return
  }
  
  try {
    await authStore.verifyEmail(token.value)
    verificationSuccess.value = true
  } catch (error) {
    console.error('Email verification failed:', error)
    verificationSuccess.value = false
  } finally {
    isVerifying.value = false
  }
})

const resendVerification = async () => {
  if (!userEmail.value) {
    const email = prompt(t('auth.verifyEmail.enterEmail'))
    if (!email) return
    userEmail.value = email
  }
  
  try {
    await authStore.resendVerification(userEmail.value)
  } catch (error) {
    console.error('Resend verification failed:', error)
  }
}
</script>