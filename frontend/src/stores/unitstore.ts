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

  let units = ref(new Map<string, Unit>());

  function push(unit: Unit) {
    units.value.set(unit.id, unit);
  }

  function deleteUnit(id: string) {
    units.value.delete(id);
  }

  function size() : number {
    let count = units.value.size;
    return count;
  }

  return { count, name, doubleCount, increment, units, push, size, deleteUnit}
})

