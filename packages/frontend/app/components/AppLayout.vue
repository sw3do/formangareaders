<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-950 overflow-x-hidden">
    <Navbar @toggle-sidebar="toggleSidebar" />
    
    <div class="flex relative">
      <ClientOnly>
        <Sidebar
          :is-open="sidebarOpen"
          @close="closeSidebar"
        />
      </ClientOnly>
      
      <main class="flex-1 transition-all duration-300 min-w-0">
        <div class="p-4 sm:p-6">
          <slot />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
const sidebarOpen = ref(false)

const toggleSidebar = () => {
  sidebarOpen.value = !sidebarOpen.value
}

const closeSidebar = () => {
  sidebarOpen.value = false
}



onMounted(() => {
  const handleEscape = (e: KeyboardEvent) => {
    if (e.key === 'Escape' && sidebarOpen.value) {
      closeSidebar()
    }
  }
  
  document.addEventListener('keydown', handleEscape)
  
  onUnmounted(() => {
    document.removeEventListener('keydown', handleEscape)
  })
})
</script>