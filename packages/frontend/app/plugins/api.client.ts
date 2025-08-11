export default defineNuxtPlugin(() => {
  const { apiCall } = useApi()
  
  return {
    provide: {
      api: apiCall
    }
  }
})