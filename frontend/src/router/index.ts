import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Units from "../../src/views/Units.vue";
/*
import License from "../../src/views/License.vue";
import CreateNewAccount from "../../src/views/CreateNewAccount.vue";
import RetrievePassword from "../../src/views/RetrievePassword.vue";
import PasswordChange from "../../src/views/PasswordChange.vue";
*/

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "units",
    component: Units,
  },
  /*
  {
    path: "/license",
    name: "license",
    component: License,
  },
  {
    path: "/createnewaccount",
    name: "createnewaccount",
    component: CreateNewAccount,
  },
  {
    path: "/retrievePassword",
    name: "retrievePassword",
    component: RetrievePassword,
  },
  {
    path: "/passwordChange",
    name: "passwordChange",
    component: PasswordChange,
  },
  */
  {
    path: "/about",
    name: "about",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () =>
      import(/* webpackChunkName: "about" */ "../views/About.vue"),
    // beforeEnter: authGuard,
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
