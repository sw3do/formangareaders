import { defineStore } from 'pinia'
import type { User, AuthResponse, LoginRequest, RegisterRequest } from '~/types/auth'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const isAuthenticated = computed(() => !!token.value && !!user.value)
  const isAdmin = computed(() => user.value?.role === 'admin')
  const isModerator = computed(() => user.value?.role === 'moderator' || isAdmin.value)

  const { apiCall } = useApi()

  const setAuth = (authData: AuthResponse) => {
    user.value = authData.user
    token.value = authData.token
    
    const tokenCookie = useCookie<string | null>('auth-token', {
      default: () => null,
      maxAge: 60 * 60 * 24 * 7,
      secure: true,
      sameSite: 'strict'
    })
    tokenCookie.value = authData.token
    

  }

  const clearAuth = () => {
    user.value = null
    token.value = null
    error.value = null
    
    const tokenCookie = useCookie<string | null>('auth-token')
    tokenCookie.value = null
    
    navigateTo('/auth/login')
  }

  const login = async (credentials: LoginRequest) => {
    try {
      isLoading.value = true
      error.value = null
      
      const response = await apiCall<AuthResponse>('/auth/login', {
        method: 'POST',
        data: credentials
      })
      
      setAuth(response)
      await navigateTo('/')
      
      return response
    } catch (err: any) {
      error.value = err.data?.message || 'Login failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const register = async (userData: RegisterRequest) => {
    try {
      isLoading.value = true
      error.value = null
      
      const response = await apiCall<{ user: User }>('/auth/register', {
        method: 'POST',
        data: userData
      })
      
      return response
    } catch (err: any) {
      error.value = err.data?.message || 'Registration failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const logout = async () => {
    try {
      if (token.value) {
        await apiCall('/auth/logout', {
          method: 'POST'
        })
      }
    } catch (err) {
      console.error('Logout error:', err)
    } finally {
      clearAuth()
    }
  }

  const fetchUser = async () => {
    try {
      if (!token.value) return
      
      const response = await apiCall<User>('/auth/me')
      user.value = response
      

      
      return response
    } catch (err: any) {
      console.error('fetchUser error:', err)
      if (err.response?.status === 401) {
        clearAuth()
      }
      throw err
    }
  }

  const verifyEmail = async (verificationToken: string) => {
    try {
      isLoading.value = true
      error.value = null
      
      await apiCall('/auth/verify-email', {
        method: 'POST',
        data: { token: verificationToken }
      })
      
      return true
    } catch (err: any) {
      error.value = err.data?.message || 'Email verification failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const resendVerification = async (email: string) => {
    try {
      isLoading.value = true
      error.value = null
      
      await apiCall('/auth/resend-verification', {
        method: 'POST',
        data: { email }
      })
      
      return true
    } catch (err: any) {
      error.value = err.data?.message || 'Failed to resend verification email'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const forgotPassword = async (email: string) => {
    try {
      isLoading.value = true
      error.value = null
      
      await apiCall('/auth/forgot-password', {
        method: 'POST',
        data: { email }
      })
      
      return true
    } catch (err: any) {
      error.value = err.data?.message || 'Failed to send password reset email'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const resetPassword = async (resetToken: string, newPassword: string) => {
    try {
      isLoading.value = true
      error.value = null
      
      await apiCall('/auth/reset-password', {
        method: 'POST',
        data: { token: resetToken, new_password: newPassword }
      })
      
      return true
    } catch (err: any) {
      error.value = err.data?.message || 'Password reset failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const initializeAuth = async () => {
    const tokenCookie = useCookie('auth-token')
    if (tokenCookie.value) {
      token.value = tokenCookie.value
      try {
        await fetchUser()
      } catch (err: any) {
        console.error('initializeAuth error:', err)
        if (err.response?.status === 401) {
          clearAuth()
        }
      }
    }
  }

  return {
    user,
    token,
    isLoading,
    error,
    isAuthenticated,
    isAdmin,
    isModerator,
    setAuth,
    login,
    register,
    logout,
    fetchUser,
    verifyEmail,
    resendVerification,
    forgotPassword,
    resetPassword,
    initializeAuth,
    clearAuth
  }
})