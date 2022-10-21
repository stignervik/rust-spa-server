<template>
  <v-container fluid>
    <v-col class="pa-0">
      <v-col>
        <v-card class="pa-1 ma-auto" :elevation="6" color="#E3F2FD">
          <v-row class="pa-2 mb-0">
            <v-text-field id="customer-id"
              variant="outlined"
              bg-color="white"
              v-model="state.search"
              label="Search"
              prepend-inner-icon="mdi-magnify"
              clearable
              aria-autocomplete="false"
              class="pa-2 mt-2"
              @keyup="filterUnits"
            ></v-text-field>
            <h1 class="pa-2 mt-2">{{unitStore.size()}}</h1>
            <v-btn @click="addUnit" class="ma-4" color="success" icon="mdi-plus"></v-btn>
          </v-row>
        </v-card>
      </v-col>
      <v-spacer></v-spacer>
      <v-col>
        <v-card class="pa-1 md-2" :elevation="6">
          <v-list>
            <v-list-item v-for="(item, i) in unitStore.units" :key="i">
              <v-card class="pa-1 md-2" :elevation="6" color="#E3F2FD">
                <v-row class="pt-0">
                  <v-text-field id="customer-id"
                    variant="outlined"
                    bg-color="white"
                    v-model="item.at(1).id"
                    label="Id"
                    prepend-inner-icon="mdi-account"
                    clearable
                    required
                    aria-autocomplete="false"
                    class="pt-6 pl-6 pr-6"
                  ></v-text-field>
                  <v-text-field id="order-id"
                    variant="outlined"
                    bg-color="white"
                    v-model="item.at(1).name"
                    label="Name"
                    prepend-inner-icon="mdi-order-bool-descending-variant"
                    clearable
                    required
                    aria-autocomplete="false"
                    class="pt-6 pl-6 pr-6"
                  ></v-text-field>
                </v-row>
                <v-row class="pt-0">
                  <v-text-field id="customer-id"
                    variant="outlined"
                    bg-color="white"
                    v-model="item.at(1).unit_class"
                    label="Class"
                    prepend-inner-icon="mdi-order-bool-descending-variant"
                    clearable
                    required
                    aria-autocomplete="false"
                    class="pt-2 pl-6 pr-6"
                  ></v-text-field>
                  <v-text-field id="order-id"
                    variant="outlined"
                    bg-color="white"
                    v-model="item.at(1).unit_func"
                    label="Function"
                    prepend-inner-icon="mdi-order-bool-descending-variant"
                    clearable
                    required
                    aria-autocomplete="false"
                    class="pt-2 pl-6 pr-6"
                  ></v-text-field>
                </v-row>
                <v-card-actions class="pa-0">
                  <v-spacer></v-spacer>
                  <v-btn @click="deleteUnit(item.at(1).id)" icon="mdi-trash-can-outline" color="red"></v-btn>
                </v-card-actions>
              </v-card>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>
    </v-col>
  </v-container>
</template>

