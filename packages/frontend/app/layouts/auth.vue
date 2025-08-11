<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <div class="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div class="sm:mx-auto sm:w-full sm:max-w-md">
        <div class="text-center">
          <NuxtLink to="/" class="inline-block">
            <h1 class="text-3xl font-bold text-indigo-600 dark:text-indigo-400">
              {{ config.public.appName }}
            </h1>
          </NuxtLink>
        </div>
      </div>
      
      <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
        <div class="bg-white dark:bg-gray-800 py-8 px-4 shadow sm:rounded-lg sm:px-10">
          <slot />
        </div>
      </div>
      
      <div class="mt-8 text-center">
        <div class="flex items-center justify-center space-x-4 text-sm text-gray-500 dark:text-gray-400">
          <NuxtLink to="/privacy" class="hover:text-gray-700 dark:hover:text-gray-300">
            {{ t('footer.privacy') }}
          </NuxtLink>
          <span>•</span>
          <NuxtLink to="/terms" class="hover:text-gray-700 dark:hover:text-gray-300">
            {{ t('footer.terms') }}
          </NuxtLink>
          <span>•</span>
          <NuxtLink to="/contact" class="hover:text-gray-700 dark:hover:text-gray-300">
            {{ t('footer.contact') }}
          </NuxtLink>
        </div>
        <p class="mt-2 text-xs text-gray-400 dark:text-gray-500">
          © 2025 {{ config.public.appName }}. {{ t('footer.rights') }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig()
const { t } = useI18n()
</script>