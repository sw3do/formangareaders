import { useAuthStore } from '~/stores/auth'

export default defineNuxtRouteMiddleware(async (to) => {
  const authStore = useAuthStore()
  
  if (!authStore.isAuthenticated) {
    const tokenCookie = useCookie('auth-token')
    if (tokenCookie.value) {
      try {
        await authStore.initializeAuth()
      } catch (error) {
        return navigateTo('/auth/login')
      }
    } else {
      return navigateTo('/auth/login')
    }
  }
})