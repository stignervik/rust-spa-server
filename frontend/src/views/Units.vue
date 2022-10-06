<template>
  <v-container>
    <v-col class="blue-grey--text mt-4" align="center">
      <h3 class="ma-4">Units</h3>
      <h4> An rust server, serve embed Vue application.</h4>
    </v-col>
    <v-btn
      id="btnTestConnection"
      type="button"
      class="bg-color"
      color="primary"
      @click="fetchUnits"
    >
      Fetch
    </v-btn>
  </v-container>
</template>

<script setup lang="ts">
  import { ref, reactive} from 'vue';
  import { http } from '../service/rest'

  async function fetchUnits() {
    console.log(`base url: ${http.defaults.baseURL}`)
    const response = await http.get('/get');
    console.log(`Response: ${response.data}`)

    const resUnits = await http.get('/units');
    console.log(`Response: ${resUnits.data}`)

    const resBooks = await http.get('/books');
    console.log(`Response: ${resBooks.data}`)
  }
</script>