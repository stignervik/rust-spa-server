import { defineStore } from "pinia"
import { computed, ref } from "vue"
import { Unit } from "../model/unit"

export const useUnitStore = defineStore('unit', () => {
  const count = ref(0)
  const name = ref('Test')
  const doubleCount = computed(() => count.value * 2)
  function increment() {
    count.value++
  }

  let units = ref(new Map<number, Unit>());

  function createUnit() {
    console.log('units length: ', units.value.values.length);
    let unit = {id: units.value.size + 1, name: "unit1", unitClass: "Unit", unitFunc: "Unit"};
    console.log(`unit id:  ${unit.id}`);
    units.value.set(unit.id, unit);
    console.log('units length: ', units.value.size);
  }

  function size() : number {
    let count = units.value.size;
    return count;
  }

  return { count, name, doubleCount, increment, createUnit, size }
})