<script setup lang="ts">
  import { ref, reactive, onMounted } from 'vue';
  import { http } from '../service/rest'
  import { useCounterStore } from '../stores/counter'
  import { useUnitStore } from '../stores/unitstore'
  import { Unit } from '../model/unit'

  const store = useCounterStore();
  const unitStore = useUnitStore();

  onMounted(() => {
    // Fill in some test data
    /*
    state.units.push({id: 1, name: 'Unit 1', unit_class: 'Unit', unit_func: 'Unit'});
    state.units.push({id: 2, name: 'Unit 2', unit_class: 'Unit', unit_func: 'Unit'});
    state.units.push({id: 3, name: 'Unit 3', unit_class: 'Unit', unit_func: 'Unit'});
    state.units.push({id: 4, name: 'Unit 4', unit_class: 'Unit', unit_func: 'Unit'});
    state.units.push({id: 5, name: 'Unit 5', unit_class: 'Unit', unit_func: 'Unit'});
    */

    // getUnits();
  })

  /*
  interface StockKeepingUnit {
    type: string,
    number: number,
  }

  interface Features {
    demo: boolean,
    selfVerify: boolean,
    coverDetection: boolean,
    analogValues: boolean,
    opticalIntegrity: boolean,
    requireSignedConfigurations: boolean,
    allowDowngrades: boolean,
  }

  interface SalesOrder {
    customerId: string,
    orderId: string,
    features: Features,
    snumbers: Array<string>,
    skus: Array<StockKeepingUnit>
  }
  */




  const num = ref(0);

  const state = reactive({
    customerId: "",
    orderId: "",
    snumbers: [] as string[],
    search: "",
    units: [] as Unit[],
    selectedItem: 1,
    allUnits: [] as Unit[],

    // backend
    systemLicense: [],
  });

  async function addUnit() {
    console.log(`Create user: `);
    http.post('/create_unit', {
      id: "0", // dummy id, will be set by the server, optional could be an option, but not for the id
      name: `Unit ${unitStore.size()}`,
      unit_class: 'Unit',
      unit_func: 'Unit'
    })
    .then(function (response) {
      console.log(response.data);
      unitStore.push(response.data);
    })
    .catch(function (error) {
      console.log('error...')
      console.log(error);
    });

    console.log(`counter: ${unitStore.size()}`)
  }

  function deleteUnit(id: string) {
    http.post('/delete_unit', {
      id: id,
      name: `Unit`,
      unit_class: 'Unit',
      unit_func: 'Unit'
    })
    .then(function (response) {
      console.log(response.data);
      unitStore.deleteUnit(response.data.id);
    })
    .catch(function (error) {
      console.log('error...')
      console.log(error);
    });
  }

  function filterUnits() {
    if (state.allUnits.length == 0) {
      state.allUnits = [...state.units];
    }

    state.units = state.units.filter((obj) => {
      return obj.id === state.search
    });

    if (state.search == "") {
      state.units = [...state.allUnits];
    }
  }

  /*
  function addPanleSerialNumber(customerId: string, orderId: string) {
    state.salesOrders.forEach((item, index) => {
      if ((item.customerId === customerId) && (item.orderId === orderId)) {
        item.snumbers.push("");
      }
    });
  }

  function deletePanelSerialNumber(customerId: string, orderId: string, serialNumber: string) {
    state.salesOrders.forEach((item, index) => {
      if ((item.customerId === customerId) && (item.orderId === orderId)) {
        item.snumbers.forEach((sn, i) => {
          if (sn === serialNumber) {
            item.snumbers.splice(i, 1);
          }
        });
      }
    });
  }

  function addStockKeepingUnit(customerId: string, orderId: string) {
    state.salesOrders.forEach((item, index) => {
      if ((item.customerId === customerId) && (item.orderId === orderId)) {
        item.skus.push({type: "", number: 0});
      }
    });
  }

  function deleteStockKeepingUnit(customerId: string, orderId: string, stockKeepingUnit: string) {
    state.salesOrders.forEach((item, index) => {
      if ((item.customerId === customerId) && (item.orderId === orderId)) {
        item.skus.forEach((sku, i) => {
          if (sku.type === stockKeepingUnit) {
            item.skus.splice(i, 1);
          }
        });
      }
    });
  }

  // function calls to backend.
  async function getSystems() {
    try {
      console.log("base url: ", http.defaults.baseURL);
      const response = await http.get('/list/system');
      console.log(response.data.systemLicense)
      state.systemLicense = response.data.systemLicense
      num.value = state.systemLicense.length
      state.systemLicense.forEach((item) => {
        console.log("Item: ", item.customerId);
        // build
        state.salesOrders.push({customerId: `${item.customerId}`, orderId: `${item.orderId}`, features: {demo: false, selfVerify: false, coverDetection: false, analogValues: false, opticalIntegrity: false, requireSignedConfigurations: false, allowDowngrades: false}, snumbers: [`${item.serialNumbers}`], skus: []});
      })
    } catch (error) {
      console.error(error);
    }
  }
  */

</script>

<style lang="scss" scoped>
  .container-style {
    display: flex;
    flex: 1;
    background-color: grey;
    align-items: stretch;
  }

</style>



<!-- <template>
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
</script> -->