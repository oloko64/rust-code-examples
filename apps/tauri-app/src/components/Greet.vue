<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface FileData {
  files: Blob[];
}

const greetMsg = ref("");
const name = ref("");
const fileContents = ref("");
const doc: Ref<FileData> = ref({ files: [] });

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

function readFile(data: any) {
  const file = doc?.value?.files[0];
  const reader = new FileReader();
  reader.onload = (res) => {
    if (!res?.target?.result) {
      alert("Error reading file");
      return;
    }
    const resultString = res.target.result as string;
    const start = resultString.split(",", 1) + ",";
    fileContents.value = resultString.replace(start, "");
  };
  reader.readAsDataURL(file);
}

async function saveFile() {
  const res: boolean = await invoke("save_file", {
    content: fileContents.value,
    fileName: "teste_vue-tauri.jpeg",
  });
  if (res) {
    alert("File saved successfully");
  } else {
    alert("Error saving file");
  }
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <input type="file" ref="doc" @change="readFile" />

  <p>{{ greetMsg }}</p>

  <button @click="saveFile">Save File</button>
</template>
