import { useAuthStore } from '~/stores/auth'

export default defineNuxtRouteMiddleware(async () => {
  const authStore = useAuthStore()
  
  if (!authStore.isAuthenticated) {
    const tokenCookie = useCookie('auth-token')
    if (tokenCookie.value) {
      try {
        await authStore.initializeAuth()
        if (authStore.isAuthenticated) {
          return navigateTo('/')
        }
      } catch (error) {
        
      }
    }
  } else {
    return navigateTo('/')
  }
})