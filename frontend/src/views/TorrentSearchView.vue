<template>
  <div v-if="search_results">
    <TorrentSearchInputs
      ref="searchInputsRef"
      class="torrent-search-inputs"
      @search="search"
      :loading
      :initialForm="initialForm"
      :showStaffOptions="userStore.class === 'staff'"
    />
    <PaginatedResults
      v-if="search_results.length > 0"
      :totalPages
      :initialPage="initialForm.page"
      :totalItems="totalResults"
      :pageSize
      @changePage="searchInputsRef.changePage($event.page)"
    >
      <TitleGroupList :titleGroups="search_results" :titleGroupPreview />
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { searchTorrentsLite, type TorrentSearch } from '@/services/api/torrentService'
import { type TitleGroupHierarchyLite } from '@/services/api/artistService'
import TorrentSearchInputs from '@/components/torrent/TorrentSearchInputs.vue'
import TitleGroupList from '@/components/title_group/TitleGroupList.vue'
import type { titleGroupPreviewMode } from '@/components/title_group/TitleGroupList.vue'
import { useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'
import type { VNodeRef } from 'vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { computed } from 'vue'

const route = useRoute()
const userStore = useUserStore()

const searchInputsRef = ref<VNodeRef | null>(null)

const search_results = ref<TitleGroupHierarchyLite[]>([])
const titleGroupPreview = ref<titleGroupPreviewMode>('table') // TODO: make a select button to switch from cover-only to table
const loading = ref(false)
const initialForm = ref<TorrentSearch>({
  title_group_name: '',
  title_group_include_empty_groups: false,
  torrent_created_by_id: null,
  torrent_snatched_by_id: null,
  torrent_staff_checked: false,
  torrent_reported: null,
  page: 1,
  page_size: 10,
  order_by_column: 'torrent_created_at',
  order_by_direction: 'desc',
})
const totalResults = ref(0)
const pageSize = ref(initialForm.value.page_size)
const totalPages = computed(() => Math.ceil(totalResults.value / pageSize.value))

const search = async (torrentSearch: TorrentSearch) => {
  loading.value = true
  const results = await searchTorrentsLite(torrentSearch).finally(() => {
    loading.value = false
  })
  pageSize.value = torrentSearch.page_size
  totalResults.value = results.total_items
  search_results.value = results.results
}

const loadInitialForm = async () => {
  initialForm.value.title_group_name = route.query.title_group_name?.toString() ?? ''
  initialForm.value.torrent_created_by_id = route.query.created_by_id ? parseInt(route.query.created_by_id as string) : null
  initialForm.value.torrent_snatched_by_id = route.query.snatched_by_id ? parseInt(route.query.snatched_by_id as string) : null
  initialForm.value.page = route.query.page ? parseInt(route.query.page as string) : 1
  if (userStore.class === 'staff') {
    initialForm.value.torrent_staff_checked = false
    initialForm.value.torrent_reported = null
  }
  search(initialForm.value)
}

onMounted(async () => {
  loadInitialForm()
})
</script>

<style scoped>
.torrent-search-inputs {
  margin-bottom: 25px;
}
</style>
