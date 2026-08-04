<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

async function launch() {
  await invoke('launch_codex')
}

async function theme() {
  await invoke('apply_default_theme')
}
</script>

<template>
  <main class="container">
    <h1>Codex Buddy</h1>
    <p>Your Codex companion</p>
    <button @click="launch">启动 Codex</button>
    <button @click="theme">应用默认主题</button>
  </main>
</template>

<style scoped>
.container {
  padding: 32px;
  font-family: system-ui;
}
button {
  margin-right: 12px;
  padding: 10px 18px;
}
</style>
