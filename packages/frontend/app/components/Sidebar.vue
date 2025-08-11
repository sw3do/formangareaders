<template>
  <div>
    <div
      v-if="isOpen"
      class="fixed inset-0 z-55 transition-opacity duration-300"
      @click="closeSidebar"
    >
      <div class="fixed inset-0 bg-black/60 backdrop-blur-md" />
    </div>

    <aside
      :class="[
        'fixed top-0 left-0 h-full w-72 transform transition-all duration-500 ease-out',
        'z-60',
        isOpen ? 'translate-x-0 shadow-2xl' : '-translate-x-full'
      ]"
    >
      <div class="flex h-full flex-col bg-linear-to-b from-white via-gray-50/50 to-white dark:from-gray-900 dark:via-gray-800/50 dark:to-gray-900 border-r border-gray-200/50 dark:border-gray-700/50 backdrop-blur-xl">
        <div class="flex h-18 items-center justify-between px-6 py-4 border-b border-gray-200/50 dark:border-gray-700/50 bg-linear-to-r from-blue-50/30 to-purple-50/30 dark:from-blue-900/10 dark:to-purple-900/10">
          <NuxtLink to="/" class="flex items-center space-x-3 group" @click="closeSidebar">
            <div class="w-10 h-10 bg-linear-to-br from-blue-500 via-purple-500 to-indigo-600 rounded-xl flex items-center justify-center shadow-lg group-hover:shadow-xl transition-all duration-300 group-hover:scale-105">
              <Icon name="i-heroicons-book-open" class="w-6 h-6 text-white" />
            </div>
            <span class="text-lg font-bold bg-linear-to-r from-gray-900 to-gray-700 dark:from-white dark:to-gray-200 bg-clip-text text-transparent group-hover:from-blue-600 group-hover:to-purple-600 transition-all duration-300">
              ForManga
            </span>
          </NuxtLink>
          
          <UButton
            icon="i-heroicons-x-mark"
            variant="ghost"
            color="neutral"
            size="sm"
            class="hover:bg-red-50 dark:hover:bg-red-900/20 hover:text-red-600 dark:hover:text-red-400 transition-all duration-200 hover:scale-105 rounded-xl"
            @click="closeSidebar"
          />
        </div>

        <nav class="flex-1 overflow-y-auto px-6 py-6 scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600 scrollbar-track-transparent">
          <div class="space-y-8">
            <div>
              <h3 class="mb-4 text-xs font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider flex items-center">
                <div class="w-2 h-2 bg-linear-to-r from-blue-500 to-purple-500 rounded-full mr-2"></div>
                {{ $t('sidebar.navigation') }}
              </h3>
              <ul class="space-y-2">
                <li v-for="item in navigationItems" :key="item.name">
                  <NuxtLink
                    :to="item.href"
                    class="group flex items-center px-4 py-3 text-sm font-medium rounded-xl transition-all duration-300 hover:bg-linear-to-r hover:from-blue-50 hover:to-indigo-50 dark:hover:from-blue-900/20 dark:hover:to-indigo-900/20 hover:shadow-md hover:scale-105 relative overflow-hidden"
                    active-class="bg-linear-to-r from-blue-50 to-indigo-50 dark:from-blue-900/20 dark:to-indigo-900/20 text-blue-600 dark:text-blue-400 shadow-md scale-105"
                    @click="closeSidebar"
                  >
                    <div class="absolute inset-0 bg-linear-to-r from-blue-500/5 to-purple-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                    <Icon
                      :name="item.icon"
                      class="mr-4 h-5 w-5 text-gray-400 group-hover:text-blue-500 dark:text-gray-500 dark:group-hover:text-blue-400 transition-all duration-300 relative z-10"
                    />
                    <span class="relative z-10">{{ $t(item.name) }}</span>
                  </NuxtLink>
                </li>
              </ul>
            </div>

            <div v-if="libraryItems.length > 0">
              <h3 class="mb-4 text-xs font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider flex items-center">
                <div class="w-2 h-2 bg-linear-to-r from-green-500 to-emerald-500 rounded-full mr-2"></div>
                {{ $t('sidebar.library') }}
              </h3>
              <ul class="space-y-2">
                <li v-for="item in libraryItems" :key="item.name">
                  <NuxtLink
                    :to="item.href"
                    class="group flex items-center px-4 py-3 text-sm font-medium rounded-xl transition-all duration-300 hover:bg-linear-to-r hover:from-green-50 hover:to-emerald-50 dark:hover:from-green-900/20 dark:hover:to-emerald-900/20 hover:shadow-md hover:scale-105 relative overflow-hidden"
                    active-class="bg-linear-to-r from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 text-green-600 dark:text-green-400 shadow-md scale-105"
                    @click="closeSidebar"
                  >
                    <div class="absolute inset-0 bg-linear-to-r from-green-500/5 to-emerald-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                    <Icon
                      :name="item.icon"
                      class="mr-4 h-5 w-5 text-gray-400 group-hover:text-green-500 dark:text-gray-500 dark:group-hover:text-green-400 transition-all duration-300 relative z-10"
                    />
                    <span class="relative z-10 flex-1">{{ item.name }}</span>
                    <span
                      v-if="item.count"
                      class="ml-auto inline-flex items-center justify-center px-2.5 py-1 text-xs font-bold leading-none text-green-600 dark:text-green-400 bg-linear-to-r from-green-100 to-emerald-100 dark:from-green-900/30 dark:to-emerald-900/30 rounded-full shadow-sm relative z-10"
                    >
                      {{ item.count }}
                    </span>
                  </NuxtLink>
                </li>
              </ul>
            </div>

            <div v-if="categories.length > 0">
              <h3 class="mb-4 text-xs font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider flex items-center">
                <div class="w-2 h-2 bg-linear-to-r from-purple-500 to-pink-500 rounded-full mr-2"></div>
                {{ $t('sidebar.categories') }}
              </h3>
              <ul class="space-y-2">
                <li v-for="category in categories" :key="category.name">
                  <NuxtLink
                    :to="category.href"
                    class="group flex items-center px-4 py-3 text-sm font-medium rounded-xl transition-all duration-300 hover:bg-linear-to-r hover:from-purple-50 hover:to-pink-50 dark:hover:from-purple-900/20 dark:hover:to-pink-900/20 hover:shadow-md hover:scale-105 relative overflow-hidden"
                    active-class="bg-linear-to-r from-purple-50 to-pink-50 dark:from-purple-900/20 dark:to-pink-900/20 text-purple-600 dark:text-purple-400 shadow-md scale-105"
                    @click="closeSidebar"
                  >
                    <div class="absolute inset-0 bg-linear-to-r from-purple-500/5 to-pink-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                    <div
                      :class="[
                        'mr-4 h-3 w-3 rounded-full shadow-sm transition-all duration-300 group-hover:scale-110 relative z-10',
                        category.color
                      ]"
                    />
                    <span class="relative z-10">{{ $t(category.name) }}</span>
                  </NuxtLink>
                </li>
              </ul>
            </div>
          </div>
        </nav>

        <div class="border-t border-gray-200/50 dark:border-gray-700/50 p-6 bg-linear-to-r from-gray-50/50 to-gray-100/50 dark:from-gray-800/50 dark:to-gray-900/50">
          <div class="flex items-center space-x-4 p-3 rounded-xl bg-white/60 dark:bg-gray-800/60 backdrop-blur-sm border border-gray-200/30 dark:border-gray-700/30 shadow-sm hover:shadow-md transition-all duration-300">
            <div class="relative">
              <UAvatar
                src="https://avatars.githubusercontent.com/u/739984?v=4"
                alt="User avatar"
                size="sm"
                class="ring-2 ring-blue-200 dark:ring-blue-800 transition-all duration-300"
              />
              <div class="absolute -top-1 -right-1 w-3 h-3 bg-green-400 rounded-full border-2 border-white dark:border-gray-800 animate-pulse"></div>
            </div>
            <div class="flex-1 min-w-0">
              <p class="text-sm font-semibold text-gray-900 dark:text-white truncate">
                John Doe
              </p>
              <p class="text-xs text-gray-500 dark:text-gray-400 truncate">
                john@example.com
              </p>
            </div>
            <UPopover>
              <UButton
                icon="i-heroicons-ellipsis-vertical"
                variant="ghost"
                color="neutral"
                size="xs"
                class="hover:bg-gray-100 dark:hover:bg-gray-700 transition-all duration-200 hover:scale-105 rounded-lg"
              />
              <template #panel>
                <div class="p-3 w-44 bg-white/95 dark:bg-gray-900/95 backdrop-blur-xl border border-gray-200/50 dark:border-gray-700/50 rounded-2xl shadow-xl">
                  <div class="space-y-2">
                    <UButton
                      variant="ghost"
                      color="neutral"
                      size="sm"
                      class="w-full justify-start hover:bg-blue-50 dark:hover:bg-blue-900/20 hover:text-blue-600 dark:hover:text-blue-400 transition-all duration-200 rounded-xl"
                      @click="$router.push('/profile')"
                    >
                      <Icon name="i-heroicons-user" class="mr-3 h-4 w-4" />
                      {{ $t('nav.profile') }}
                    </UButton>
                    <UButton
                      variant="ghost"
                      color="neutral"
                      size="sm"
                      class="w-full justify-start hover:bg-gray-50 dark:hover:bg-gray-800 transition-all duration-200 rounded-xl"
                      @click="$router.push('/settings')"
                    >
                      <Icon name="i-heroicons-cog-6-tooth" class="mr-3 h-4 w-4" />
                      {{ $t('nav.settings') }}
                    </UButton>
                    <div class="border-t border-gray-100 dark:border-gray-800 my-2"></div>
                    <UButton
                      variant="ghost"
                      color="neutral"
                      size="sm"
                      class="w-full justify-start hover:bg-red-50 dark:hover:bg-red-900/20 hover:text-red-600 dark:hover:text-red-400 transition-all duration-200 rounded-xl"
                      @click="signOut"
                    >
                      <Icon name="i-heroicons-arrow-right-on-rectangle" class="mr-3 h-4 w-4" />
                      {{ $t('nav.signOut') }}
                    </UButton>
                  </div>
                </div>
              </template>
            </UPopover>
          </div>
        </div>
      </div>
    </aside>
  </div>
</template>

<script setup lang="ts">
interface NavigationItem {
  name: string
  href: string
  icon: string
  count?: number
}

interface Category {
  name: string
  href: string
  color: string
}

const props = defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const { $router } = useNuxtApp()

const navigationItems: NavigationItem[] = [
  { name: 'nav.home', href: '/', icon: 'i-heroicons-home' },
]

const libraryItems: NavigationItem[] = [

]

const categories: Category[] = [

]



const closeSidebar = () => {
  emit('close')
}

const signOut = () => {
  // Handle sign out logic
}


</script>