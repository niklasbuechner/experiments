<template>
  <div id="app">
    <h1>Expanding Boxes</h1>
    <expanding-box></expanding-box>
    <expanding-box></expanding-box>
    <expanding-box></expanding-box>
    <expanding-box></expanding-box>
    <expanding-box></expanding-box>
  </div>
</template>

<script>
import expandingBox from './expanding-box.vue'

export default {
  name: 'app',
  components: {
    expandingBox
  },
  data () {return {};}
}
</script>

<style>
#app {
  width: 1024px;
  margin: auto;
}
</style>
