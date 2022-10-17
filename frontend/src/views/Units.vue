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
      @click="createUnit"
    >
      Create Unit
    </v-btn>
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
    <v-btn
      id="btnTestConnection"
      type="button"
      class="bg-color"
      color="info"
      @click="cyclic_fetch"
    >
      Cyclic fetch
    </v-btn>
  </v-container>
</template>

<script setup lang="ts">
  import { ref, reactive} from 'vue';
  import { http } from '../service/rest'


  async function createUnit() {
    http.post('/create_unit', {
      name: 'Unit1',
      class: "Unit",
      func: "Unit",
    })
    .then(function (response) {
      console.log(response);
    })
    .catch(function (error) {
      console.log('error...')
      console.log(error);
    });
  }


  async function fetchUnits() {

    if (typeof(Worker) !== "undefined") {
      // Yes! Web worker support!
      console.log("web worker support!")
      // Some code.....
    } else {
      // Sorry! No Web Worker support..
      console.log("No web worker support!")
    }

    let res = await http.get('/units');
    console.log(`List units: ${res.data}`);

    res = await http.get('/units_len');
    console.log(`Unit length: ${res.data}`);

    // let resMessage = await http.get('/hello/Stig');
    // console.log(`List units: ${resMessage.data.message}`)



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

  async function cyclic_fetch() {
    while (true) {
      let count = await http.get('/hello/:stig');
      setTimeout(() => {console.log(`Units count: ${count.data}`);}, 500);
    }
  }
</script>