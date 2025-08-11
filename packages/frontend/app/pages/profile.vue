<template>
  <div class="max-w-4xl mx-auto py-8 px-4 sm:px-6 lg:px-8">
    <div class="bg-white dark:bg-gray-800 shadow rounded-lg">
      <div class="px-4 py-5 sm:p-6">
        <div class="flex items-center space-x-5">
          <div class="shrink-0">
            <div class="h-20 w-20 rounded-full bg-indigo-600 flex items-center justify-center">
              <span class="text-2xl font-medium text-white">
                {{ userInitials }}
              </span>
            </div>
          </div>
          <div class="flex-1 min-w-0">
            <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
              {{ authStore.user?.display_name || authStore.user?.username }}
            </h1>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {{ authStore.user?.email }}
            </p>
            <div class="mt-1 flex items-center space-x-2">
              <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="roleClasses">
                {{ t(`auth.roles.${authStore.user?.role.toLowerCase()}`) }}
              </span>
              <span v-if="authStore.user?.is_verified" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100">
                <Icon name="heroicons:check-circle" class="w-3 h-3 mr-1" />
                {{ t('auth.verified') }}
              </span>
              <span v-else class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-100">
                <Icon name="heroicons:exclamation-triangle" class="w-3 h-3 mr-1" />
                {{ t('auth.unverified') }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="mt-8 grid grid-cols-1 gap-6 lg:grid-cols-2">
      <div class="bg-white dark:bg-gray-800 shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg leading-6 font-medium text-gray-900 dark:text-white">
            {{ t('profile.accountInfo') }}
          </h3>
          <div class="mt-5 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                {{ t('auth.username') }}
              </label>
              <p class="mt-1 text-sm text-gray-900 dark:text-white">
                {{ authStore.user?.username }}
              </p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                {{ t('auth.displayName') }}
              </label>
              <p class="mt-1 text-sm text-gray-900 dark:text-white">
                {{ authStore.user?.display_name || t('profile.notSet') }}
              </p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                {{ t('profile.memberSince') }}
              </label>
              <p class="mt-1 text-sm text-gray-900 dark:text-white">
                {{ formatDate(authStore.user?.created_at) }}
              </p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                {{ t('profile.language') }}
              </label>
              <div class="mt-1">
                <select
                  v-model="selectedLocale"
                  @change="updateUserLocale"
                  class="block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
                  <option value="en">English</option>
                  <option value="tr">Türkçe</option>
                </select>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 shadow rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <h3 class="text-lg leading-6 font-medium text-gray-900 dark:text-white">
            {{ t('profile.actions') }}
          </h3>
          <div class="mt-5 space-y-3">
            <button
              v-if="!authStore.user?.is_verified"
              @click="resendVerification"
              :disabled="authStore.isLoading"
              class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <span v-if="authStore.isLoading" class="mr-2">
                <Icon name="heroicons:arrow-path" class="h-4 w-4 animate-spin" />
              </span>
              {{ t('auth.verifyEmail.resend') }}
            </button>
            
            <button
              v-if="authStore.user?.provider === 'local'"
              @click="changePassword"
              class="w-full flex justify-center py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
              {{ t('profile.changePassword') }}
            </button>
            
            <div
              v-else
              class="w-full flex justify-center py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-500 dark:text-gray-400 bg-gray-50 dark:bg-gray-800"
            >
              <Icon name="heroicons:information-circle" class="w-4 h-4 mr-2" />
              {{ t('profile.oauthPasswordInfo') }}
            </div>
            
            <button
              @click="logout"
              class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
            >
              {{ t('auth.logout') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'
import { useApi } from '~/composables/useApi'

definePageMeta({
  middleware: 'auth'
})

const authStore = useAuthStore()
const { t, locale, setLocale } = useI18n()

const selectedLocale = ref(authStore.user?.locale || locale.value)

const userInitials = computed(() => {
  const user = authStore.user
  if (!user) return ''
  
  const displayName = user.display_name || user.username
  return displayName
    .split(' ')
    .map((name: string) => name.charAt(0).toUpperCase())
    .slice(0, 2)
    .join('')
})

const roleClasses = computed(() => {
  const role = authStore.user?.role
  switch (role) {
    case 'admin':
      return 'bg-red-100 text-red-800 dark:bg-red-800 dark:text-red-100'
    case 'moderator':
      return 'bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-100'
    default:
      return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300'
  }
})

const formatDate = (dateString?: string) => {
  if (!dateString) return t('profile.unknown')
  return new Date(dateString).toLocaleDateString()
}

const updateUserLocale = async () => {
  try {
    const { apiCall } = useApi()
    const response = await apiCall('/auth/update-locale', {
      method: 'POST',
      data: {
        locale: selectedLocale.value
      }
    })
    
    if (response) {
       setLocale(selectedLocale.value as 'en' | 'tr')
       await authStore.fetchUser()
     }
  } catch (error) {
    console.error('Failed to update locale:', error)
  }
}

const resendVerification = async () => {
  if (!authStore.user?.email) return
  
  try {
    await authStore.resendVerification(authStore.user.email)
  } catch (error) {
    console.error('Resend verification failed:', error)
  }
}

const changePassword = () => {
  navigateTo('/auth/forgot-password')
}

const logout = async () => {
  await authStore.logout()
}
</script>