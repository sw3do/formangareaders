import axios, { type AxiosRequestConfig, type AxiosResponse } from 'axios'
import { useRuntimeConfig } from 'nuxt/app'

export const useApi = () => {
  const { public: runtimeConfig } = useRuntimeConfig()

  const apiInstance = axios.create({
    baseURL: runtimeConfig.apiUrl as string,
    headers: {
      'Content-Type': 'application/json'
    }
  })

  apiInstance.interceptors.request.use((config) => {
    const tokenCookie = useCookie('auth-token')
    if (tokenCookie.value) {
      config.headers.Authorization = `Bearer ${tokenCookie.value}`
    }
    return config
  })

  apiInstance.interceptors.response.use(
    (response) => response,
    (error) => {
      if (error.response?.status === 401) {
        const tokenCookie = useCookie('auth-token')
        tokenCookie.value = null
        navigateTo('/auth/login')
      }
      return Promise.reject(error)
    }
  )

  const apiCall = async <T = any>(endpoint: string, options?: AxiosRequestConfig): Promise<T> => {
    try {
      const response: AxiosResponse<T> = await apiInstance({
        url: endpoint,
        ...options
      })

      return response.data
    } catch (error) {
      console.error('API call failed:', error)
      throw error
    }
  }

  return {
    apiUrl: runtimeConfig.apiUrl as string,
    apiCall,
    apiInstance
  }
}