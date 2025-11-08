<template>
  <DataTable :value="searchResults" size="small">
    <Column :header="`${t('forum.category')} > ${t('forum.subcategory')}`" style="width: 17%">
      <template #body="slotProps">
        <RouterLink to="/forum">
          {{ slotProps.data.category_name }}
        </RouterLink>
        &gt;
        <RouterLink :to="`/forum/sub-category/${slotProps.data.sub_category_id}`">
          {{ slotProps.data.sub_category_name }}
        </RouterLink>
      </template>
    </Column>
    <Column :header="t('forum.thread')">
      <template #body="slotProps">
        <div style="display: flex; justify-content: space-between; align-items: center">
          <div class="left">
            <div class="top">
              <RouterLink :to="`/forum/thread/${slotProps.data.thread_id}?post_id=${slotProps.data.post_id}#post-${slotProps.data.post_id}`">
                {{ slotProps.data.thread_name }}
              </RouterLink>
            </div>
            <div class="bottom" style="font-size: 0.8em; text-overflow: ellipsis; white-space: nowrap; overflow: hidden; width: 40vw">
              {{ slotProps.data.post }}
            </div>
          </div>
          <div class="right">
            {{ t('general.by') }}
            <RouterLink :to="`/user/${slotProps.data.post_created_by_id}`">
              {{ slotProps.data.post_created_by_username }}
            </RouterLink>
          </div>
        </div>
      </template>
    </Column>
    <Column :header="t('general.time')" style="width: 10em">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.post_created_at) }}
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import type { ForumSearchResult } from '@/services/api/forumService'
import { timeAgo } from '@/services/helpers'
import { Column, DataTable } from 'primevue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  searchResults: ForumSearchResult[]
}>()
</script>

<style scoped></style>
