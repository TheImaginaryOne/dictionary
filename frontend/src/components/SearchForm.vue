<template>
  <div class="search-form">
    <input v-model="query"/>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  data: function () {
    return {
      query: ''
    }
  },
  watch: {
    // when query is updated
    query: function () {
      this.updateQuery()
    }
  },
  methods: {
    updateQuery: function () {
      // emit event
      this.$emit('update:query', this.query)
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss">
</style>
