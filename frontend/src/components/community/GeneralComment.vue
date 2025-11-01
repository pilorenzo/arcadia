<template>
  <ContentContainer class="comment-container">
    <div class="comment">
      <div class="user">
        <img class="avatar" :src="comment.created_by.avatar ?? '/default_user_avatar.jpg'" :alt="comment.created_by.username + '\'s avatar'" />
        <RouterLink :to="`/user/${comment.created_by.id}`">
          <span class="username">
            {{ comment.created_by.username }}

            <i v-if="comment.created_by.banned" v-tooltip.top="t('user.banned')" class="banned pi pi-ban" />
            <i v-if="!comment.created_by.banned && comment.created_by.warned" v-tooltip.top="t('user.warned')" class="warned pi pi-exclamation-triangle" />
          </span>
        </RouterLink>
        <span class="time-ago">
          {{ timeAgo(comment.created_at) }}
        </span>
      </div>
      <div class="comment-body">
        <BBCodeRenderer :content="comment.content" />
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import type { TitleGroupCommentHierarchy } from '@/services/api/commentService'
import { timeAgo } from '@/services/helpers'
import type { ForumPostHierarchy } from '@/services/api/forumService'
import { useI18n } from 'vue-i18n'
import type { ConversationMessageHierarchy } from '@/services/api/conversationService'

const { t } = useI18n()

defineProps<{
  comment: TitleGroupCommentHierarchy | ForumPostHierarchy | ConversationMessageHierarchy
}>()
</script>

<style scoped>
.comment-container {
  margin-top: 10px;
}
.comment {
  display: flex;
  align-items: flex-start;
}
.user {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 10px;
  background-color: var(--color-background-primary);
  border-radius: 7px;
}
.avatar {
  width: 7em;
  border-radius: 7px;
}
.comment-body {
  padding: 7px;
}
.username {
  color: var(--color-primary);
  i {
    &.banned {
      color: red;
    }
    &.warned {
      color: yellow;
    }
  }
}
.time-ago {
  font-size: 0.8em;
  margin-top: 5px;
}
</style>
