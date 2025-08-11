<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900 dark:text-white">
          {{ t('auth.login.title') }}
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400">
          {{ t('auth.login.subtitle') }}
        </p>
      </div>
      
      <form class="mt-8 space-y-6" @submit.prevent="handleLogin">
        <div class="rounded-md shadow-sm -space-y-px">
          <div>
            <label for="email" class="sr-only">{{ t('auth.email') }}</label>
            <input
              id="email"
              v-model="form.email"
              name="email"
              type="email"
              autocomplete="email"
              required
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
              :placeholder="t('auth.email')"
            >
          </div>
          <div>
            <label for="password" class="sr-only">{{ t('auth.password') }}</label>
            <input
              id="password"
              v-model="form.password"
              name="password"
              type="password"
              autocomplete="current-password"
              required
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
              :placeholder="t('auth.password')"
            >
          </div>
        </div>

        <div class="flex items-center justify-between">
          <div class="flex items-center">
            <input
              id="remember-me"
              v-model="form.rememberMe"
              name="remember-me"
              type="checkbox"
              class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
            >
            <label for="remember-me" class="ml-2 block text-sm text-gray-900 dark:text-gray-300">
              {{ t('auth.rememberMe') }}
            </label>
          </div>

          <div class="text-sm">
            <NuxtLink
              to="/auth/forgot-password"
              class="font-medium text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300"
            >
              {{ t('auth.forgotPassword') }}
            </NuxtLink>
          </div>
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
            {{ authStore.isLoading ? t('auth.loggingIn') : t('auth.login.button') }}
          </button>
        </div>

        <div class="mt-6">
          <div class="relative">
            <div class="absolute inset-0 flex items-center">
              <div class="w-full border-t border-gray-300 dark:border-gray-600" />
            </div>
            <div class="relative flex justify-center text-sm">
              <span class="px-2 bg-gray-50 dark:bg-gray-900 text-gray-500 dark:text-gray-400">{{ t('auth.orContinueWith') }}</span>
            </div>
          </div>

          <div class="mt-6 grid grid-cols-2 gap-3">
            <a
              :href="`${runtimeConfig.public.apiUrl}/auth/google`"
              class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm bg-white dark:bg-gray-800 text-sm font-medium text-gray-500 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              <Icon name="lucide:chrome" class="h-5 w-5" />
              <span class="ml-2">Google</span>
            </a>

            <a
              :href="`${runtimeConfig.public.apiUrl}/auth/discord`"
              class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm bg-white dark:bg-gray-800 text-sm font-medium text-gray-500 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              <Icon name="lucide:message-circle" class="h-5 w-5" />
              <span class="ml-2">Discord</span>
            </a>
          </div>
        </div>

        <div class="text-center">
          <span class="text-sm text-gray-600 dark:text-gray-400">
            {{ t('auth.noAccount') }}
            <NuxtLink
              to="/auth/register"
              class="font-medium text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300"
            >
              {{ t('auth.register.link') }}
            </NuxtLink>
          </span>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { LoginRequest } from '~/types/auth'
import { useAuthStore } from '~/stores/auth'

definePageMeta({
  middleware: 'guest',
  layout: false
})

const authStore = useAuthStore()
const runtimeConfig = useRuntimeConfig()
const { t } = useI18n()

const form = reactive<LoginRequest & { rememberMe: boolean }>({
  email: '',
  password: '',
  rememberMe: false
})

const handleLogin = async () => {
  try {
    await authStore.login({
      email: form.email,
      password: form.password
    })
  } catch (error) {
    console.error('Login failed:', error)
  }
}
</script>