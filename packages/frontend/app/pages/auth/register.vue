<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900 dark:text-white">
          {{ t('auth.register.title') }}
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400">
          {{ t('auth.register.subtitle') }}
        </p>
      </div>
      
      <form class="mt-8 space-y-6" @submit.prevent="handleRegister">
        <div class="space-y-4">
          <div>
            <label for="username" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ t('auth.username') }}
            </label>
            <input
              id="username"
              v-model="form.username"
              name="username"
              type="text"
              autocomplete="username"
              required
              class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
              :placeholder="t('auth.username')"
            >
          </div>
          
          <div>
            <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ t('auth.email') }}
            </label>
            <input
              id="email"
              v-model="form.email"
              name="email"
              type="email"
              autocomplete="email"
              required
              class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
              :placeholder="t('auth.email')"
            >
          </div>
          
          <div>
            <label for="display_name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ t('auth.displayName') }} ({{ t('auth.optional') }})
            </label>
            <input
              id="display_name"
              v-model="form.display_name"
              name="display_name"
              type="text"
              autocomplete="name"
              class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
              :placeholder="t('auth.displayName')"
            >
          </div>
          
          <div>
            <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ t('auth.password') }}
            </label>
            <input
              id="password"
              v-model="form.password"
              name="password"
              type="password"
              autocomplete="new-password"
              required
              class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm bg-white dark:bg-gray-800"
              :placeholder="t('auth.password')"
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
        
        <div v-if="registrationSuccess" class="text-green-600 dark:text-green-400 text-sm text-center">
          {{ t('auth.register.success') }}
        </div>

        <div>
          <button
            type="submit"
            :disabled="authStore.isLoading || passwordMismatch"
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="authStore.isLoading" class="absolute left-0 inset-y-0 flex items-center pl-3">
              <Icon name="heroicons:arrow-path" class="h-5 w-5 text-indigo-500 group-hover:text-indigo-400 animate-spin" />
            </span>
            {{ authStore.isLoading ? t('auth.registering') : t('auth.register.button') }}
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
            {{ t('auth.hasAccount') }}
            <NuxtLink
              to="/auth/login"
              class="font-medium text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300"
            >
              {{ t('auth.login.link') }}
            </NuxtLink>
          </span>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RegisterRequest } from '~/types/auth'
import { useAuthStore } from '~/stores/auth'

definePageMeta({
  middleware: 'guest',
  layout: false
})

const authStore = useAuthStore()
const runtimeConfig = useRuntimeConfig()
const { t, locale } = useI18n()

const form = reactive<RegisterRequest>({
  email: '',
  username: '',
  password: '',
  display_name: '',
  locale: locale.value
})

const confirmPassword = ref('')
const registrationSuccess = ref(false)

const passwordMismatch = computed(() => {
  return !!(confirmPassword.value && form.password !== confirmPassword.value)
})

const handleRegister = async () => {
  if (passwordMismatch.value) return
  
  try {
    await authStore.register(form)
    registrationSuccess.value = true
    
    setTimeout(() => {
      navigateTo('/auth/login')
    }, 2000)
  } catch (error) {
    console.error('Registration failed:', error)
  }
}
</script>