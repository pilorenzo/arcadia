<template>
  <div class="bbcode-editor">
    <div class="boilerplate-inserters" v-if="!preview">
      <Button size="small" @click="insertBoilerplate('[b]', '[/b]')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="-1.5 0 24 24">
            <path
              d="M21.2 7c0-3.8-3.2-7-7-7h-12a2.2 2.2 0 0 0 0 4.4h1v15.2h-1a2.2 2.2 0 0 0 0 4.4h12a7 7 0 0 0 5-12 7 7 0 0 0 2-5zm-7 2.8H7.4V4.4H14a2.7 2.7 0 0 1 0 5.4zm0 9.8H7.4v-5.4H14a2.7 2.7 0 0 1 0 5.4z"
            />
          </svg>
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[i]', '[/i]')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <path stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 3h10M4 21h10m1-18L9 21" />
          </svg>
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[s]', '[/s]')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <g fill="none" fill-rule="nonzero">
              <path d="M24 0v24H0V0h24ZM12.6 23.3h-.2v.5h.2v-.5Zm.3-.2-.2.1v.5l.2.1v-.6Zm-.8 0v.7h.2v-.5l-.2-.2Z" />
              <path
                fill="#09244B"
                d="M19 12a1 1 0 0 1 .1 2H17a4.5 4.5 0 0 1-3.6 7h-1.9c-2 0-4-1.1-5-3l-.1-.2-.2-.3a1 1 0 0 1 0-.6 1 1 0 0 1 1.7-.5v.1l.3.4c.6 1.2 1.8 2 3.1 2l.2.1h1.7a2.5 2.5 0 0 0 1.3-4.7h-.1l-.5-.3H5a1 1 0 0 1-.1-2H19Zm-6.4-9c2.1 0 4.1 1.2 5.1 3.2l.2.3a1 1 0 0 1 0 1 1 1 0 0 1-.7.5 1 1 0 0 1-1-.5l-.3-.4C15.3 5.8 14 5 12.6 5h-1.7a2.5 2.5 0 0 0-1.2 4.8l2.5 1.2H8A4.5 4.5 0 0 1 11 3h1.7Z"
              />
            </g>
          </svg>
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[u]', '[/u]')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <path stroke="#323232" stroke-linecap="round" stroke-width="2" d="M7 4v7a5 5 0 0 0 10 0V4" />
            <path stroke="#323232" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 20h14" />
          </svg>
        </template>
      </Button>
    </div>
    <FloatLabel style="width: 100%" variant="in" v-if="!preview">
      <Textarea ref="textareaRef" v-model="content" rows="5" style="width: 100%" autoResize @value-change="emit('valueChange', content)" name="content" />
      <label for="in_label">{{ label }}</label>
    </FloatLabel>
    <div class="message">
      <slot name="message"></slot>
    </div>
    <ContentContainer v-if="preview">
      <BBCodeRenderer :content />
    </ContentContainer>
    <div class="actions">
      <Button :label="t('general.preview')" :icon="`pi pi-eye${preview ? '-slash' : ''}`" @click="preview = !preview" />
      <slot name="buttons"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { FloatLabel, Textarea } from 'primevue'
import ContentContainer from '../ContentContainer.vue'
import { watch } from 'vue'
import Button from 'primevue/button'
import { useI18n } from 'vue-i18n'
import BBCodeRenderer from '../community/BBCodeRenderer.vue'
import { onMounted } from 'vue'
import { nextTick } from 'vue'

const props = defineProps<{
  label: string
  emptyInput?: boolean
  initialValue?: string
}>()

const emit = defineEmits<{
  inputEmptied: [boolean]
  valueChange: [string]
}>()

const { t } = useI18n()

const content = ref('')
const preview = ref(false)

const textareaRef = ref<InstanceType<typeof Textarea> | null>(null)
const insertBoilerplate = (openTag: string, closeTag: string) => {
  // @ts-expect-error TODO: properly type the textarea ref
  const textareaEl = textareaRef.value?.$el as HTMLTextAreaElement
  if (!textareaEl) return

  const start = textareaEl.selectionStart
  const end = textareaEl.selectionEnd
  const currentVal = content.value
  const selectedText = currentVal.substring(start, end)

  const replacement = openTag + selectedText + closeTag

  // Update the content
  content.value = currentVal.substring(0, start) + replacement + currentVal.substring(end)

  emit('valueChange', content.value)

  // nextTick ensures the DOM has updated *before* we try to set focus/selection
  nextTick(() => {
    // Restore focus
    textareaEl.focus()

    const newCursorPos = selectedText ? start + replacement.length : start + openTag.length

    // Set the new cursor position
    textareaEl.setSelectionRange(newCursorPos, newCursorPos)
  })
}

onMounted(() => {
  content.value = props.initialValue ?? content.value
})

watch(
  () => props.emptyInput,
  (newVal) => {
    if (newVal) {
      content.value = ''
      emit('inputEmptied', true)
    }
  },
)
</script>
<style scoped>
.bbcode-editor {
  width: 100%;
}
.boilerplate-inserters {
  width: 100%;
  margin-bottom: 5px;
  .p-button {
    margin-right: 4px;
    padding: 0.45em;
  }
}
.message {
  height: 0;
}
.actions {
  text-align: right;
  margin-top: 5px;
}
</style>

<style>
.bbcode-editor {
  .actions {
    .p-button {
      margin-left: 10px;
    }
  }
}
</style>
