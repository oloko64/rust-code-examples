<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>
  <p>{{ greetMsg }}</p>

  <div>
    <input style="margin-right: 20px;" type="file" ref="doc" @change="readFile" />
    <input v-model="fileName" type="text" placeholder="Name your file here" />
  </div>

  <button @click.prevent="saveFile">Save File</button>
  <button @click.prevent="openWindow">Open Second Window</button>
</template>

<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow } from "@tauri-apps/api/window";

interface FileData {
  files: Blob[];
}

function openWindow() {
  const webview = new WebviewWindow("Second-Window", {
    height: 800,
    width: 1000,
    url: "/second-page",
  });
}

const greetMsg = ref("");
const name = ref("");
const fileContents = ref("");
const doc: Ref<FileData> = ref({ files: [] });
const fileName = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

function readFile() {
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
  if (!fileName.value) {
    alert("Please enter a file name");
    return;
  }
  const res: boolean = await invoke("save_file", {
    content: fileContents.value,
    fileName: fileName.value,
  });
  if (res) {
    alert("File saved successfully");
  } else {
    alert("Error saving file");
  }
}
</script>
