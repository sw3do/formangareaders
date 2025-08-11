// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";


export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  ssr: true,
  modules: [
    '@nuxt/image',
    '@nuxt/scripts',
    '@nuxt/ui',
    '@nuxtjs/color-mode',
    '@vueuse/nuxt',
    '@nuxtjs/i18n',
    '@nuxt/icon'
  ],
  icon: {
    collections: ['heroicons', 'lucide', 'tabler', 'material-symbols']
  },
  css: ['~/assets/css/main.css'],
  colorMode: {
    preference: 'system',
    fallback: 'light',
    classSuffix: '',
    storageKey: 'nuxt-color-mode'
  },
  i18n: {
    locales: [
      {
        code: 'en',
        name: 'English',
        file: 'en.json'
      },
      {
        code: 'tr',
        name: 'Türkçe',
        file: 'tr.json'
      }
    ],
    defaultLocale: 'en',
    langDir: './locales/',
    strategy: 'prefix_except_default',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root',
      alwaysRedirect: false,
      fallbackLocale: 'en'
    }
  },
  runtimeConfig: {
    public: {
      appName: process.env.APP_NAME || 'ForMangaReaders',
      apiUrl: process.env.API_URL || 'http://localhost:8080/api/v1'
    }
  },
  vite: {
    plugins: [tailwindcss()]
  }
})