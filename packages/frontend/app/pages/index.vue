<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <nav class="bg-white dark:bg-gray-800 shadow">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <h1 class="text-xl font-bold text-indigo-600 dark:text-indigo-400">
              {{ config.public.appName }}
            </h1>
          </div>
          
          <div class="flex items-center space-x-4">
            <template v-if="authStore.isAuthenticated">
              <span class="text-sm text-gray-700 dark:text-gray-300">
                {{ t('welcome') }}, {{ authStore.user?.display_name || authStore.user?.username }}!
              </span>
              <NuxtLink
                to="/profile"
                class="text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300 text-sm font-medium"
              >
                {{ t('profile.title') }}
              </NuxtLink>
              <button
                @click="logout"
                class="bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-2 rounded-md text-sm font-medium"
              >
                {{ t('auth.logout') }}
              </button>
            </template>
            
            <template v-else>
              <NuxtLink
                to="/auth/login"
                class="text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300 text-sm font-medium"
              >
                {{ t('auth.login.link') }}
              </NuxtLink>
              <NuxtLink
                to="/auth/register"
                class="bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-2 rounded-md text-sm font-medium"
              >
                {{ t('auth.register.link') }}
              </NuxtLink>
            </template>
          </div>
        </div>
      </div>
    </nav>
    
    <main class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
      <div class="text-center">
        <h1 class="text-4xl font-extrabold text-gray-900 dark:text-white sm:text-5xl">
          {{ t('home.welcome') }}
        </h1>
        <p class="mt-4 text-xl text-gray-600 dark:text-gray-400">
          {{ t('home.subtitle') }}
        </p>
        
        <div v-if="!authStore.isAuthenticated" class="mt-8 space-x-4">
          <NuxtLink
            to="/auth/register"
            class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
          >
            {{ t('home.getStarted') }}
          </NuxtLink>
          <NuxtLink
            to="/auth/login"
            class="inline-flex items-center px-6 py-3 border border-gray-300 dark:border-gray-600 text-base font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            {{ t('auth.login.link') }}
          </NuxtLink>
        </div>
        
        <div v-else class="mt-8">
          <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6 max-w-md mx-auto">
            <div class="flex items-center space-x-4">
              <div class="h-12 w-12 rounded-full bg-indigo-600 flex items-center justify-center">
                <span class="text-lg font-medium text-white">
                  {{ userInitials }}
                </span>
              </div>
              <div class="flex-1 text-left">
                <h3 class="text-lg font-medium text-gray-900 dark:text-white">
                  {{ authStore.user?.display_name || authStore.user?.username }}
                </h3>
                <p class="text-sm text-gray-500 dark:text-gray-400">
                  {{ authStore.user?.email }}
                </p>
                <div class="mt-1">
                  <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium" :class="roleClasses">
                    {{ t(`auth.roles.${authStore.user?.role.toLowerCase()}`) }}
                  </span>
                </div>
              </div>
            </div>
            
            <div class="mt-4">
              <NuxtLink
                to="/profile"
                class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                {{ t('profile.viewProfile') }}
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'

const authStore = useAuthStore()
const config = useRuntimeConfig()
const { t } = useI18n()

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

const logout = async () => {
  await authStore.logout()
}
</script>