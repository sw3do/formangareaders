<template>
  <div class="container mx-auto px-4 py-8">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-4xl font-bold text-gray-900 dark:text-white">
        Manga Readers
      </h1>
      <ClientOnly v-if="!colorMode?.forced">
        <UButton
          :icon="isDark ? 'i-lucide-moon' : 'i-lucide-sun'"
          color="neutral"
          variant="ghost"
          @click="isDark = !isDark"
        />

        <template #fallback>
          <div class="size-8" />
        </template>
      </ClientOnly>
    </div>
    
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <UCard class="hover:shadow-lg transition-shadow duration-300">
        <template #header>
          <h3 class="text-xl font-semibold">Welcome</h3>
        </template>
        <p class="text-gray-600 dark:text-gray-300">
          This is your manga reading platform with beautiful dark and light themes.
        </p>
      </UCard>
      
      <UCard class="hover:shadow-lg transition-shadow duration-300">
        <template #header>
          <h3 class="text-xl font-semibold">Features</h3>
        </template>
        <ul class="text-gray-600 dark:text-gray-300 space-y-2">
          <li>• Dark/Light theme support</li>
          <li>• Responsive design</li>
          <li>• Modern UI components</li>
        </ul>
      </UCard>
      
      <UCard class="hover:shadow-lg transition-shadow duration-300">
        <template #header>
          <h3 class="text-xl font-semibold">Theme</h3>
        </template>
        <p class="text-gray-600 dark:text-gray-300">
          Current theme: <span class="font-semibold">{{ isDark ? 'Dark' : 'Light' }}</span>
        </p>
      </UCard>
    </div>
  </div>
</template>

<script setup lang="ts">
const colorMode = useColorMode()

const isDark = computed({
  get() {
    return colorMode.value === 'dark'
  },
  set(_isDark) {
    colorMode.preference = _isDark ? 'dark' : 'light'
  }
})
</script>