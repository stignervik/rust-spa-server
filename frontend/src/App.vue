<template>
  <v-app>
    <v-app-bar app color="info" dark>
      <div class="d-flex align-center">
    <!--<v-img
      :width="70"
      :src="iconPath"
      cover
      class="ml-4"
    ></v-img> -->
        <h3 class="ml-4">Rust SPA Server</h3>
      </div>
      <v-spacer></v-spacer>
      <v-btn :to="{ path: '/' }">
        <span class="mr-1">Units</span>
        <v-icon>mdi-ballot-outline</v-icon>
      </v-btn>
      <v-btn :to="{ path: '/about' }">
        <span class="mr-1">About</span>
        <v-icon>mdi-information-outline</v-icon>
      </v-btn>
      <v-btn :to="{ path: '/user' }">
        <span class="mr-1">User</span>
        <v-icon>mdi-account-outline</v-icon>
      </v-btn>
    </v-app-bar>
    <v-main class="main-view">
      <v-container fluid>
        <router-view/>
      </v-container>
    </v-main>
    <v-footer app color="info">
      <h4 class="ml-2">Rust Embed</h4>
      <v-spacer></v-spacer>
      <v-btn
        href="https://www.rust-lang.org/"
        target="_blank"
        variant="text"
      >
        <span class="mr-2">Rust</span>
        <v-icon>mdi-open-in-new</v-icon>
      </v-btn>
  </v-footer>
  </v-app>
</template>

<script setup lang="ts">
  import { defineComponent, onMounted, ref } from 'vue'
  import Worker from './workers/worker?worker'

  const worker = new Worker()
  worker.postMessage('')
  // import { useAuthStore } from '../src/stores/auth'
  // import appIcon from './assets/autronica_logo.png'

    // const store = useAuthStore()
    /*
    function logout() {
      console.log("Logout...")
      store.loginState = false
      location.reload()
    }
    */
  let counter = ref(0)

  onMounted(() => {
    worker.onmessage = (e) => {
        counter = e.data
        // if (counter % 10 === 0) {
          // console.log("counter: ", counter)
        // }
      }
  })

  /*
  export default defineComponent({
    name: 'App',

    components: {
    },

    data () {
      return {
        // iconPath: appIcon
      }
    },
  })
  */
</script>


<style lang="scss" scoped>
  .main-view {
    background-color: #e3f2fd;
    display: flex;
    flex: 1;
  }

  .snackbar {
    margin-top: 50px;
  }
  </style>
