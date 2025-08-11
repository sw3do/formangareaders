<template>
  <UApp>
    <ClientOnly>
      <AppLayout>
        <NuxtPage />
      </AppLayout>
      <template #fallback>
        <div class="min-h-screen bg-gray-50 dark:bg-gray-950 flex items-center justify-center relative">
          <div class="absolute inset-0 bg-linear-to-br from-blue-50/30 via-purple-50/20 to-indigo-50/30 dark:from-blue-900/10 dark:via-purple-900/5 dark:to-indigo-900/10"></div>
          <div class="relative z-10 text-center space-y-6 px-4">
            <div class="relative mx-auto w-20 h-20">
              <div class="absolute inset-0 border-4 border-blue-200/40 dark:border-blue-800/40 rounded-full animate-spin">
                <div class="absolute top-0 left-0 w-full h-full border-4 border-transparent border-t-blue-600 dark:border-t-blue-400 rounded-full animate-spin" style="animation-duration: 0.8s;"></div>
              </div>
              <div class="absolute inset-2 border-2 border-purple-200/30 dark:border-purple-800/30 rounded-full animate-spin" style="animation-direction: reverse; animation-duration: 1.2s;">
                <div class="absolute top-0 left-0 w-full h-full border-2 border-transparent border-t-purple-500 dark:border-t-purple-400 rounded-full animate-spin" style="animation-direction: reverse; animation-duration: 1.2s;"></div>
              </div>
            </div>
            <div class="space-y-3">
              <h2 class="text-2xl font-bold bg-linear-to-r from-gray-900 to-gray-700 dark:from-white dark:to-gray-200 bg-clip-text text-transparent">
                {{ $t('common.loading') }}
              </h2>
              <p class="text-base text-gray-600 dark:text-gray-400 max-w-md mx-auto">
                {{ $t('common.appPreparing') }}
              </p>
            </div>
            <div class="flex justify-center space-x-1">
              <div class="w-2 h-2 bg-blue-500 rounded-full animate-bounce" style="animation-delay: 0s;"></div>
              <div class="w-2 h-2 bg-purple-500 rounded-full animate-bounce" style="animation-delay: 0.1s;"></div>
              <div class="w-2 h-2 bg-indigo-500 rounded-full animate-bounce" style="animation-delay: 0.2s;"></div>
            </div>
          </div>
        </div>
      </template>
    </ClientOnly>
  </UApp>
</template>
