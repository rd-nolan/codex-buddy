<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

type Theme = {
  name: string
  path: string
}

const themes = ref<Theme[]>([])
const selectedTheme = ref('default')
const status = ref('Disconnected')

async function launch() {
  await invoke('launch_codex')
  status.value = 'Codex started'
}

async function loadThemes() {
  try {
    themes.value = await invoke<Theme[]>('list_themes')
  } catch {
    themes.value = [
      {
        name: 'default',
        path: 'themes/default'
      }
    ]
  }
}

async function applyTheme() {
  await invoke('apply_theme', {
    theme: selectedTheme.value
  })
  status.value = `Applied ${selectedTheme.value}`
}

onMounted(loadThemes)
</script>

<template>
  <main class="container">
    <h1>Codex Buddy</h1>
    <p>Your Codex companion</p>

    <div class="status">
      Status: {{ status }}
    </div>

    <button @click="launch">启动 Codex</button>

    <section class="themes">
      <h3>Themes</h3>
      <label v-for="item in themes" :key="item.name">
        <input
          v-model="selectedTheme"
          type="radio"
          :value="item.name"
        />
        {{ item.name }}
      </label>
    </section>

    <button @click="applyTheme">Apply Theme</button>
  </main>
</template>

<style scoped>
.container {
  padding: 32px;
  font-family: system-ui;
}

button {
  margin: 8px 12px 8px 0;
  padding: 10px 18px;
}

.themes {
  margin: 24px 0;
}

label {
  display: block;
  margin: 8px 0;
}
</style>
