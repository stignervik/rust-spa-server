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
    <v-btn
      id="btnTestConnection"
      type="button"
      class="bg-color"
      color="info"
      @click="test"
    >
      Test
    </v-btn>
  </v-container>
</template>

<script setup lang="ts">
  import { ref, reactive} from 'vue';
  import { http } from '../service/rest'

  async function fetchUnits() {

    console.log("fetch units ...");

    let resUnits = await http.get('/units');
    console.log(`List units: ${resUnits.data}`)


    /*
    let resUnits = await http.get('/units');
    console.log(`List units: ${resUnits.data}`)

    resUnits = await http.get('/create_unit');
    console.log(`Add unit: ${resUnits.data}`)

    resUnits = await http.get('/units');
    console.log(`List units: ${resUnits.data}`)

    let count = await http.get('/units_len');
    console.log(`Units count: ${count.data}`);
    */
    /*
    let resBooks = await http.get('/booklist');
    console.log(`List books: ${resBooks.data}`);

    resBooks = await http.get('/books');
    console.log(`Add a book: ${resBooks.data}`);

    resBooks = await http.get('/booklist');
    console.log(`List books: ${resBooks.data}`);
    */

    /*
    console.log(`base url: ${http.defaults.baseURL}`)
    const response = await http.get('/get');
    console.log(`Response: ${response.data}`)

    let resUnits = await http.get('/units');
    console.log(`List units: ${resUnits.data}`)

    resUnits = await http.get('/create_unit');
    console.log(`Add unit: ${resUnits.data}`)

    resUnits = await http.get('/units');
    console.log(`List units: ${resUnits.data}`)

    let resBooks = await http.get('/books');
    console.log(`Response: ${resBooks.data}`);

    resBooks = await http.get('/booklist');
    console.log(`Response: ${resBooks.data}`);
    */
  }

  async function test() {

console.log("fetch units ...");

let resUnits = await http.get('/units');
console.log(`List units: ${resUnits.data}`)

resUnits = await http.get('/create_unit');
console.log(`Add unit: ${resUnits.data}`)

resUnits = await http.get('/units');
console.log(`List units: ${resUnits.data}`)

let count = await http.get('/units_len');
console.log(`Units count: ${count.data}`);
  }
</script>