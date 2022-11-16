<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>
  <p>{{ greetMsg }}</p>

  <div>
    <button @click="openFile()">Open File</button>
    <h3>{{ filePath }}</h3>
    <button @click="saveFile()">Save</button>
  </div>

  <button @click.prevent="openWindow">Open Second Window</button>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow } from "@tauri-apps/api/window";
import { open, save } from "@tauri-apps/api/dialog";

function openWindow() {
  // https://tauri.app/v1/guides/features/multiwindow#create-a-window-in-javascript
  const webview = new WebviewWindow("Second-Window", {
    height: 800,
    width: 1000,
    title: "Second Window",
    url: "/second-page",
  });

  // since the webview window is created asynchronously,
  // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
  webview.once("tauri://created", function () {
    // webview window successfully created
  });
  webview.once("tauri://error", function (e) {
    // an error happened creating the webview window
  });
}

const greetMsg = ref("");
const name = ref("");
const filePath = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function openFile() {
  const selected = await open();
  if (Array.isArray(selected)) {
    // user selected multiple files
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    filePath.value = selected;
    // user selected a single file
  }
}

async function saveFile() {
  const savePath = await save();

  // Loading the file to Javascript is unnecessary, since we can read and write directly inside Rust, this is a demo only.
  const file_bytes = await invoke("read_file", { path: filePath.value });
  try {
    await invoke("save_file", { content: file_bytes, fileName: savePath });
    alert("File saved successfully");
  } catch (err) {
    console.log(err);
  }
}
</script>
