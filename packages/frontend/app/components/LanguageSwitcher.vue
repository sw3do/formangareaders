<template>
  <ClientOnly>
    <UPopover mode="click" :popper="{ placement: 'bottom-end', strategy: 'fixed' }">
      <UButton
        :icon="currentLocale === 'tr' ? 'i-heroicons-language' : 'i-heroicons-globe-alt'"
        variant="ghost"
        color="neutral"
        size="sm"
        :label="showLabel ? currentLocaleName : undefined"
        class="hover:bg-indigo-50 dark:hover:bg-indigo-900/20 hover:text-indigo-600 dark:hover:text-indigo-400 transition-all duration-300 hover:scale-105 rounded-xl font-medium min-w-0"
      />
      <template #content>
        <div class="p-3 w-36 sm:w-44">
          <div class="space-y-2">
            <div class="px-3 py-2 border-b border-gray-100 dark:border-gray-800">
               <div class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider flex items-center">
                 <div class="w-2 h-2 bg-linear-to-r from-indigo-500 to-purple-500 rounded-full mr-2"></div>
                 {{ $t('common.language') }}
               </div>
             </div>
            <UButton
              v-for="locale in availableLocales"
              :key="locale.code"
              variant="ghost"
              color="neutral"
              size="sm"
              class="w-full justify-start transition-all duration-200 rounded-xl hover:scale-105"
              :class="{
                'bg-linear-to-r from-indigo-50 to-purple-50 dark:from-indigo-900/20 dark:to-purple-900/20 text-indigo-600 dark:text-indigo-400 shadow-sm scale-105': locale.code === currentLocale,
                'hover:bg-linear-to-r hover:from-indigo-50/50 hover:to-purple-50/50 dark:hover:from-indigo-900/10 dark:hover:to-purple-900/10 hover:text-indigo-600 dark:hover:text-indigo-400': locale.code !== currentLocale
              }"
              @click="switchLanguage(locale.code)"
            >
              <div class="flex items-center w-full">
                <div class="flex items-center justify-center w-8 h-8 rounded-lg mr-3 transition-all duration-300"
                     :class="{
                       'bg-linear-to-br from-indigo-500 to-purple-500 text-white shadow-md': locale.code === currentLocale,
                       'bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400': locale.code !== currentLocale
                     }">
                  <Icon
                    :name="locale.code === 'tr' ? 'i-heroicons-language' : 'i-heroicons-globe-alt'"
                    class="h-4 w-4"
                  />
                </div>
                <div class="flex-1">
                  <p class="font-medium">{{ locale.name }}</p>
                  <p class="text-xs text-gray-500 dark:text-gray-400">{{ locale.code.toUpperCase() }}</p>
                </div>
                <div v-if="locale.code === currentLocale" class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
              </div>
            </UButton>
          </div>
        </div>
      </template>
    </UPopover>
    <template #fallback>
      <UButton
        icon="i-heroicons-globe-alt"
        variant="ghost"
        color="neutral"
        size="sm"
        label="EN"
        class="rounded-xl"
        disabled
      />
    </template>
  </ClientOnly>
</template>

<script setup lang="ts">
type LocaleCode = 'tr' | 'en'

const { locale, locales } = useI18n()
const switchLocalePath = useSwitchLocalePath()

const currentLocale = computed(() => locale.value)

const availableLocales = computed(() => {
  return locales.value as Array<{ code: string; name: string }>
})

const currentLocaleName = computed(() => {
  const current = locales.value.find((l: any) => l.code === locale.value)
  return current?.name || 'Language'
})

const showLabel = ref(true)

const switchLanguage = async (code: string) => {
  try {
    await navigateTo(switchLocalePath(code as LocaleCode))
  } catch (error) {
    console.error('Language switching failed:', error)
  }
}

onMounted(() => {
  const updateShowLabel = () => {
    showLabel.value = window.innerWidth >= 640
  }
  
  updateShowLabel()
  window.addEventListener('resize', updateShowLabel)
  
  onUnmounted(() => {
    window.removeEventListener('resize', updateShowLabel)
  })
})
</script>