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