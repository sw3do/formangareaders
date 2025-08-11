<template>
  <nav
    class="sticky top-0 z-50 bg-white/80 dark:bg-gray-900/80 backdrop-blur-2xl border-b border-gray-200/30 dark:border-gray-700/30 shadow-lg shadow-gray-200/20 dark:shadow-gray-900/20">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center h-20">
        <div class="flex items-center space-x-3 sm:space-x-6">
          <UButton icon="i-heroicons-bars-3" variant="ghost" color="neutral" size="lg"
            class="hover:bg-linear-to-r hover:from-blue-50 hover:to-indigo-50 dark:hover:from-blue-900/20 dark:hover:to-indigo-900/20 hover:text-blue-600 dark:hover:text-blue-400 transition-all duration-300 hover:scale-110 rounded-2xl p-3"
            @click="toggleSidebar" />

          <NuxtLink to="/" class="flex items-center space-x-2 sm:space-x-4 group">
            <div class="relative">
              <div
                class="w-10 h-10 sm:w-12 sm:h-12 bg-linear-to-br from-blue-500 via-purple-500 to-indigo-600 rounded-2xl flex items-center justify-center shadow-xl group-hover:shadow-2xl transition-all duration-500 group-hover:scale-110 group-hover:rotate-3">
                <Icon name="i-heroicons-book-open" class="w-5 h-5 sm:w-7 sm:h-7 text-white" />
              </div>
              <div
                class="absolute -top-1 -right-1 w-3 h-3 sm:w-4 sm:h-4 bg-linear-to-r from-green-400 to-emerald-500 rounded-full border-2 border-white dark:border-gray-900 animate-pulse">
              </div>
            </div>
            <div class="hidden sm:block">
              <span
                class="text-xl sm:text-2xl font-black bg-linear-to-r from-gray-900 via-blue-800 to-purple-800 dark:from-white dark:via-blue-200 dark:to-purple-200 bg-clip-text text-transparent group-hover:from-blue-600 group-hover:via-purple-600 group-hover:to-indigo-600 transition-all duration-500">
                {{ runtimeConfig.appName }}
              </span>
            </div>
          </NuxtLink>
        </div>

        <div class="hidden lg:flex items-center space-x-1">
          <NuxtLink v-for="item in navigationItems" :key="item.name" :to="$localePath(item.href)"
            class="relative text-gray-700 dark:text-gray-300 hover:text-white px-6 py-3 rounded-2xl text-sm font-semibold transition-all duration-500 hover:bg-linear-to-r hover:from-blue-500 hover:via-purple-500 hover:to-indigo-500 hover:scale-110 hover:shadow-xl group overflow-hidden"
            active-class="text-white bg-linear-to-r from-blue-500 via-purple-500 to-indigo-500 shadow-xl scale-105">
            <span class="relative z-10">{{ $t(item.name) }}</span>
            <div
              class="absolute inset-0 bg-linear-to-r from-blue-500/20 via-purple-500/20 to-indigo-500/20 rounded-2xl opacity-0 group-hover:opacity-100 transition-all duration-500 blur-sm">
            </div>
          </NuxtLink>
        </div>

        <div class="flex items-center space-x-2 sm:space-x-4">
          <div class="relative hidden lg:block group">
            <UInput v-model="searchQuery" :placeholder="$t('nav.search')" icon="i-heroicons-magnifying-glass" size="md"
              class="w-64 xl:w-80 bg-linear-to-r from-gray-50/90 to-white/90 dark:from-gray-800/90 dark:to-gray-700/90 border-gray-200/60 dark:border-gray-600/60 focus:border-blue-400 dark:focus:border-blue-500 rounded-2xl transition-all duration-500 focus:shadow-2xl focus:scale-105 focus:bg-white dark:focus:bg-gray-800 backdrop-blur-xl">
              <template #trailing>
                <div class="flex items-center space-x-2">
                  <UKbd size="sm"
                    class="bg-linear-to-r from-gray-100 to-gray-200 dark:from-gray-700 dark:to-gray-600 text-gray-600 dark:text-gray-300 border-gray-300 dark:border-gray-600 rounded-lg shadow-sm">
                    ⌘K</UKbd>
                </div>
              </template>
            </UInput>
            <div
              class="absolute inset-0 bg-linear-to-r from-blue-500/10 to-purple-500/10 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none">
            </div>
          </div>

          <UButton icon="i-heroicons-magnifying-glass" variant="ghost" color="neutral" size="sm"
            class="lg:hidden hover:bg-linear-to-r hover:from-blue-50 hover:to-indigo-50 dark:hover:from-blue-900/20 dark:hover:to-indigo-900/20 hover:text-blue-600 dark:hover:text-blue-400 transition-all duration-300 hover:scale-110 rounded-2xl p-2 shadow-lg hover:shadow-xl"
            @click="toggleSearch" />

          <div
            class="relative z-50 bg-linear-to-r from-gray-100/60 to-gray-200/60 dark:from-gray-800/60 dark:to-gray-700/60 rounded-2xl p-1 sm:p-2 shadow-lg backdrop-blur-sm border border-gray-200/30 dark:border-gray-600/30">
            <LanguageSwitcher />
          </div>

          <ClientOnly>
            <UButton :icon="isDark ? 'i-heroicons-moon' : 'i-heroicons-sun'" variant="ghost" color="neutral" size="sm"
              class="hover:bg-linear-to-r hover:from-yellow-50 hover:to-orange-50 dark:hover:from-yellow-900/20 dark:hover:to-orange-900/20 hover:text-yellow-600 dark:hover:text-yellow-400 transition-all duration-500 hover:scale-110 hover:rotate-12 rounded-2xl p-2 sm:p-3 shadow-lg hover:shadow-xl"
              @click="toggleTheme" />
            <template #fallback>
              <UButton icon="i-heroicons-sun" variant="ghost" color="neutral" size="sm" class="rounded-2xl p-2 sm:p-3"
                disabled />
            </template>
          </ClientOnly>

          <UPopover>
            <div class="relative group">
              <div
                class="p-0.5 sm:p-1 bg-linear-to-r from-blue-500 via-purple-500 to-indigo-500 rounded-2xl shadow-xl group-hover:shadow-2xl transition-all duration-500 group-hover:scale-110">
                <UAvatar src="https://avatars.githubusercontent.com/u/739984?v=4" alt="User avatar" size="sm"
                  class="cursor-pointer ring-2 ring-white dark:ring-gray-900 transition-all duration-300" />
              </div>
              <div
                class="absolute -top-1 -right-1 w-3 h-3 sm:w-4 sm:h-4 bg-linear-to-r from-green-400 to-emerald-500 rounded-full border-2 border-white dark:border-gray-900 animate-pulse shadow-lg">
              </div>
            </div>
            <template #panel>
              <div
                class="p-4 w-64 bg-white/95 dark:bg-gray-900/95 backdrop-blur-2xl border border-gray-200/30 dark:border-gray-700/30 rounded-3xl shadow-2xl">
                <div class="space-y-3">
                  <div
                    class="px-4 py-3 border-b border-gray-200 dark:border-gray-700 rounded-2xl bg-linear-to-r from-gray-50/50 to-gray-100/50 dark:from-gray-800/50 dark:to-gray-700/50">
                    <p class="text-sm font-bold text-gray-900 dark:text-white">John Doe</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400 font-medium">john@example.com</p>
                  </div>
                  <UButton variant="ghost" color="neutral" size="md"
                    class="w-full justify-start hover:bg-linear-to-r hover:from-blue-50 hover:to-indigo-50 dark:hover:from-blue-900/20 dark:hover:to-indigo-900/20 hover:text-blue-600 dark:hover:text-blue-400 transition-all duration-300 rounded-2xl hover:scale-105 hover:shadow-lg"
                    @click="$router.push('/profile')">
                    <Icon name="i-heroicons-user" class="mr-3 h-5 w-5" />
                    {{ $t('nav.profile') }}
                  </UButton>
                  <UButton variant="ghost" color="neutral" size="md"
                    class="w-full justify-start hover:bg-linear-to-r hover:from-gray-50 hover:to-gray-100 dark:hover:from-gray-800 dark:hover:to-gray-700 transition-all duration-300 rounded-2xl hover:scale-105 hover:shadow-lg"
                    @click="$router.push('/settings')">
                    <Icon name="i-heroicons-cog-6-tooth" class="mr-3 h-5 w-5" />
                    {{ $t('nav.settings') }}
                  </UButton>
                  <div
                    class="border-t border-gradient-to-r from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 my-3">
                  </div>
                  <UButton variant="ghost" color="neutral" size="md"
                    class="w-full justify-start hover:bg-linear-to-r hover:from-red-50 hover:to-pink-50 dark:hover:from-red-900/20 dark:hover:to-pink-900/20 hover:text-red-600 dark:hover:text-red-400 transition-all duration-300 rounded-2xl hover:scale-105 hover:shadow-lg"
                    @click="signOut">
                    <Icon name="i-heroicons-arrow-right-on-rectangle" class="mr-3 h-5 w-5" />
                    {{ $t('nav.signOut') }}
                  </UButton>
                </div>
              </div>
            </template>
          </UPopover>
        </div>
      </div>

      <div v-if="showMobileSearch"
        class="md:hidden py-6 border-t border-gray-200/30 dark:border-gray-700/30 bg-linear-to-r from-gray-50/80 to-white/80 dark:from-gray-800/80 dark:to-gray-900/80 backdrop-blur-xl">
        <UInput v-model="searchQuery" :placeholder="$t('nav.search')" icon="i-heroicons-magnifying-glass" size="md"
          class="w-full bg-white/90 dark:bg-gray-800/90 border-gray-200/60 dark:border-gray-700/60 rounded-2xl focus:border-blue-400 dark:focus:border-blue-500 transition-all duration-500 focus:shadow-xl focus:scale-105 backdrop-blur-sm" />
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
const colorMode = useColorMode()
const { $router } = useNuxtApp()
const { public: runtimeConfig } = useRuntimeConfig()

const searchQuery = ref('')
const showMobileSearch = ref(false)

const isDark = computed({
  get() {
    return colorMode.value === 'dark'
  },
  set(value: boolean) {
    colorMode.preference = value ? 'dark' : 'light'
  }
})

const navigationItems = [
  { name: 'nav.home', href: '/' },
]



const emit = defineEmits<{
  toggleSidebar: []
}>()

const toggleSidebar = () => {
  emit('toggleSidebar')
}

const toggleTheme = () => {
  isDark.value = !isDark.value
}

const toggleSearch = () => {
  showMobileSearch.value = !showMobileSearch.value
}

const signOut = () => {
  // Handle sign out logic
}

onMounted(() => {
  const handleKeydown = (e: KeyboardEvent) => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault()
      const searchInput = document.querySelector('input[placeholder="Search manga..."]') as HTMLInputElement
      searchInput?.focus()
    }
  }

  document.addEventListener('keydown', handleKeydown)

  onUnmounted(() => {
    document.removeEventListener('keydown', handleKeydown)
  })
})
</script>